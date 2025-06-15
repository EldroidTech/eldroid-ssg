use minify_html::minify as minify_html_content;
use lightningcss::{
    stylesheet::{MinifyOptions, ParserOptions, StyleSheet},
    targets::Browsers,
};
use log::warn;

pub struct Minifier {
    html_config: minify_html::Cfg,
    css_options: MinifyOptions,
}

impl Default for Minifier {
    fn default() -> Self {
        Self {
            html_config: minify_html::Cfg {
                minify_doctype: false, // replaces do_not_minify_doctype: true
                allow_noncompliant_unquoted_attribute_values: false, // replaces ensure_spec_compliant_unquoted_attribute_values: true
                allow_removing_spaces_between_attributes: false, // replaces keep_spaces_between_attributes: true
                allow_optimal_entities: false, // optional, default is fine
                keep_closing_tags: true,
                keep_html_and_head_opening_tags: true,
                keep_comments: false,
                keep_ssi_comments: false,
                minify_css: true,
                minify_js: true,
                preserve_brace_template_syntax: false,
                preserve_chevron_percent_template_syntax: false,
                remove_bangs: false,
                remove_processing_instructions: true,
                keep_input_type_text_attr: false,
            },
            css_options: MinifyOptions {
                targets: Browsers::default().into(),
                ..MinifyOptions::default()
            },
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
        let mut stylesheet = match StyleSheet::parse(content, ParserOptions::default()) {
            Ok(stylesheet) => stylesheet,
            Err(e) => {
                warn!("CSS minification error: {}", e);
                return content.to_string();
            }
        };

        match stylesheet.minify(MinifyOptions {
            targets: self.css_options.targets.clone(),
            ..MinifyOptions::default()
        }) {
            Ok(_) => stylesheet.to_css(Default::default())
                .map(|out| out.code)
                .unwrap_or_else(|e| {
                    warn!("CSS serialization error: {}", e);
                    content.to_string()
                }),
            Err(e) => {
                warn!("CSS minification error: {}", e);
                content.to_string()
            }
        }
    }

    pub fn minify_js(&self, content: &str) -> String {
        // For now, return unminified content since we removed swc
        // TODO: Implement JS minification using lightningcss or another library
        content.to_string()
    }
}