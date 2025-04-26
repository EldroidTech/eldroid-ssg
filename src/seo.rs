use std::collections::HashMap;
use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Clone)]
pub struct TwitterCard {
    pub card_type: String,
    pub site: Option<String>,
    pub creator: Option<String>,
    pub image: Option<String>,
}

#[derive(Deserialize, Clone)]
pub struct OpenGraph {
    pub title: Option<String>,
    pub description: Option<String>,
    pub image: Option<String>,
    pub url: Option<String>,
    pub site_name: Option<String>,
    pub locale: Option<String>,
    #[serde(rename = "type")]
    pub og_type: Option<String>,
}

#[derive(Deserialize, Clone)]
pub struct RobotsMeta {
    pub index: bool,
    pub follow: bool,
    pub archive: bool,
    pub imageindex: bool,
}

#[derive(Deserialize, Clone)]
pub struct JsonLd {
    pub schema_type: String,
    #[serde(flatten)]
    pub properties: HashMap<String, serde_json::Value>,
}

#[derive(Deserialize, Clone)]
pub struct PageSEO {
    pub title: Option<String>,
    pub description: Option<String>,
    pub canonical_url: Option<String>,
    pub meta_tags: Option<HashMap<String, String>>,
    pub open_graph: Option<OpenGraph>,
    pub twitter_card: Option<TwitterCard>,
    pub robots: Option<RobotsMeta>,
    pub json_ld: Option<Vec<JsonLd>>,
}

#[derive(Deserialize)]
pub struct SEOConfig {
    pub enable_seo: bool,
    pub default_title: Option<String>,
    pub default_description: Option<String>,
    pub site_name: Option<String>,
    pub base_url: Option<String>,
    pub default_language: Option<String>,
    pub alternate_languages: Option<HashMap<String, String>>,
    pub meta_tags: HashMap<String, String>,
    pub open_graph: Option<OpenGraph>,
    pub twitter_card: Option<TwitterCard>,
    pub robots: Option<RobotsMeta>,
    pub json_ld: Option<Vec<JsonLd>>,
}

impl SEOConfig {
    pub fn merge_with_page(&self, page_seo: Option<&PageSEO>) -> PageSEO {
        let mut merged = PageSEO {
            title: self.default_title.clone(),
            description: self.default_description.clone(),
            canonical_url: None,
            meta_tags: Some(self.meta_tags.clone()),
            open_graph: self.open_graph.clone(),
            twitter_card: self.twitter_card.clone(),
            robots: self.robots.clone(),
            json_ld: self.json_ld.clone(),
        };

        if let Some(page) = page_seo {
            if let Some(title) = &page.title {
                merged.title = Some(title.clone());
            }
            if let Some(desc) = &page.description {
                merged.description = Some(desc.clone());
            }
            if let Some(canonical) = &page.canonical_url {
                merged.canonical_url = Some(canonical.clone());
            }
            if let Some(meta) = &page.meta_tags {
                merged.meta_tags.get_or_insert_with(HashMap::new).extend(meta.clone());
            }
            if let Some(og) = &page.open_graph {
                merged.open_graph = Some(og.clone());
            }
            if let Some(twitter) = &page.twitter_card {
                merged.twitter_card = Some(twitter.clone());
            }
            if let Some(robots) = &page.robots {
                merged.robots = Some(robots.clone());
            }
            if let Some(json_ld) = &page.json_ld {
                merged.json_ld = Some(json_ld.clone());
            }
        }

        merged
    }
}

pub fn load_seo_config(config_path: &str) -> Option<SEOConfig> {
    if let Ok(config_content) = fs::read_to_string(config_path) {
        toml::from_str(&config_content).ok()
    } else {
        None
    }
}

pub fn parse_page_seo(content: &str) -> Option<PageSEO> {
    if let Some(start) = content.find("<!--seo") {
        if let Some(end) = content.find("-->") {
            let seo_content = &content[start + 6..end].trim();
            toml::from_str(seo_content).ok()
        } else {
            None
        }
    } else {
        None
    }
}