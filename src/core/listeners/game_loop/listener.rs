use lotus_proc_macros::Resource;

/// Struct to update the current engine state as the end-user.
#[derive(Clone, Debug, Resource)]
pub struct GameLoopListener {
    pub state: GameLoopState,
    pub current_fps: u32,
    pub fps_cap: Option<u32>,
}

impl GameLoopListener {
    /// Create a new loop listener.
    pub fn new() -> Self {
        return Self {
            state: GameLoopState::Running,
            current_fps: 60,
            fps_cap: None
        };
    }

    /// Update the current loop status to Paused.
    pub(crate) fn _pause(&mut self) {
        self.state = GameLoopState::Paused;
    }

    /// Update the current loop status to Running.
    pub(crate) fn _resume(&mut self) {
        self.state = GameLoopState::Running;
    }

    /// Enable FPS capping.
    pub fn fps_cap(&mut self, fps_cap: u32) {
        self.fps_cap = Some(fps_cap);
    }
}

/// Enumerator to store the engine current state.
#[derive(Clone, Default, Debug, PartialEq)]
pub enum GameLoopState {
    #[default]
    Running,
    Paused
}
