use std::time::Duration;
use lotus_proc_macros::Resource;

/// Enumerator to define the type of the timer.
#[derive(Clone, Default, Debug, PartialEq)]
pub enum TimerType {
    #[default]
    Once,
    Repeat
}

/// Struct to represent a timer that can be used at easy by the end-user.
#[derive(Clone, Debug, Resource)]
pub struct Timer {
    pub timer_type: TimerType,
    pub duration: Duration,
    pub elapsed: Duration,
    pub is_finished: bool
}

impl Timer {
    /// Create a new timer with parameters.
    pub fn new(timer_type: TimerType, duration: Duration) -> Self {
        return Self {
            timer_type,
            duration,
            elapsed: Duration::ZERO,
            is_finished: false
        }
    }

    /// Initialize the timer countdown.
    pub fn tick(&mut self, delta: f32) {
        self.elapsed += Duration::from_secs_f32(delta);
        self.is_finished = false;

        if self.elapsed >= self.duration {
            match self.timer_type {
                TimerType::Once => {
                    self.is_finished = true;
                    self.elapsed = self.duration;
                },
                TimerType::Repeat => {
                    self.is_finished = true;
                    self.elapsed = Duration::ZERO;
                }
            }
        }
    }

    /// Resets the timer to its initial state.
    pub fn reset(&mut self) {
        self.is_finished = false;
        self.elapsed = Duration::ZERO;
    }

    /// Returns if the timer is already finished.
    pub fn is_finished(&self) -> bool {
        return self.is_finished;
    }

    /// Returns the duration time as seconds on the f32 format.
    pub fn duration_as_secs_f32(&self) -> f32 {
        return self.duration.as_secs_f32();
    }

    /// Returns the elapsed time as seconds on the f32 format.
    pub fn elapsed_as_secs_f32(&self) -> f32 {
        return self.elapsed.as_secs_f32();
    }
}
