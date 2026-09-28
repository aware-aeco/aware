//! Shared helpers for the crate's synchronous HTTP client (`ureq`).
//!
//! Two unrelated concerns, both of which every caller would otherwise retype:
//! the deadline machinery that keeps a stalled DNS lookup or a dribbling peer
//! from outliving a caller's wall clock, and the plain "GET this URL into
//! memory" fetch the generators use.

use std::io::Read;
use std::net::{SocketAddr, ToSocketAddrs};
use std::time::{Duration, Instant};

use crate::error::AwareError;

/// GET `url` and return the whole response body as bytes.
///
/// `builder::npm`, `builder::ruby` and `builder::nuget` each carried a
/// byte-identical copy of this — `ureq::get`, `.call()`, map to
/// [`AwareError::Network`], `read_to_end`, map again — and `builder::yard` and
/// `builder::openapi` carried the `String` flavour below. Six copies of one
/// idea, differing only in incidentals: three said a bare `read:`, which told a
/// user whose build failed nothing at all about which fetch had failed, so the
/// wording that names the URL is the one kept.
///
/// The sixth, `commands::sidecar`'s `http_get_bytes`, also pre-sized the buffer
/// from the response's `content-length`, falling back to 8 MB. That is dropped
/// rather than generalized: the header is supplied by the peer and was used
/// uncapped, so a server answering `content-length: 40000000000` made the client
/// reserve 40 GB before reading a byte of a body it never had to send. A growing
/// `Vec` reallocates a handful of times on a real download and cannot be steered
/// from the wire, which is the better trade for every caller here.
///
/// **Unbounded and untimed, deliberately.** These fetches pull whole package
/// tarballs from a registry the user named on the command line, and every
/// implementation this replaces read to end with no cap and no deadline. A
/// caller reading from a peer it does not trust to be finite wants
/// [`read_with_deadline`] instead, which bounds both.
pub(crate) fn get_bytes(url: &str) -> Result<Vec<u8>, AwareError> {
    let mut bytes = Vec::new();
    response_reader(url)?
        .read_to_end(&mut bytes)
        .map_err(|error| AwareError::Network(format!("read {url}: {error}")))?;
    Ok(bytes)
}

/// GET `url` and return the whole response body as a `String`.
///
/// The text flavour of [`get_bytes`]; the same caveats apply. A body that is not
/// valid UTF-8 is a `Network` error, which is what the copies this replaces did
/// (`read_to_string` fails on invalid UTF-8 rather than substituting).
pub(crate) fn get_string(url: &str) -> Result<String, AwareError> {
    let mut body = String::new();
    response_reader(url)?
        .read_to_string(&mut body)
        .map_err(|error| AwareError::Network(format!("read {url}: {error}")))?;
    Ok(body)
}

/// The request half both fetchers share: a non-2xx status is an error from
/// `ureq::Request::call` itself, so there is no status check to forget here.
fn response_reader(url: &str) -> Result<Box<dyn Read + Send + Sync + 'static>, AwareError> {
    Ok(ureq::get(url)
        .call()
        .map_err(|error| AwareError::Network(format!("GET {url}: {error}")))?
        .into_reader())
}

/// A DNS resolver that puts a wall-clock bound around the standard library's
/// otherwise-unbounded synchronous lookup. The detached worker performs only
/// name resolution: if it outlives the caller it cannot transmit credentials
/// or a request body.
#[derive(Debug, Clone, Copy)]
pub(crate) struct BoundedDnsResolver {
    timeout: Duration,
}

impl BoundedDnsResolver {
    pub(crate) fn new(timeout: Duration) -> Self {
        Self { timeout }
    }
}

impl ureq::Resolver for BoundedDnsResolver {
    fn resolve(&self, netloc: &str) -> std::io::Result<Vec<SocketAddr>> {
        let netloc = netloc.to_owned();
        resolve_with_timeout(self.timeout, move || {
            netloc
                .to_socket_addrs()
                .map(|addresses| addresses.collect())
        })
    }
}

fn resolve_with_timeout(
    timeout: Duration,
    resolve: impl FnOnce() -> std::io::Result<Vec<SocketAddr>> + Send + 'static,
) -> std::io::Result<Vec<SocketAddr>> {
    let (tx, rx) = std::sync::mpsc::sync_channel(1);
    std::thread::Builder::new()
        .name("aware-bounded-dns".into())
        .spawn(move || {
            let _ = tx.send(resolve());
        })
        .map_err(|error| std::io::Error::other(format!("start DNS resolver: {error}")))?;

    match rx.recv_timeout(timeout) {
        Ok(result) => result,
        Err(std::sync::mpsc::RecvTimeoutError::Timeout) => Err(std::io::Error::new(
            std::io::ErrorKind::TimedOut,
            "DNS lookup deadline exceeded",
        )),
        Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => Err(std::io::Error::other(
            "DNS resolver stopped before returning a result",
        )),
    }
}

/// Read a detached HTTP response body without letting a slow-dribbling peer
/// defeat the caller's wall-clock deadline or an oversized body grow memory
/// without bound.
pub(crate) fn read_with_deadline(
    reader: Box<dyn Read + Send + Sync + 'static>,
    remaining: Duration,
    max_bytes: usize,
    thread_name: &str,
) -> Result<Vec<u8>, String> {
    let deadline = Instant::now()
        .checked_add(remaining)
        .ok_or_else(|| "response body deadline overflowed".to_string())?;
    let (tx, rx) = std::sync::mpsc::sync_channel(1);
    std::thread::Builder::new()
        .name(thread_name.into())
        .spawn(move || {
            let mut body = Vec::new();
            let result = DeadlineReader { reader, deadline }
                .take((max_bytes + 1) as u64)
                .read_to_end(&mut body)
                .map_err(|error| format!("read response: {error}"))
                .and_then(|_| {
                    if body.len() > max_bytes {
                        Err(format!("response exceeds the {max_bytes}-byte limit"))
                    } else {
                        Ok(body)
                    }
                });
            let _ = tx.send(result);
        })
        .map_err(|error| format!("start bounded response reader: {error}"))?;

    match rx.recv_timeout(deadline.saturating_duration_since(Instant::now())) {
        Ok(result) => result,
        Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
            Err("request deadline exceeded while reading response body".into())
        }
        Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => {
            Err("response reader stopped before returning a result".into())
        }
    }
}

struct DeadlineReader<R> {
    reader: R,
    deadline: Instant,
}

impl<R: Read> Read for DeadlineReader<R> {
    fn read(&mut self, buffer: &mut [u8]) -> std::io::Result<usize> {
        if Instant::now() >= self.deadline {
            return Err(std::io::Error::new(
                std::io::ErrorKind::TimedOut,
                "request deadline exceeded while reading response body",
            ));
        }
        let read = self.reader.read(buffer)?;
        if Instant::now() >= self.deadline {
            return Err(std::io::Error::new(
                std::io::ErrorKind::TimedOut,
                "request deadline exceeded while reading response body",
            ));
        }
        Ok(read)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dns_lookup_is_bounded_even_when_the_system_resolver_stalls() {
        let started = Instant::now();
        let error = resolve_with_timeout(Duration::from_millis(20), || {
            std::thread::sleep(Duration::from_millis(200));
            Ok(Vec::new())
        })
        .unwrap_err();
        assert_eq!(error.kind(), std::io::ErrorKind::TimedOut);
        assert!(
            started.elapsed() < Duration::from_millis(150),
            "the caller waited for the unbounded resolver"
        );
    }

    #[test]
    fn dns_lookup_returns_the_resolvers_result() {
        let expected = "127.0.0.1:443".parse().unwrap();
        assert_eq!(
            resolve_with_timeout(Duration::from_secs(1), move || Ok(vec![expected])).unwrap(),
            vec![expected]
        );
    }

    struct SlowDribble {
        remaining: usize,
    }

    impl Read for SlowDribble {
        fn read(&mut self, buffer: &mut [u8]) -> std::io::Result<usize> {
            if self.remaining == 0 || buffer.is_empty() {
                return Ok(0);
            }
            std::thread::sleep(Duration::from_millis(10));
            buffer[0] = b'x';
            self.remaining -= 1;
            Ok(1)
        }
    }

    #[test]
    fn deadline_stops_a_slow_dribble() {
        let started = Instant::now();
        let error = read_with_deadline(
            Box::new(SlowDribble { remaining: 20 }),
            Duration::from_millis(35),
            1024,
            "aware-test-response-reader",
        )
        .unwrap_err();
        assert!(error.contains("deadline exceeded"), "{error}");
        assert!(
            started.elapsed() < Duration::from_millis(150),
            "the caller followed the dribbling body instead of its wall-clock deadline"
        );
    }

    #[test]
    fn oversized_body_is_rejected_after_at_most_one_extra_byte() {
        let error = read_with_deadline(
            Box::new(std::io::Cursor::new(vec![b'x'; 6])),
            Duration::from_secs(1),
            5,
            "aware-test-response-reader",
        )
        .unwrap_err();
        assert_eq!(error, "response exceeds the 5-byte limit");
    }
}
