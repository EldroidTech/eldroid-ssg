use minify_html::{minify as minify_html_content, Cfg};
use lightningcss::{
    stylesheet::{MinifyOptions, ParserOptions, StyleSheet},
    targets::Browsers,
};
use swc::{try_with_handler, HandlerOpts, config::JsMinifyOptions};
use swc_common::SourceMap;
use log::warn;

pub struct Minifier {
    html_config: Cfg,
    css_options: MinifyOptions,
    js_options: JsMinifyOptions,
}

impl Default for Minifier {
    fn default() -> Self {
        Self {
            html_config: Cfg {
                do_not_minify_doctype: true,
                ensure_spec_compliant_unquoted_attribute_values: true,
                keep_closing_tags: true,
                keep_html_and_head_opening_tags: true,
                keep_spaces_between_attributes: true,
                minify_css: true,
                minify_js: true,
                remove_bangs: false,
                remove_processing_instructions: true,
            },
            css_options: MinifyOptions {
                targets: Some(Browsers::default()),
                ..MinifyOptions::default()
            },
            js_options: JsMinifyOptions::default(),
        }
    }
}

impl Minifier {
    pub fn minify_html(&self, content: &str) -> String {
        String::from_utf8_lossy(&minify_html_content(
            content.as_bytes(),
            &self.html_config
        )).into_owned()
    }

    pub fn minify_css(&self, content: &str) -> String {
        let stylesheet = match StyleSheet::parse(content, ParserOptions::default()) {
            Ok(stylesheet) => stylesheet,
            Err(e) => {
                warn!("CSS minification error: {}", e);
                return content.to_string();
            }
        };

        match stylesheet.minify(self.css_options.clone()) {
            Ok(minified) => minified.code,
            Err(e) => {
                warn!("CSS minification error: {}", e);
                content.to_string()
            }
        }
    }

    pub fn minify_js(&self, content: &str) -> String {
        let cm = SourceMap::default();
        let handler_opts = HandlerOpts {
            skip_filename_check: true,
            ..Default::default()
        };

        match try_with_handler(cm, handler_opts, |handler| {
            let program = swc::parse_js(
                handler,
                content,
                swc::config::ParseOptions::default(),
            )?;
            swc::minify(program, handler, self.js_options.clone())
        }) {
            Ok(minified) => minified,
            Err(e) => {
                warn!("JS minification error: {}", e);
                content.to_string()
            }
        }
    }
}