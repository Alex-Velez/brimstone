use std::time::{Duration, Instant};

#[derive(Debug, PartialEq)]
pub struct Timer {
    instant: Option<Instant>,
    pub wait_time: Duration,
}

impl Timer {
    pub const fn new(wait_time: Duration) -> Timer {
        Timer {
            instant: None,
            wait_time,
        }
    }

    pub fn from_secs_f32(secs: f32) -> Timer {
        Timer::new(Duration::from_secs_f32(secs))
    }

    pub fn set_wait_time(&mut self, secs: f32) {
        self.wait_time = Duration::from_secs_f32(secs);
    }

    pub fn time_elapsed(&self) -> Duration {
        if let Some(instant) = self.instant {
            instant.elapsed()
        } else {
            Duration::ZERO
        }
    }

    pub fn is_finished(&mut self) -> bool {
        if let Some(instant) = self.instant {
            if instant.elapsed() > self.wait_time {
                self.start();
                true
            } else {
                false
            }
        } else {
            self.start();
            true
        }
    }

    pub fn start(&mut self) {
        self.instant = Some(Instant::now());
    }

    pub fn stop(&mut self) {
        self.instant = None;
    }
}
