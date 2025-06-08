use std::path::{Path, PathBuf};
use anyhow::{Result, anyhow};
use chrono::DateTime;
use chrono_humanize::HumanTime;
use pulldown_cmark::{Parser, html, Options, Event, Tag, TagEnd, CodeBlockKind};
use serde::{Serialize, Deserialize};
use yaml_front_matter::{YamlFrontMatter};
use crate::variables::Variables;
use std::fs;
use std::collections::HashMap;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;
use syntect::html::highlighted_html_for_string;
use html_escape;
use lazy_static::lazy_static;

#[derive(Debug, Serialize, Deserialize)]
pub struct BlogFrontMatter {
    pub title: String,
    #[serde(default)]
    pub author: Option<String>,
    pub date: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub keywords: Option<Vec<String>>,
    #[serde(default)]
    pub canonical_url: Option<String>,
    #[serde(default)]
    pub structured_data: Option<String>,
    #[serde(default)]
    pub image: Option<String>, // For og:image and twitter:image
}

#[derive(Debug)]
pub struct BlogPost {
    pub front_matter: BlogFrontMatter,
    pub content: String,
    pub html_content: String,
    pub url: String,
    pub file_path: PathBuf,
}

impl BlogPost {
    pub fn from_file(file_path: &Path, content_dir: &Path) -> Result<Self> {
        let content = fs::read_to_string(file_path)?;
        let yaml_content = YamlFrontMatter::parse::<BlogFrontMatter>(&content)
            .map_err(|e| anyhow!("Failed to parse front matter: {}", e))?;

        let markdown_content = yaml_content.content;
        let html_content = markdown_to_html(&markdown_content);
        
        // Generate URL from file path
        let url = file_path.strip_prefix(content_dir)?
            .with_extension("")
            .to_string_lossy()
            .to_string();

        Ok(BlogPost {
            front_matter: yaml_content.metadata,
            content: markdown_content,
            html_content,
            url: format!("/{}", url),
            file_path: file_path.to_path_buf(),
        })
    }

    pub fn formatted_date(&self) -> Result<String> {
        let date = DateTime::parse_from_rfc3339(&self.front_matter.date)
            .map_err(|e| anyhow!("Invalid date format: {}", e))?;
        let human_time = HumanTime::from(date);
        Ok(human_time.to_string())
    }

    pub fn generate_json_ld(&self, site_name: &str, base_url: &str) -> Result<String> {
        let mut json_ld = serde_json::json!({
            "@context": "https://schema.org",
            "@type": "Article",
            "headline": self.front_matter.title,
            "datePublished": self.front_matter.date,
            "dateModified": self.front_matter.date,
            "url": format!("{}{}", base_url.trim_end_matches('/'), self.url),
            "publisher": {
                "@type": "Organization",
                "name": site_name,
                "url": base_url
            }
        });

        // Add optional fields if they exist
        if let Some(description) = &self.front_matter.description {
            json_ld.as_object_mut().unwrap()
                .insert("description".to_string(), serde_json::Value::String(description.clone()));
        }

        if let Some(author) = &self.front_matter.author {
            json_ld.as_object_mut().unwrap()
                .insert("author".to_string(), serde_json::json!({
                    "@type": "Person",
                    "name": author
                }));
        }

        if let Some(image) = &self.front_matter.image {
            json_ld.as_object_mut().unwrap()
                .insert("image".to_string(), serde_json::json!([
                    format!("{}{}", base_url.trim_end_matches('/'), image)
                ]));
        }

        // Add keywords if available
        if let Some(keywords) = &self.front_matter.keywords {
            json_ld.as_object_mut().unwrap()
                .insert("keywords".to_string(), serde_json::Value::String(keywords.join(", ")));
        }

        // Generate article body from markdown content
        let text_content = html2text::from_read(self.html_content.as_bytes(), 80);
        json_ld.as_object_mut().unwrap()
            .insert("articleBody".to_string(), serde_json::Value::String(text_content));

        Ok(serde_json::to_string_pretty(&json_ld)?)
    }
}

pub fn markdown_to_html(content: &str) -> String {
    lazy_static! {
        static ref SYNTAX_SET: SyntaxSet = SyntaxSet::load_defaults_newlines();
        static ref THEME_SET: ThemeSet = ThemeSet::load_defaults();
    }
    
    let theme = &THEME_SET.themes["base16-ocean.dark"];
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    
    let mut html_output = String::new();
    let parser = Parser::new_ext(content, options);
    
    let mut in_code_block = false;
    let mut code_content = String::new();
    let mut code_lang = String::new();
    
    for event in parser {
        match event {
            // If we find a code block, apply syntax highlighting
            Event::Code(code) => {
                let escaped = html_escape::encode_text(&code);
                html_output.push_str(&format!("<code>{}</code>", escaped));
            },
            Event::Start(Tag::CodeBlock(kind)) => {
                in_code_block = true;
                code_content.clear();
                code_lang = match kind {
                    CodeBlockKind::Fenced(lang) => lang.to_string(),
                    CodeBlockKind::Indented => String::from("text"),
                };
            },
            Event::End(TagEnd::CodeBlock) => {
                in_code_block = false;
                
                let syntax = SYNTAX_SET.find_syntax_by_token(&code_lang)
                    .or_else(|| SYNTAX_SET.find_syntax_by_extension(&code_lang))
                    .unwrap_or_else(|| SYNTAX_SET.find_syntax_plain_text());
                
                // Apply syntax highlighting
                let html = highlighted_html_for_string(&code_content, &SYNTAX_SET, syntax, theme)
                    .unwrap_or_else(|_| html_escape::encode_text(&code_content).to_string());
                
                html_output.push_str(&format!("<pre><code class=\"language-{}\">{}</code></pre>", 
                    code_lang,
                    html
                ));
            },
            Event::Text(text) => {
                if in_code_block {
                    code_content.push_str(&text);
                } else {
                    html::push_html(&mut html_output, std::iter::once(Event::Text(text)));
                }
            },
            // For all other markdown elements, just convert to HTML
            _ => {
                if !in_code_block {
                    html::push_html(&mut html_output, std::iter::once(event));
                }
            }
        }
    }
    
    html_output
}

pub struct BlogProcessor {
    posts: Vec<BlogPost>,
    content_dir: PathBuf,
}

impl BlogProcessor {
    pub fn new(content_dir: PathBuf) -> Self {
        Self {
            posts: Vec::new(),
            content_dir,
        }
    }

    pub fn with_option_components(content_dir: PathBuf, _vars: Option<Variables>) -> Self {
        Self {
            posts: Vec::new(),
            content_dir,
        }
    }

    pub fn load_posts(&mut self) -> Result<()> {
        self.posts.clear();
        let blog_dir = self.content_dir.join("blog");
        
        if !blog_dir.exists() {
            return Ok(());
        }

        for entry in fs::read_dir(blog_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.extension().map_or(false, |ext| ext == "md") {
                match BlogPost::from_file(&path, &self.content_dir) {
                    Ok(post) => self.posts.push(post),
                    Err(e) => log::warn!("Failed to load blog post {}: {}", path.display(), e),
                }
            }
        }

        // Sort posts by date, newest first
        self.posts.sort_by(|a, b| {
            b.front_matter.date.cmp(&a.front_matter.date)
        });

        Ok(())
    }

    pub fn process_post(&self, post: &BlogPost) -> Result<String> {
        // Find prev/next posts
        let post_idx = self.posts.iter().position(|p| p.url == post.url);
        let prev_post = post_idx.and_then(|idx| self.posts.get(idx + 1));
        let next_post = post_idx.and_then(|idx| idx.checked_sub(1).and_then(|i| self.posts.get(i)));

        // Set up variables for the blog template
        let mut variables = HashMap::new();
        variables.insert("title".to_string(), post.front_matter.title.clone());
        variables.insert("date".to_string(), post.formatted_date()?);

        // SEO metadata
        let mut seo_comment = format!(
            "<!-- SEO {{\n  \"title\": \"{}\",\n", 
            &post.front_matter.title
        );

        if let Some(desc) = &post.front_matter.description {
            variables.insert("description".to_string(), desc.clone());
            seo_comment.push_str(&format!("  \"description\": \"{}\",\n", desc));
        }

        if let Some(keywords) = &post.front_matter.keywords {
            seo_comment.push_str(&format!("  \"keywords\": {},\n", serde_json::to_string(keywords)?));
        }

        seo_comment.push_str(&format!("  \"url\": \"{}\",\n", &post.url));

        if let Some(canonical) = &post.front_matter.canonical_url {
            seo_comment.push_str(&format!("  \"canonical_url\": \"{}\",\n", canonical));
        }

        // Generate JSON-LD if not provided in front matter
        let structured_data = match &post.front_matter.structured_data {
            Some(data) => data.clone(),
            None => post.generate_json_ld("Eldroid SSG", "https://eldroid-ssg.dev")?
        };
        seo_comment.push_str(&format!("  \"structured_data\": {},\n", structured_data));

        if let Some(image) = &post.front_matter.image {
            variables.insert("og_image".to_string(), image.clone());
        }
        
        if let Some(author) = &post.front_matter.author {
            variables.insert("author".to_string(), author.clone());
        }

        seo_comment.push_str("}} -->\n");
        variables.insert("seo_meta".to_string(), seo_comment);
        variables.insert("json_ld".to_string(), structured_data);
        
        // Navigation
        if let Some(prev) = prev_post {
            variables.insert("prev_post.url".to_string(), prev.url.clone());
            variables.insert("prev_post.title".to_string(), prev.front_matter.title.clone());
        }

        if let Some(next) = next_post {
            variables.insert("next_post.url".to_string(), next.url.clone());
            variables.insert("next_post.title".to_string(), next.front_matter.title.clone());
        }

        variables.insert("navigation_tree".to_string(), self.generate_navigation_tree());
        variables.insert("site_title".to_string(), "Blog".to_string());

        // Generate final HTML using the blog layout
        let blog_layout = fs::read_to_string(self.content_dir.parent().unwrap().join("components/blog_layout.html"))?;
        
        // Inject the post content and variables into the template
        let mut content = blog_layout.replace("@{yield}", &post.html_content);

        // Process variables
        for (key, value) in variables {
            content = content.replace(&format!("@{{{}}}",key), &value);
        }

        Ok(content)
    }

    pub fn generate_navigation_tree(&self) -> String {
        let mut html = String::from("<ul class=\"nav-tree\">");
        
        for post in &self.posts {
            html.push_str(&format!(
                "<li><a href=\"{}\">{}</a></li>",
                post.url,
                post.front_matter.title
            ));
        }
        
        html.push_str("</ul>");
        html
    }
}
