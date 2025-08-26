use super::super::{
    super::utils::constants::font::{
        UNDERDOG_REGULAR_PATH,
        CODYSTAR_LIGHT_PATH,
        CODYSTAR_REGULAR_PATH,
        ROBOTO_MONO_PATH,
        ROBOTO_MONO_ITALIC_PATH
    },
    asset_loader::AssetLoader
};

/// Struct to represent a font.
#[derive(Clone)]
pub struct Font {
    pub bytes: Vec<u8>,
    pub size: f32
}

impl Font {
    /// Create a new font struct.
    pub fn new(path: String, size: f32) -> Self {
        let bytes: Vec<u8> = if path.contains("../../") {
            let font: Fonts = Fonts::from_path(path).unwrap();
            font.get_bytes()
        } else {
            AssetLoader::load_bytes(&path).ok().unwrap()
        };

        return Self {
            bytes,
            size
        };
    }
}

/// Enumerator that represents the available default fonts on the engine.
/// The end-user can use it's own fonts at any moment.
#[derive(Clone)]
pub enum Fonts {
    UnderdogRegular,
    CodystarLight,
    CodystarRegular,
    RobotoMono,
    RobotoMonoItalic
}

impl Fonts {
    /// Returns the path of the following font.
    pub fn get_path(&self) -> String {
        return match self {
            Self::UnderdogRegular => UNDERDOG_REGULAR_PATH.to_string(),
            Self::CodystarLight => CODYSTAR_LIGHT_PATH.to_string(),
            Self::CodystarRegular => CODYSTAR_REGULAR_PATH.to_string(),
            Self::RobotoMono => ROBOTO_MONO_PATH.to_string(),
            Self::RobotoMonoItalic => ROBOTO_MONO_ITALIC_PATH.to_string()
        }
    }

    /// Returns the bytes of the following font.
    pub fn get_bytes(&self) -> Vec<u8> {
        return match self {
            Self::UnderdogRegular => include_bytes!("../../../assets/fonts/Underdog-Regular.ttf").to_vec(),
            Self::CodystarLight => include_bytes!("../../../assets/fonts/Codystar-Light.ttf").to_vec(),
            Self::CodystarRegular => include_bytes!("../../../assets/fonts/Codystar-Regular.ttf").to_vec(),
            Self::RobotoMono => include_bytes!("../../../assets/fonts/RobotoMono-VariableFont_wght.ttf").to_vec(),
            Self::RobotoMonoItalic => include_bytes!("../../../assets/fonts/RobotoMono-Italic-VariableFont_wght.ttf").to_vec()
        }
    }

    /// Returns the enumerator value from the path.
    pub fn from_path(path: String) -> Option<Self> {
        let path_as_str: &str = &path;

        return match path_as_str {
            UNDERDOG_REGULAR_PATH => Some(Self::UnderdogRegular),
            CODYSTAR_LIGHT_PATH => Some(Self::CodystarLight),
            CODYSTAR_REGULAR_PATH => Some(Self::CodystarRegular),
            ROBOTO_MONO_PATH => Some(Self::RobotoMono),
            ROBOTO_MONO_ITALIC_PATH => Some(Self::RobotoMonoItalic),
            _ => None
        }
    }
}
