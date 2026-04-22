/// Enumerator to display the possible present mode options.
/// 
/// Directly related to how the frames are displayed to the user.
#[derive(Clone, Debug, Copy)]
pub enum PresentMode {
    AutoVsync,
    AutoNoVsync,
    Fifo,
    FifoRelaxed,
    Immediate,
    Mailbox
}

impl PresentMode {
    /// Returns the PresentMode as a WGPU Struct.
    pub fn to_wgpu(self) -> wgpu::PresentMode {
        match self {
            Self::AutoVsync => wgpu::PresentMode::AutoVsync,
            Self::AutoNoVsync => wgpu::PresentMode::AutoNoVsync,
            Self::Fifo => wgpu::PresentMode::Fifo,
            Self::FifoRelaxed => wgpu::PresentMode::FifoRelaxed,
            Self::Immediate => wgpu::PresentMode::Immediate,
            Self::Mailbox => wgpu::PresentMode::Mailbox
        }
    }
}
