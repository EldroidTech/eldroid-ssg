use std::fs;
use std::path::Path;
use chrono::{Utc, TimeZone};
use sitemap::writer::SiteMapWriter;
use sitemap::structs::{UrlEntry, ChangeFreq};
use rss::{ChannelBuilder, ItemBuilder};
use crate::seo::{SEOConfig, PageSEO};
use log::info;

pub fn generate_sitemap(
    base_url: &str,
    pages: &[(String, PageSEO)],
    output_dir: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let sitemap_path = output_dir.join("sitemap.xml");
    let file = fs::File::create(&sitemap_path)?;
    let writer = SiteMapWriter::new(file);
    let mut urlwriter = writer.start_urlset()?;

    for (path, _page_seo) in pages {
        let url = format!("{}{}", base_url.trim_end_matches('/'), path);
        let mut url_entry = UrlEntry::builder()
            .loc(url)
            .changefreq(ChangeFreq::Weekly)
            .priority(0.5);

        // Use last modified time if available
        if let Ok(metadata) = fs::metadata(output_dir.join(path)) {
            if let Ok(modified) = metadata.modified() {
                // Convert SystemTime to DateTime<Utc> then to DateTime<FixedOffset>
                let timestamp = modified.duration_since(std::time::UNIX_EPOCH)
                    .map(|d| d.as_secs())
                    .unwrap_or(0);
                if let Some(datetime) = Utc.timestamp_opt(timestamp as i64, 0).earliest() {
                    url_entry = url_entry.lastmod(datetime.into());
                }
            }
        }

        urlwriter.url(url_entry.build()?)?;
    }

    urlwriter.end()?;
    info!("Generated sitemap at {}", sitemap_path.display());
    Ok(())
}

pub fn generate_rss(
    config: &SEOConfig,
    pages: &[(String, PageSEO)],
    output_dir: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let base_url = config.base_url.as_deref().unwrap_or("/");
    let site_name = config.site_name.as_deref().unwrap_or("Site Feed");

    let mut items = Vec::new();
    for (path, page_seo) in pages {
        let url = format!("{}{}", base_url.trim_end_matches('/'), path);
        let title = page_seo.title.as_deref().unwrap_or("Untitled");
        let description = page_seo.description.as_deref().unwrap_or("");

        let item = ItemBuilder::default()
            .title(title.to_string())
            .link(url)
            .description(description.to_string())
            .build();
        items.push(item);
    }

    let channel = ChannelBuilder::default()
        .title(site_name.to_string())
        .link(base_url.to_string())
        .description(config.default_description.clone().unwrap_or_default())
        .items(items)
        .build();

    let rss_path = output_dir.join("feed.xml");
    fs::write(&rss_path, channel.to_string())?;
    info!("Generated RSS feed at {}", rss_path.display());
    Ok(())
}

pub fn generate_robots_txt(
    config: &SEOConfig,
    output_dir: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let base_url = config.base_url.as_deref().unwrap_or("/");
    let robots_content = format!(
        "User-agent: *\n\
        Allow: /\n\
        \n\
        Sitemap: {}/sitemap.xml",
        base_url.trim_end_matches('/')
    );

    let robots_path = output_dir.join("robots.txt");
    fs::write(&robots_path, robots_content)?;
    info!("Generated robots.txt at {}", robots_path.display());
    Ok(())
}