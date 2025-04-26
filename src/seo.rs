use std::collections::HashMap;
use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct SEOConfig {
    pub enable_seo: bool,
    pub default_title: Option<String>,
    pub default_description: Option<String>,
    pub meta_tags: HashMap<String, String>,
}

pub fn load_seo_config(config_path: &str) -> Option<SEOConfig> {
    if let Ok(config_content) = fs::read_to_string(config_path) {
        toml::from_str(&config_content).ok()
    } else {
        None
    }
}