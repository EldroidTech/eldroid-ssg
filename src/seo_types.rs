use serde::{Serialize, Deserialize};
use chrono::{DateTime, FixedOffset};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PageSEO {
    pub title: String,
    pub description: Option<String>,
    pub keywords: Option<Vec<String>>,
    pub url: Option<String>,
    pub canonical_url: Option<String>,
    pub path: String,
    pub image: Option<String>,
    pub author: Option<String>,
    pub published_date: Option<DateTime<FixedOffset>>,
    pub last_modified: Option<DateTime<FixedOffset>>,
    pub category: Option<String>,
    pub tags: Option<Vec<String>>,
    pub schema_type: Option<String>,
    pub structured_data: Option<serde_json::Value>,
    pub change_frequency: Option<String>,
    pub priority: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonLd {
    #[serde(rename = "@context")]
    pub context: String,
    #[serde(rename = "@type")]
    pub type_: String,
    pub headline: String,
    pub description: Option<String>,
    pub url: String,
    pub image: Option<Vec<String>>,
    pub author: Option<Author>,
    pub publisher: Option<Organization>,
    pub date_published: Option<String>,
    pub date_modified: Option<String>,
    pub is_accessible_for_free: bool,
    pub keywords: Option<String>,
    pub article_section: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Author {
    #[serde(rename = "@type")]
    pub type_: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Organization {
    #[serde(rename = "@type")]
    pub type_: String,
    pub name: String,
    pub logo: Option<ImageObject>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageObject {
    #[serde(rename = "@type")]
    pub type_: String,
    pub url: String,
}

impl JsonLd {
    pub fn new_article(page: &PageSEO, config: &crate::seo::SEOConfig) -> Self {
        let base_url = config.base_url.as_deref().unwrap_or("");
        let full_url = format!("{}/{}", base_url.trim_end_matches('/'), page.path);

        Self {
            context: "https://schema.org".to_string(),
            type_: "Article".to_string(),
            headline: page.title.clone(),
            description: page.description.clone(),
            url: full_url,
            image: page.image.as_ref().map(|img| vec![img.clone()]),
            author: page.author.as_ref().map(|name| Author {
                type_: "Person".to_string(),
                name: name.clone(),
            }),
            publisher: config.organization.as_ref().map(|org| Organization {
                type_: "Organization".to_string(),
                name: org.name.clone(),
                logo: org.logo.as_ref().map(|url| ImageObject {
                    type_: "ImageObject".to_string(),
                    url: url.clone(),
                }),
            }),
            date_published: page.published_date.map(|dt| dt.to_rfc3339()),
            date_modified: page.last_modified.map(|dt| dt.to_rfc3339()),
            is_accessible_for_free: true,
            keywords: page.tags.as_ref().map(|tags| tags.join(", ")),
            article_section: page.category.clone(),
        }
    }
}
