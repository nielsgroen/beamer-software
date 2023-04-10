use std::path::PathBuf;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ProgramConfig {
    pub config_path: PathBuf,
    pub genius_api_token: Option<String>,
    pub font_size: String,
}
