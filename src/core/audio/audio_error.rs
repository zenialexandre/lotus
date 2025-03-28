/// Enumerator to represet the different errors that the audio flow can throw.
#[derive(Debug)]
pub enum AudioError {
    Io(std::io::Error),
    Backend(kira::backend::cpal::Error),
    FromFile(kira::sound::FromFileError),
    PlaySound(kira::PlaySoundError<()>),
    PlaySoundGeneric(Box<dyn std::error::Error + Send + Sync>),
    Generic(anyhow::Error)
}

impl AudioError {
    /// Helper function to convert a PlaySoundError to a generic type of E.
    pub fn from_play_sound_error<E: std::error::Error + Send + Sync + 'static>(error: kira::PlaySoundError<E>) -> Self {
        return AudioError::PlaySoundGeneric(Box::new(error));
    }
}

impl std::fmt::Display for AudioError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AudioError::Io(error) => write!(formatter, "IO Audio Error: {}.", error),
            AudioError::Backend(error) => write!(formatter, "Kira Backend Audio Error: {}.", error),
            AudioError::FromFile(error) => write!(formatter, "Kira From File Audio Error: {}.", error),
            AudioError::PlaySound(error) => write!(formatter, "Kira Play Sound Error: {}.", error),
            AudioError::PlaySoundGeneric(error) => write!(formatter, "Kira Play Sound Generic Error: {}.", error),
            AudioError::Generic(error) => write!(formatter, "Generic Audio Error: {}.", error)
        }
    }
}

impl From<std::io::Error> for AudioError {
    fn from(error: std::io::Error) -> Self {
        return AudioError::Io(error);
    }
}

impl From<kira::backend::cpal::Error> for AudioError {
    fn from(error: kira::backend::cpal::Error) -> Self {
        return AudioError::Backend(error);
    }
}

impl From<kira::sound::FromFileError> for AudioError {
    fn from(error: kira::sound::FromFileError) -> Self {
        return AudioError::FromFile(error);
    }
}

impl From<kira::PlaySoundError<()>> for AudioError {
    fn from(error: kira::PlaySoundError<()>) -> Self {
        return AudioError::PlaySound(error);
    }
}

impl From<anyhow::Error> for AudioError {
    fn from(error: anyhow::Error) -> Self {
        return AudioError::Generic(error);
    }
}
