use embedded_hal::timer::{CountDown, Periodic};
use std::time::{Duration, Instant};

pub struct SimpleTimer {
    duration: Duration,
    start: Option<Instant>,
}

impl SimpleTimer {
    pub fn new() -> Self {
        SimpleTimer {
            duration: Duration::default(),
            start: None,
        }
    }
}

impl CountDown for SimpleTimer {
    type Time = Duration;

    fn start<T>(&mut self, duration: T)
    where
        T: Into<Self::Time>,
    {
        self.duration = duration.into();
        self.start = Some(Instant::now());
    }

    fn wait(&mut self) -> nb::Result<(), void::Void> {
        if let Some(start) = self.start {
            if start.elapsed() >= self.duration {
                Ok(())
            } else {
                Err(nb::Error::WouldBlock)
            }
        } else {
            Err(nb::Error::Other(void::Void))
        }
    }
}

impl Periodic for SimpleTimer {}
