# Component System

## Core Concepts

### Component Syntax
```html
<el-component c_name="my_component" param1="foo" param2="bar" />
```

### Key Features
- **Auto-registration:** Components in `components/` directory are auto-registered
- **Nested Components:** Components can include other components
- **Circular Detection:** Prevents infinite recursion
- **Parameter Validation:** Type checking and required params
- **Inheritance:** Components can extend other components
- **Dynamic Loading:** Support for lazy-loaded components

## Component Lifecycle

### 1. Registration Phase
1. **Discovery**
   - Scan components directory
   - Parse component files
   - Register available components

2. **Validation**
   - Check for naming conflicts
   - Validate parameter types
   - Build dependency graph

3. **Optimization**
   - Cache component templates
   - Pre-process static content
   - Optimize nested structures

### 2. Build Phase
1. **Parameter Resolution**
   - Process input parameters
   - Apply default values
   - Validate required fields

2. **Content Processing**
   - Parse component content
   - Process child components
   - Apply transformations

3. **Output Generation**
   - Render final HTML
   - Apply optimizations
   - Generate source maps

## Advanced Features

### 1. Dynamic Components
Create components that load conditionally:
```html
<el-component 
    c_name="@{content_type}_view"
    data="@{content_data}" 
/>
```

### 2. Component Inheritance
Extend existing components:
```html
<!-- components/base_card.html -->
<div class="card">
    <div class="card-header">@{yield header}</div>
    <div class="card-body">@{yield}</div>
    <div class="card-footer">@{yield footer}</div>
</div>

<!-- components/feature_card.html -->
<el-component c_name="base_card">
    <header>@{title}</header>
    <main>@{description}</main>
    <footer>@{call_to_action}</footer>
</el-component>
```

### 3. Slot System
Multiple content slots in components:
```html
<!-- components/layout.html -->
<div class="layout">
    <header>@{yield header}</header>
    <nav>@{yield navigation}</nav>
    <main>@{yield}</main>
    <aside>@{yield sidebar}</aside>
    <footer>@{yield footer}</footer>
</div>

<!-- Usage -->
<el-component c_name="layout">
    <header>Site Header</header>
    <navigation>
        <el-component c_name="nav_menu" />
    </navigation>
    <main>Page Content</main>
    <sidebar>
        <el-component c_name="widget_area" />
    </sidebar>
    <footer>Site Footer</footer>
</el-component>
```

### 4. Event Handling
Components can handle build-time events:
```html
<!-- components/analytics.html -->
<script>
@{on_render()}
window.dataLayer = window.dataLayer || [];
dataLayer.push({
    'page': '@{current_page}',
    'template': '@{template_name}'
});
</script>
```

### 5. Conditional Rendering
Control component output based on conditions:
```html
<el-component c_name="feature_flag"
    name="dark_mode"
    enabled="@{is_dark_mode}"
>
    <el-component c_name="dark_theme" />
</el-component>
```

## Component Patterns

### 1. Higher-Order Components
Create components that modify other components:
```html
<!-- components/with_theme.html -->
<div class="theme-@{theme}">
    @{yield}
</div>

<!-- Usage -->
<el-component c_name="with_theme" theme="dark">
    <el-component c_name="user_profile" />
</el-component>
```

### 2. Component Composition
Combine multiple components:
```html
<!-- components/blog_post.html -->
<article>
    <el-component c_name="post_header" 
        title="@{title}" 
        date="@{date}" 
    />
    <el-component c_name="post_content">
        @{yield}
    </el-component>
    <el-component c_name="post_footer" 
        tags="@{tags}" 
    />
</article>
```

### 3. Component Libraries
Create reusable component collections:
```html
<!-- components/ui/button.html -->
<button class="btn btn-@{variant}">
    @{yield}
</button>

<!-- components/ui/card.html -->
<div class="card">
    @{yield}
</div>

<!-- Usage -->
<el-component c_name="ui/button" variant="primary">
    Click Me
</el-component>
```

## Performance Optimization

### 1. Component Caching
Cache expensive components:
```html
<el-component c_name="cached_feed"
    cache_key="@{feed_url}"
    max_age="3600"
/>
```

### 2. Lazy Loading
Load components on demand:
```html
<el-component c_name="lazy_component"
    path="heavy_component"
    fallback="Loading..."
/>
```

### 3. Code Splitting
Split components into chunks:
```html
<el-component c_name="chunked_content"
    chunk_name="blog_posts"
    async="true"
/>
```

## Development Tools

### 1. Component Inspector
Debug component hierarchy:
```bash
eldroid-ssg --inspect-component my_component
```

### 2. Performance Profiling
Profile component rendering:
```bash
eldroid-ssg --profile-components
```

### 3. Hot Reloading
Enable component hot reload:
```bash
eldroid-ssg --watch --hot-components
```

## Best Practices

### 1. Component Organization
- Use consistent naming conventions
- Group related components
- Create component documentation
- Maintain a component library

### 2. Performance
- Minimize component nesting
- Use lazy loading for heavy components
- Implement proper caching
- Profile component performance

### 3. Maintainability
- Keep components focused
- Document component APIs
- Use TypeScript for type safety
- Write component tests
