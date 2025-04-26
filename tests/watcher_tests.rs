use std::path::PathBuf;
use std::time::Duration;
use tokio::fs;
use tempfile::tempdir;
use eldroid_ssg::watcher::DevServer;

#[tokio::test]
async fn test_dev_server_creation() {
    let temp_dir = tempdir().unwrap();
    let input_dir = temp_dir.path().join("content");
    let output_dir = temp_dir.path().join("output");
    let components_dir = temp_dir.path().join("components");

    fs::create_dir_all(&input_dir).await.unwrap();
    fs::create_dir_all(&output_dir).await.unwrap();
    fs::create_dir_all(&components_dir).await.unwrap();

    let server = DevServer::new(
        input_dir,
        output_dir,
        components_dir,
        Some(3000),
        Some(3001)
    );

    assert_eq!(server.get_changed_files().len(), 0);
}

#[tokio::test]
async fn test_file_change_detection() {
    let temp_dir = tempdir().unwrap();
    let input_dir = temp_dir.path().join("content");
    let output_dir = temp_dir.path().join("output");
    let components_dir = temp_dir.path().join("components");

    fs::create_dir_all(&input_dir).await.unwrap();
    fs::create_dir_all(&output_dir).await.unwrap();
    fs::create_dir_all(&components_dir).await.unwrap();

    let server = DevServer::new(
        input_dir.clone(),
        output_dir,
        components_dir,
        Some(3002),
        Some(3003)
    );

    // Start server in background
    let server_handle = tokio::spawn(async move {
        server.start().await.unwrap();
    });

    // Create a test file
    let test_file = input_dir.join("test.html");
    fs::write(&test_file, "<html><body>Test</body></html>").await.unwrap();

    // Give watcher time to detect change
    tokio::time::sleep(Duration::from_millis(100)).await;

    // Modify the test file
    fs::write(&test_file, "<html><body>Updated</body></html>").await.unwrap();

    // Give watcher time to detect change
    tokio::time::sleep(Duration::from_millis(100)).await;

    server_handle.abort();
}

#[tokio::test]
async fn test_multiple_file_changes() {
    let temp_dir = tempdir().unwrap();
    let input_dir = temp_dir.path().join("content");
    let output_dir = temp_dir.path().join("output");
    let components_dir = temp_dir.path().join("components");

    fs::create_dir_all(&input_dir).await.unwrap();
    fs::create_dir_all(&output_dir).await.unwrap();
    fs::create_dir_all(&components_dir).await.unwrap();

    let server = DevServer::new(
        input_dir.clone(),
        output_dir,
        components_dir.clone(),
        Some(3004),
        Some(3005)
    );

    // Start server in background
    let server_handle = tokio::spawn(async move {
        server.start().await.unwrap();
    });

    // Create multiple test files
    let test_files = vec![
        input_dir.join("test1.html"),
        input_dir.join("test2.html"),
        components_dir.join("header.html"),
    ];

    for file in &test_files {
        fs::write(file, "<html><body>Test</body></html>").await.unwrap();
    }

    // Give watcher time to detect changes
    tokio::time::sleep(Duration::from_millis(200)).await;

    server_handle.abort();
}