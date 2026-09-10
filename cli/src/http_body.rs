//! Deadline helpers for synchronous HTTP clients.

use std::io::Read;
use std::net::{SocketAddr, ToSocketAddrs};
use std::time::{Duration, Instant};

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
