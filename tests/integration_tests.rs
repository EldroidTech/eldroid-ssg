use assert_fs::prelude::*;
use assert_fs::TempDir;
use eldroid_ssg::{CliArgs, BuildConfig};
use predicates::prelude::*;
use std::path::PathBuf;

#[tokio::test]
async fn test_basic_build() {
    let temp = TempDir::new().unwrap();
    
    // Create test content
    temp.child("content")
        .child("index.html")
        .write_str("<html><body>Test</body></html>")
        .unwrap();

    let args = CliArgs {
        input_dir: temp.child("content").to_path_buf().to_string_lossy().into(),
        output_dir: temp.child("output").to_path_buf().to_string_lossy().into(),
        components_dir: temp.child("components").to_path_buf().to_string_lossy().into(),
        release: false,
        analyze_performance: false,
        enable_seo: false,
        seo_config: PathBuf::from("seo_config.toml"),
        minify: false,
        security_checks: false,
        watch: false,
        port: None,
        ws_port: None,
    };

    // Run build
    let config = BuildConfig::from(&args);
    assert!(temp.child("output/index.html").exists());
}

#[tokio::test]
async fn test_seo_generation() {
    let temp = TempDir::new().unwrap();
    
    // Create test content with SEO metadata
    temp.child("content")
        .child("index.html")
        .write_str(r#"<!-- SEO {
            "title": "Test Page",
            "description": "Test description",
            "keywords": ["test"],
            "url": "/test"
        } -->
        <html><body>Test</body></html>"#)
        .unwrap();

    // Create SEO config
    temp.child("seo_config.toml")
        .write_str(r#"
        site_name = "Test Site"
        base_url = "https://test.com"
        default_description = "Default description"
        default_keywords = ["default"]
        "#)
        .unwrap();

    let args = CliArgs {
        input_dir: temp.child("content").to_path_buf().to_string_lossy().into(),
        output_dir: temp.child("output").to_path_buf().to_string_lossy().into(),
        components_dir: temp.child("components").to_path_buf().to_string_lossy().into(),
        release: false,
        analyze_performance: false,
        enable_seo: true,
        seo_config: temp.child("seo_config.toml").to_path_buf(),
        minify: false,
        security_checks: false,
        watch: false,
        port: None,
        ws_port: None,
    };

    // Run build
    let config = BuildConfig::from(&args);
    
    // Verify outputs
    assert!(temp.child("output/sitemap.xml").exists());
    assert!(temp.child("output/robots.txt").exists());
    assert!(temp.child("output/feed.rss").exists());

    // Check SEO meta tags
    let output = std::fs::read_to_string(temp.child("output/index.html")).unwrap();
    assert!(output.contains("<title>Test Page | Test Site</title>"));
    assert!(output.contains(r#"<meta name="description" content="Test description">"#));
}

#[tokio::test]
async fn test_watch_mode() {
    let temp = TempDir::new().unwrap();
    
    // Create initial content
    temp.child("content")
        .child("index.html")
        .write_str("<html><body>Initial</body></html>")
        .unwrap();

    let args = CliArgs {
        input_dir: temp.child("content").to_path_buf().to_string_lossy().into(),
        output_dir: temp.child("output").to_path_buf().to_string_lossy().into(),
        components_dir: temp.child("components").to_path_buf().to_string_lossy().into(),
        release: false,
        analyze_performance: false,
        enable_seo: false,
        seo_config: PathBuf::from("seo_config.toml"),
        minify: false,
        security_checks: false,
        watch: true,
        port: Some(0),  // Random port
        ws_port: Some(0),  // Random port
    };

    // Start server in background
    let config = BuildConfig::from(&args);
    let server_handle = tokio::spawn(async move {
        // Server code here
    });

    // Modify file
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    temp.child("content")
        .child("index.html")
        .write_str("<html><body>Updated</body></html>")
        .unwrap();

    // Wait for rebuild
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    // Verify update
    let output = std::fs::read_to_string(temp.child("output/index.html")).unwrap();
    assert!(output.contains("Updated"));

    server_handle.abort();
}

#[tokio::test]
async fn test_security_checks() {
    let temp = TempDir::new().unwrap();
    
    // Create test content with security issues
    temp.child("content")
        .child("index.html")
        .write_str(r#"<html>
            <body>
                <img src="http://insecure.com/image.jpg">
                <script>alert('inline');</script>
            </body>
        </html>"#)
        .unwrap();

    let args = CliArgs {
        input_dir: temp.child("content").to_path_buf().to_string_lossy().into(),
        output_dir: temp.child("output").to_path_buf().to_string_lossy().into(),
        components_dir: temp.child("components").to_path_buf().to_string_lossy().into(),
        release: false,
        analyze_performance: false,
        enable_seo: false,
        seo_config: PathBuf::from("seo_config.toml"),
        minify: false,
        security_checks: true,
        watch: false,
        port: None,
        ws_port: None,
    };

    // Run build
    let config = BuildConfig::from(&args);
    
    // Verify security warnings were generated
    // (Implementation specific - check logs or report file)
}