# Global Macros Guide

## Overview
Global macros in Eldroid SSG provide a powerful way to inject dynamic content and logic into your static site. They can be used in both content files and components.

## Basic Syntax

### Macro Definition
```rust
// macros.ed
@macro current_year() {
    @{DateTime.now().year}
}

@macro site_info(name, description = "A static site") {
    <div class="site-info">
        <h1>@{name}</h1>
        <p>@{description}</p>
    </div>
}
```

### Usage in Content
```html
<footer>
    Copyright © @{current_year()} My Site
    @{site_info("My Blog", "A blog about coding")}
</footer>
```

## Built-in Macros

### Date and Time
```html
<!-- Current date in various formats -->
@{date("YYYY-MM-DD")}
@{date("MMM Do, YYYY")}
@{time("HH:mm:ss")}
@{datetime("YYYY-MM-DD HH:mm")}
```

### Path and URL
```html
<!-- URL and path manipulation -->
@{base_url()}
@{relative_path("images/logo.png")}
@{absolute_url("/blog/post-1")}
```

### Environment
```html
<!-- Environment variables and build info -->
@{env("NODE_ENV")}
@{build_time()}
@{git_commit()}
```

## Advanced Features

### Conditional Logic
```rust
@macro feature_flag(name) {
    @if env("ENABLE_" + name) == "true" {
        <div class="feature @{name}">
            @{yield}
        </div>
    }
}
```

### Loops and Iteration
```rust
@macro repeat(times) {
    @for i in 0..times {
        <div class="item-@{i}">
            @{yield}
        </div>
    }
}
```

### Template Composition
```rust
@macro layout(title) {
    <!DOCTYPE html>
    <html>
        <head>
            <title>@{title} | @{site_name()}</title>
            @{yield head}
        </head>
        <body>
            <header>@{yield nav}</header>
            <main>@{yield}</main>
            <footer>@{yield footer}</footer>
        </body>
    </html>
}
```

### Data Integration
```rust
@macro blog_posts() {
    @{
        let posts = load_json("data/posts.json");
        posts.map(|post| {
            <article>
                <h2>@{post.title}</h2>
                <p>@{post.excerpt}</p>
            </article>
        }).join("\n")
    }
}
```

## Performance Optimization

### Caching
Macros can be cached based on their inputs:
```rust
@macro cached_content(key) {
    @cache(key) {
        <!-- expensive operation -->
        @{fetch_remote_data()}
    }
}
```

### Lazy Evaluation
```rust
@macro lazy_load(component) {
    <div data-lazy="@{component}">
        Loading...
    </div>
}
```

## Best Practices

### 1. Modularity
- Keep macros focused and single-purpose
- Break complex macros into smaller ones
- Use composition for complex layouts

### 2. Performance
- Cache expensive operations
- Use lazy evaluation for heavy content
- Minimize runtime computations

### 3. Maintainability
- Document macro parameters
- Use meaningful names
- Keep logic simple and readable

### 4. Error Handling
```rust
@macro safe_content(url) {
    @try {
        @{fetch_content(url)}
    } @catch(e) {
        <div class="error">Content unavailable</div>
    }
}
```

## Debug Tools

### Macro Inspection
```bash
eldroid-ssg --inspect-macro my_macro
```

### Performance Profiling
```bash
eldroid-ssg --profile-macros
```

This will show:
- Macro execution times
- Cache hit rates
- Memory usage
- Call stack traces

## Common Patterns

### 1. Asset Management
```rust
@macro asset(path) {
    @{
        let hash = file_hash(path);
        "/assets/@{path}?v=@{hash}"
    }
}
```

### 2. Internationalization
```rust
@macro t(key, locale = "en") {
    @{translations[locale][key] || key}
}
```

### 3. Component Wrappers
```rust
@macro with_analytics(id) {
    @{yield}
    <script>
        track("@{id}");
    </script>
}
```

### 4. Dynamic Content
```rust
@macro dynamic_content(type) {
    @match type {
        "latest" => @{recent_posts(5)},
        "featured" => @{featured_content()},
        _ => @{default_content()}
    }
}
```
