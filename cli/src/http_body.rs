//! Bounded response-body reads for synchronous HTTP clients.

use std::io::Read;
use std::time::{Duration, Instant};

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
