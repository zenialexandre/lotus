use super::{render_type::RenderType};

pub struct RenderBatch {
    pub render_type: RenderType,
    pub texture_path: Option<String>,
    pub instances: Vec<String>,
}
