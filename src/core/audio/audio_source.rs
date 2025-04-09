use std::{collections::HashMap, path::Path};
use anyhow::Ok;
use kira::{
    sound::{
        static_sound::{StaticSoundData, StaticSoundSettings},
        streaming::{StreamingSoundData, StreamingSoundHandle, StreamingSoundSettings},
        FromFileError,
        IntoOptionalRegion,
        PlaybackPosition,
        Region
    },
    AudioManager,
    AudioManagerSettings,
    Decibels,
    Panning,
    PlaybackRate,
    StartTime,
    Tween,
    Value
};
use lotus_proc_macros::Resource;
use super::audio_error::AudioError;

/// Struct to abstract the configurations for all types of audio.
pub struct AudioSettings {
    pub start_time: StartTime,
    pub start_position: PlaybackPosition,
    pub loop_region: Option<Region>,
    pub reverse: bool,
    pub volume: Value<Decibels>,
    pub playback_rate: Value<PlaybackRate>,
    pub panning: Value<Panning>,
    pub fade_in_tween: Option<Tween>
}

impl Default for AudioSettings {
    fn default() -> Self {
        return Self {
            start_time: StartTime::default(),
			start_position: PlaybackPosition::Seconds(0.0),
			reverse: false,
			loop_region: None,
			volume: Value::Fixed(Decibels::IDENTITY),
			playback_rate: Value::Fixed(PlaybackRate(1.0)),
			panning: Value::Fixed(Panning::CENTER),
			fade_in_tween: None
        };
    }
}

impl AudioSettings {
    /// Returns the audio settings with a start time.
	pub fn start_time(self, start_time: impl Into<StartTime>) -> Self {
		return Self {
			start_time: start_time.into(),
			..self
		};
	}

	/// Returns the audio settings with a start position.
	pub fn start_position(self, start_position: impl Into<PlaybackPosition>) -> Self {
		return Self {
			start_position: start_position.into(),
			..self
		};
	}

	/// Returns the audio settings but with the loop behaviour.
	pub fn loop_region(self, loop_region: impl IntoOptionalRegion) -> Self {
		return Self {
			loop_region: loop_region.into_optional_region(),
			..self
		};
	}

	/// Returns the audio settings with a volume.
	pub fn volume(self, volume: impl Into<Value<Decibels>>) -> Self {
		return Self {
			volume: volume.into(),
			..self
		};
	}

	/// Returns the audio settings with the playback rate.
	pub fn playback_rate(self, playback_rate: impl Into<Value<PlaybackRate>>) -> Self {
		return Self {
			playback_rate: playback_rate.into(),
			..self
		};
	}

    /// Returns the audio settings with the panning.
	pub fn panning(self, panning: impl Into<Value<Panning>>) -> Self {
		return Self {
			panning: panning.into(),
			..self
		};
	}

    /// Returns the audio settings with a fade in tween.
	pub fn fade_in_tween(self, fade_in_tween: impl Into<Option<Tween>>) -> Self {
		return Self {
			fade_in_tween: fade_in_tween.into(),
			..self
		};
	}

    /// Convert the abstract settings to a static sound specific.
    pub fn convert_to_static(&self) -> StaticSoundSettings {
        return StaticSoundSettings {
            start_time: self.start_time,
            start_position: self.start_position,
            loop_region: self.loop_region,
            reverse: self.reverse,
            volume: self.volume,
            playback_rate: self.playback_rate,
            panning: self.panning,
            fade_in_tween: self.fade_in_tween
        };
    }

    pub fn default_of_static() -> StaticSoundSettings {
        return StaticSoundSettings::default();
    }

    /// Convert the abstract settings to a streaming sound specific.
    pub fn convert_to_streaming(&self) -> StreamingSoundSettings {
        return StreamingSoundSettings {
            start_time: self.start_time,
            start_position: self.start_position,
            loop_region: self.loop_region,
            volume: self.volume,
            playback_rate: self.playback_rate,
            panning: self.panning,
            fade_in_tween: self.fade_in_tween
        };
    }

    pub fn default_of_streaming() -> StreamingSoundSettings {
        return StreamingSoundSettings::default();
    }
}

/// Struct to represent the audio resource that the end-user will use.
#[derive(Resource)]
pub struct AudioSource {
    pub audio_manager: AudioManager,
    pub static_sounds: HashMap<String, StaticSoundData>,
    pub streaming_sounds: HashMap<String, StreamingSoundData<FromFileError>>,
    pub streaming_handles: HashMap<String, StreamingSoundHandle<FromFileError>>
}

unsafe impl Send for AudioSource {}
unsafe impl Sync for AudioSource {}

impl AudioSource {
    /// Create a new audio source.
    pub fn new() -> Result<Self, AudioError> {
        let audio_manager: AudioManager = AudioManager::new(AudioManagerSettings::default()).map_err(AudioError::from)?;

        return Ok(Self {
            audio_manager,
            static_sounds: HashMap::new(),
            streaming_sounds: HashMap::new(),
            streaming_handles: HashMap::new()
        }).map_err(AudioError::from);
    }

    /// Check the audio source file format.
    /// The formats allowed are 'wav', 'ogg' and 'flac'.
    pub fn check_sound_file_format(&mut self, path: impl AsRef<Path>) -> Result<(), AudioError> {
        let file_extension: Option<String> = path.as_ref()
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.to_lowercase());

        match file_extension.as_deref() {
            Some("wav" | "ogg" | "flac") => Ok(()).map_err(AudioError::from),
            _ => Err(AudioError::Generic(anyhow::anyhow!("Format not supported! Use 'wav', 'ogg' or 'flac' instead.")))
        }
    }

    /// Loads a static sound to the audio source.
    pub fn load_static_sound(&mut self, name: impl Into<String>, path: impl AsRef<Path>, audio_settings: AudioSettings) -> Result<(), AudioError> {
        self.check_sound_file_format(&path)?;

        let static_sound_data: StaticSoundData = StaticSoundData::from_file(path).map_err(AudioError::from)?;
        self.static_sounds.insert(name.into(), static_sound_data.with_settings(audio_settings.convert_to_static()).clone());
        return Ok(()).map_err(AudioError::from);
    }

    /// Loads a streaming sound to the audio source.
    pub fn load_streaming_sound(&mut self, name: impl Into<String>, path: impl AsRef<Path>, audio_settings: AudioSettings) -> Result<(), AudioError> {
        self.check_sound_file_format(&path)?;

        let streaming_sound_data: StreamingSoundData<FromFileError> = StreamingSoundData::from_file(path).map_err(AudioError::from)?;
        self.streaming_sounds.insert(name.into(), streaming_sound_data.with_settings(audio_settings.convert_to_streaming()));
        return Ok(()).map_err(AudioError::from);
    }

    /// Plays a static sound from the audio source.
    pub fn play_static_sound(&mut self, name: String) -> Result<(), AudioError> {
        if let Some(static_sound_data) = self.static_sounds.get(&name) {
            self.audio_manager.play(static_sound_data.clone()).map_err(AudioError::from)?;
        }
        return Ok(()).map_err(AudioError::from);
    }

    /// Plays a streaming sound from the audio source.
    pub fn play_streaming_sound(&mut self, name: String) -> Result<(), AudioError> {
        if let Some(streaming_sound_data) = self.streaming_sounds.remove(&name) {
            let stremaing_sound_handle: StreamingSoundHandle<FromFileError> = self.audio_manager.play::<StreamingSoundData<FromFileError>>(streaming_sound_data).map_err(AudioError::from_play_sound_error)?;
            self.streaming_handles.insert(name.clone(), stremaing_sound_handle);
        }
        return Ok(()).map_err(AudioError::from);
    }

    /// Resumes a streaming sound from the audio source by its handle.
    pub fn resume_streaming_sound(&mut self, name: String) -> Result<(), AudioError> {
        if let Some(stremaing_sound_handle) = self.streaming_handles.get_mut(&name) {
            stremaing_sound_handle.resume(Tween::default());
        }
        return Ok(()).map_err(AudioError::from);
    }

    /// Pauses a streaming sound from the audio source by its handle.
    pub fn pause_streaming_sound(&mut self, name: String) -> Result<(), AudioError> {
        if let Some(streaming_sound_handle) = self.streaming_handles.get_mut(&name) {
            streaming_sound_handle.pause(Tween::default());
        }
        return Ok(()).map_err(AudioError::from);
    }

    /// Stop a streaming sound from the audio source by its handle.
    /// This operation causes the complete removal of the streaming sound handle!
    pub fn stop_streaming_sound(&mut self, name: String) -> Result<(), AudioError> {
        if let Some(mut streaming_sound_handle) = self.streaming_handles.remove(&name) {
            streaming_sound_handle.stop(Tween::default());
        }
        return Ok(()).map_err(AudioError::from);
    }
}
