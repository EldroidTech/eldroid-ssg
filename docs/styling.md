# Styling Guide

This guide explains how to customize the appearance of your Eldroid SSG site, particularly for markdown content.

## Default Theme

Eldroid SSG comes with a built-in responsive theme that supports both light and dark modes. The theme automatically switches based on the user's system preferences.

## Customizing Styles

### Theme Colors

You can customize the theme colors by overriding CSS variables in your own stylesheet:

```css
:root {
    /* Light theme variables */
    --md-bg: #ffffff;
    --md-text: #2c3e50;
    --md-title: #1a202c;
    --md-link: #3182ce;
    --md-link-hover: #2c5282;
    --md-border: #e2e8f0;
    --md-code-bg: #f7fafc;
    --md-code-text: #805ad5;
    --md-blockquote-bg: #f8f9fa;
    --md-blockquote-border: #cbd5e0;
    --md-table-border: #e2e8f0;
    --md-table-stripe: #f7fafc;
    --md-inline-code-bg: #edf2f7;
}

@media (prefers-color-scheme: dark) {
    :root {
        /* Dark theme variables */
        --md-bg: #1a202c;
        --md-text: #e2e8f0;
        --md-title: #f7fafc;
        --md-link: #63b3ed;
        --md-link-hover: #90cdf4;
        --md-border: #2d3748;
        --md-code-bg: #2d3748;
        --md-code-text: #b794f4;
        --md-blockquote-bg: #2d3748;
        --md-blockquote-border: #4a5568;
        --md-table-border: #2d3748;
        --md-table-stripe: #2d3748;
        --md-inline-code-bg: #2d3748;
    }
}
```

### Typography

To customize the typography, you can override these styles:

```css
.markdown-content {
    /* Base text styles */
    font-family: your-preferred-font;
    font-size: 16px;
    line-height: 1.7;
}

.markdown-content h1 {
    font-size: 2.25em;
    font-weight: 700;
}

.markdown-content h2 {
    font-size: 1.8em;
    font-weight: 600;
}

/* ... and so on for h3-h6 */
```

### Code Blocks

Customize code block appearance:

```css
.markdown-content pre {
    /* Code block container */
    padding: 1.5em;
    border-radius: 8px;
    background: var(--md-code-bg);
}

.markdown-content code {
    /* Inline code */
    font-family: 'Your Preferred Mono Font', monospace;
    font-size: 0.9em;
}
```

### Adding Custom Components

1. Create your component HTML file in the `components/` directory
2. Style it using CSS variables for theme consistency
3. Use it in your markdown with the component syntax:

```markdown
<MyCustomComponent prop="value">
  Content here
</MyCustomComponent>
```

### Responsive Design

The default theme is mobile-friendly, but you can customize breakpoints:

```css
/* Tablet */
@media (max-width: 768px) {
    .markdown-content {
        font-size: 15px;
    }
}

/* Mobile */
@media (max-width: 480px) {
    .markdown-content {
        font-size: 14px;
    }
}
```

### Advanced Customization

#### Custom Syntax Highlighting

To use a different syntax highlighting theme:

1. Create a new CSS file in `static/css/`
2. Import it in your layout after the default styles
3. Override the code block styles

#### Custom Fonts

To use custom fonts:

1. Add font files to `static/fonts/`
2. Define the font faces:

```css
@font-face {
    font-family: 'YourCustomFont';
    src: url('/fonts/your-font.woff2') format('woff2');
    font-weight: 400;
    font-style: normal;
    font-display: swap;
}
```

3. Use the font in your styles:

```css
.markdown-content {
    font-family: 'YourCustomFont', sans-serif;
}
```

## Best Practices

1. Use CSS variables for colors to maintain theme consistency
2. Keep responsive design in mind
3. Optimize web fonts for performance
4. Test both light and dark themes
5. Ensure sufficient color contrast for accessibility

## Example: Custom Theme

Here's a complete example of a custom theme:

```css
/* In static/css/custom-theme.css */
:root {
    /* Custom light theme */
    --md-bg: #fdfcf7;
    --md-text: #2d3748;
    --md-title: #1a202c;
    --md-link: #2b6cb0;
    --md-code-bg: #f7fafc;
}

@media (prefers-color-scheme: dark) {
    :root {
        /* Custom dark theme */
        --md-bg: #1a1a1a;
        --md-text: #e2e8f0;
        --md-title: #f7fafc;
        --md-link: #63b3ed;
        --md-code-bg: #2d3748;
    }
}

.markdown-content {
    /* Custom typography */
    font-family: 'Inter', sans-serif;
    font-size: 16px;
    line-height: 1.8;
}

/* Custom heading styles */
.markdown-content h1 {
    font-size: 2.5em;
    letter-spacing: -0.03em;
}

/* Custom link styles */
.markdown-content a {
    text-decoration: none;
    border-bottom: 1px solid var(--md-link);
}

.markdown-content a:hover {
    border-bottom-width: 2px;
}
```
