use eldroid_ssg::seo::{SEOConfig, PageSEO, parse_page_seo};

#[test]
fn test_parse_page_seo() {
    let content = r#"<!-- SEO {
        "title": "Test Page",
        "description": "Test description",
        "keywords": ["test", "seo"],
        "url": "/test",
        "canonical_url": "https://example.com/test",
        "structured_data": "{\"@type\": \"Article\"}"
    } -->
    <html><body>Test</body></html>"#;

    let seo = parse_page_seo(content).unwrap();
    assert_eq!(seo.title, "Test Page");
    assert_eq!(seo.description.unwrap(), "Test description");
    assert_eq!(seo.keywords.unwrap(), vec!["test", "seo"]);
    assert_eq!(seo.url, "/test");
    assert_eq!(seo.canonical_url.unwrap(), "https://example.com/test");
    assert!(seo.structured_data.is_some());
}

#[test]
fn test_parse_invalid_seo() {
    let content = r#"<!-- SEO {
        invalid json
    } -->
    <html><body>Test</body></html>"#;

    assert!(parse_page_seo(content).is_none());
}

#[test]
fn test_parse_minimal_seo() {
    let content = r#"<!-- SEO {
        "title": "Test",
        "url": "/test"
    } -->
    <html><body>Test</body></html>"#;

    let seo = parse_page_seo(content).unwrap();
    assert_eq!(seo.title, "Test");
    assert_eq!(seo.url, "/test");
    assert!(seo.description.is_none());
    assert!(seo.keywords.is_none());
    assert!(seo.canonical_url.is_none());
    assert!(seo.structured_data.is_none());
}