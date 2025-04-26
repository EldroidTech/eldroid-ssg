use std::path::{Path, PathBuf};
use std::fs;
use chrono::{DateTime, Utc};
use crate::seo::SEOConfig;

pub fn generate_sitemap(processed_files: &[PathBuf], config: &SEOConfig, output_dir: &str) -> std::io::Result<()> {
    let mut sitemap = String::from(r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">"#);

    for file in processed_files {
        if let Some(relative_path) = file.strip_prefix(output_dir).ok() {
            let url_path = relative_path
                .to_str()
                .unwrap()
                .replace("\\", "/")
                .trim_start_matches('/')
                .to_string();

            let last_modified = fs::metadata(file)?
                .modified()?;
            
            let datetime: DateTime<Utc> = last_modified.into();
            let lastmod = datetime.format("%Y-%m-%d");

            sitemap.push_str(&format!(r#"
    <url>
        <loc>{}/{}</loc>
        <lastmod>{}</lastmod>
        <changefreq>weekly</changefreq>
        <priority>0.8</priority>
    </url>"#,
                config.base_url.as_ref().unwrap_or(&String::new()),
                url_path,
                lastmod
            ));
        }
    }

    sitemap.push_str("\n</urlset>");
    fs::write(Path::new(output_dir).join("sitemap.xml"), sitemap)?;
    Ok(())
}

pub fn generate_rss(processed_files: &[PathBuf], config: &SEOConfig, output_dir: &str) -> std::io::Result<()> {
    let mut rss = format!(r#"<?xml version="1.0" encoding="UTF-8"?>
<rss version="2.0">
    <channel>
        <title>{}</title>
        <link>{}</link>
        <description>{}</description>
        <language>en-us</language>
        <lastBuildDate>{}</lastBuildDate>"#,
        config.site_name,
        config.base_url.as_ref().unwrap_or(&String::new()),
        config.default_description,
        Utc::now().format("%a, %d %b %Y %H:%M:%S GMT")
    );

    for file in processed_files {
        if let Some(relative_path) = file.strip_prefix(output_dir).ok() {
            if let Ok(content) = fs::read_to_string(file) {
                // Extract title and description from HTML
                let title = extract_title(&content)
                    .unwrap_or_else(|| relative_path.to_string_lossy().to_string());
                let description = extract_description(&content)
                    .unwrap_or_else(|| String::from("No description available"));

                let url_path = relative_path
                    .to_str()
                    .unwrap()
                    .replace("\\", "/")
                    .trim_start_matches('/')
                    .to_string();

                let pub_date = fs::metadata(file)?
                    .modified()?;
                let datetime: DateTime<Utc> = pub_date.into();
                
                rss.push_str(&format!(r#"
        <item>
            <title>{}</title>
            <link>{}/{}</link>
            <description>{}</description>
            <pubDate>{}</pubDate>
            <guid>{}/{}</guid>
        </item>"#,
                    title,
                    config.base_url.as_ref().unwrap_or(&String::new()),
                    url_path,
                    description,
                    datetime.format("%a, %d %b %Y %H:%M:%S GMT"),
                    config.base_url.as_ref().unwrap_or(&String::new()),
                    url_path
                ));
            }
        }
    }

    rss.push_str("\n    </channel>\n</rss>");
    fs::write(Path::new(output_dir).join("feed.rss"), rss)?;
    Ok(())
}

pub fn generate_robots_txt(config: &SEOConfig, output_dir: &str) -> std::io::Result<()> {
    let robots = format!(r#"User-agent: *
Allow: /

# Sitemaps
Sitemap: {}/sitemap.xml
"#,
        config.base_url.as_ref().unwrap_or(&String::new())
    );

    fs::write(Path::new(output_dir).join("robots.txt"), robots)?;
    Ok(())
}

fn extract_title(html: &str) -> Option<String> {
    if let Some(start) = html.find("<title>") {
        if let Some(end) = html[start..].find("</title>") {
            return Some(html[start + 7..start + end].trim().to_string());
        }
    }
    None
}

fn extract_description(html: &str) -> Option<String> {
    if let Some(start) = html.find(r#"<meta name="description" content=""#) {
        if let Some(content_start) = html[start..].find("content=\"") {
            let desc_start = start + content_start + 9;
            if let Some(end) = html[desc_start..].find('"') {
                return Some(html[desc_start..desc_start + end].to_string());
            }
        }
    }
    None
}