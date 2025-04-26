use std::sync::Arc;
use eldroid_ssg::html::{HtmlGenerator, generate_html_with_seo};
use eldroid_ssg::seo::{SEOConfig, PageSEO};

#[test]
fn test_html_generation() {
    let generator = HtmlGenerator::new();
    let input = "<html><body>Test</body></html>";
    let output = generator.generate(input);
    assert_eq!(output, input);
}

#[test]
fn test_html_with_seo() {
    let generator = Arc::new(HtmlGenerator::new());
    let config = SEOConfig {
        site_name: "Test Site".to_string(),
        base_url: Some("https://example.com".to_string()),
        default_description: "Default description".to_string(),
        default_keywords: vec!["test".to_string()],
        twitter_handle: None,
        facebook_app_id: None,
        google_site_verification: None,
    };

    let input = r#"<!-- SEO {
        "title": "Test Page",
        "description": "Test description",
        "keywords": ["test"],
        "url": "/test"
    } -->
    <html>
        <head></head>
        <body>Test</body>
    </html>"#;

    let output = generate_html_with_seo(input, &config, &generator);
    assert!(output.contains("<title>Test Page | Test Site</title>"));
    assert!(output.contains(r#"<meta name="description" content="Test description">"#));
    assert!(output.contains(r#"<meta property="og:title""#));
}

#[test]
fn test_html_without_seo_comment() {
    let generator = Arc::new(HtmlGenerator::new());
    let config = SEOConfig {
        site_name: "Test Site".to_string(),
        base_url: None,
        default_description: "Default description".to_string(),
        default_keywords: vec![],
        twitter_handle: None,
        facebook_app_id: None,
        google_site_verification: None,
    };

    let input = "<html><head></head><body>Test</body></html>";
    let output = generate_html_with_seo(input, &config, &generator);
    assert_eq!(output, input);
}

#[test]
fn test_html_with_existing_meta() {
    let generator = Arc::new(HtmlGenerator::new());
    let config = SEOConfig {
        site_name: "Test Site".to_string(),
        base_url: Some("https://example.com".to_string()),
        default_description: "Default description".to_string(),
        default_keywords: vec!["test".to_string()],
        twitter_handle: None,
        facebook_app_id: None,
        google_site_verification: None,
    };

    let input = r#"<!-- SEO {
        "title": "New Title",
        "description": "New description",
        "url": "/test"
    } -->
    <html>
        <head>
            <title>Old Title</title>
            <meta name="description" content="Old description">
        </head>
        <body>Test</body>
    </html>"#;

    let output = generate_html_with_seo(input, &config, &generator);
    assert!(output.contains("<title>New Title | Test Site</title>"));
    assert!(output.contains(r#"<meta name="description" content="New description">"#));
    assert!(!output.contains("Old Title"));
    assert!(!output.contains("Old description"));
}