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
}

#[derive(Debug)]
pub struct PageSEO {
    pub title: String,
    pub description: Option<String>,
    pub keywords: Option<Vec<String>>,
    pub url: String,
    pub canonical_url: Option<String>,
    pub structured_data: Option<String>,
}

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
        match serde_json::from_str::<serde_json::Value>(json) {
            Ok(v) => {
                Some(PageSEO {
                    title: v["title"].as_str()?.to_string(),
                    description: v["description"].as_str().map(String::from),
                    keywords: v["keywords"].as_array().map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str())
                            .map(String::from)
                            .collect()
                    }),
                    url: v["url"].as_str()?.to_string(),
                    canonical_url: v["canonical_url"].as_str().map(String::from),
                    structured_data: v["structured_data"].as_str().map(String::from),
                })
            },
            Err(e) => {
                log::warn!("Failed to parse page SEO data: {}", e);
                None
            }
        }
    })
}