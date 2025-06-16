use serde::Deserialize;
use std::path::Path;
use std::fs;
use regex::Regex;
use lazy_static::lazy_static;

#[derive(Debug, Deserialize)]
pub struct SEOConfig {
    pub site_name: String,
    pub base_url: Option<String>,
    pub default_description: String,
    pub default_keywords: Vec<String>,
    pub twitter_handle: Option<String>,
    pub facebook_app_id: Option<String>,
    pub google_site_verification: Option<String>,
    pub organization: Option<Organization>,
    pub default_language: Option<String>,
    pub social_media: Option<SocialMedia>,
    pub structured_data: Option<StructuredData>,
}

#[derive(Debug, Deserialize)]
pub struct Organization {
    pub name: String,
    pub logo: Option<String>,
    pub social_profiles: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct SocialMedia {
    pub twitter_site: Option<String>,
    pub twitter_creator: Option<String>,
    pub facebook_page: Option<String>,
    pub linkedin_page: Option<String>,
    pub instagram_profile: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct StructuredData {
    pub site_search_url: Option<String>,
    pub contact_point: Option<ContactPoint>,
    pub same_as: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct ContactPoint {
    pub telephone: String,
    pub contact_type: String,
    pub email: Option<String>,
    pub area_served: Option<String>,
    pub available_language: Option<Vec<String>>,
}

// PageSEO is now defined in seo_types.rs
pub use crate::seo_types::PageSEO;

pub fn load_seo_config(config_path: &Path) -> Option<SEOConfig> {
    match fs::read_to_string(config_path) {
        Ok(content) => match toml::from_str(&content) {
            Ok(config) => Some(config),
            Err(e) => {
                log::error!("Failed to parse SEO config: {}", e);
                None
            }
        },
        Err(e) => {
            log::error!("Failed to read SEO config file: {}", e);
            None
        }
    }
}

pub fn parse_page_seo(content: &str) -> Option<PageSEO> {
    lazy_static! {
        static ref SEO_COMMENT: Regex = Regex::new(
            r"<!--\s*SEO\s*\{(?P<json>.*?)\}\s*-->"
        ).unwrap();
    }

    SEO_COMMENT.captures(content).and_then(|cap| {
        let json = cap.name("json")?.as_str();
        match serde_json::from_str::<PageSEO>(json) {
            Ok(page_seo) => Some(page_seo),
            Err(e) => {
                log::warn!("Failed to parse page SEO data: {}", e);
                None
            }
        }
    })
}