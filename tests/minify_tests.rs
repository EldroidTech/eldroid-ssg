use eldroid_ssg::minify::Minifier;

#[test]
fn test_html_minification() {
    let minifier = Minifier::default();
    
    let input = r#"
        <html>
            <head>
                <title>Test</title>
            </head>
            <body>
                <div class="test">
                    Hello World
                </div>
            </body>
        </html>
    "#;

    let output = minifier.minify_html(input);
    assert!(!output.contains("\n"));
    assert!(!output.contains("    "));
    assert!(output.contains("<html><head><title>Test</title></head>"));
}

#[test]
fn test_css_minification() {
    let minifier = Minifier::default();
    
    let input = r#"
        .test {
            color: #ffffff;
            padding: 10px;
            margin: 20px;
        }

        .other {
            background-color: #000000;
            display: flex;
        }
    "#;

    let output = minifier.minify_css(input);
    assert!(!output.contains("\n"));
    assert!(!output.contains("    "));
    assert!(output.contains(".test{color:#fff"));
}

#[test]
fn test_js_minification() {
    let minifier = Minifier::default();
    
    let input = r#"
        function test() {
            const message = 'Hello World';
            console.log(message);
            return message;
        }

        const obj = {
            prop: 'value',
            method: function() {
                return this.prop;
            }
        };
    "#;

    let output = minifier.minify_js(input);
    assert!(!output.contains("\n"));
    assert!(!output.contains("    "));
    assert!(output.contains("function test(){"));
}