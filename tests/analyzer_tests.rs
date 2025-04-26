use eldroid_ssg::analyzer::{Analyzer, PerformanceMetrics, SecurityIssue};

#[test]
fn test_performance_analysis() {
    let analyzer = Analyzer::new();
    let html = r#"
        <html>
            <head>
                <script src="large.js"></script>
                <link rel="stylesheet" href="styles.css">
                <img src="large-image.jpg" width="2000" height="1500">
            </head>
            <body>
                <div>
                    <img src="unoptimized.png">
                    <script>
                        // Inline script
                        console.log('test');
                    </script>
                </div>
            </body>
        </html>
    "#;

    let metrics = analyzer.analyze_performance(html);
    
    assert!(metrics.unoptimized_images.len() > 0);
    assert!(metrics.blocking_scripts.len() > 0);
    assert!(metrics.inline_scripts.len() > 0);
    assert!(metrics.unminified_resources.len() > 0);
}

#[test]
fn test_security_analysis() {
    let analyzer = Analyzer::new();
    let html = r#"
        <html>
            <head>
                <script src="http://external.com/script.js"></script>
            </head>
            <body>
                <img src="http://unsecure.com/image.jpg">
                <form action="http://example.com/submit">
                    <input type="text" name="test">
                </form>
                <script>
                    eval('console.log("test")');
                </script>
            </body>
        </html>
    "#;

    let issues = analyzer.analyze_security(html);
    
    // Check for mixed content
    assert!(issues.iter().any(|i| matches!(i, SecurityIssue::MixedContent { .. })));
    
    // Check for unsafe eval
    assert!(issues.iter().any(|i| matches!(i, SecurityIssue::UnsafeEval { .. })));
    
    // Check for insecure form
    assert!(issues.iter().any(|i| matches!(i, SecurityIssue::InsecureForm { .. })));
}

#[test]
fn test_clean_html() {
    let analyzer = Analyzer::new();
    let html = r#"
        <html>
            <head>
                <script src="https://secure.com/script.js"></script>
                <link rel="stylesheet" href="styles.min.css">
            </head>
            <body>
                <img src="https://secure.com/image.jpg" width="800" height="600">
                <form action="https://secure.com/submit">
                    <input type="text" name="test">
                </form>
            </body>
        </html>
    "#;

    let performance = analyzer.analyze_performance(html);
    let security = analyzer.analyze_security(html);

    assert!(performance.unoptimized_images.is_empty());
    assert!(performance.blocking_scripts.is_empty());
    assert!(performance.inline_scripts.is_empty());
    assert!(security.is_empty());
}

#[test]
fn test_resource_optimization() {
    let analyzer = Analyzer::new();
    let css = r#"
        .test {
            background-image: url('large-bg.jpg');
            color: #ffffff;
        }
    "#;
    
    let js = r#"
        function test() {
            console.log('Debug message');
            // TODO: Remove this
            debugger;
        }
    "#;

    let metrics = analyzer.analyze_resources(css, js);
    
    assert!(metrics.unoptimized_images.len() > 0);
    assert!(metrics.debug_statements.len() > 0);
    assert!(metrics.unminified_resources.len() > 0);
}