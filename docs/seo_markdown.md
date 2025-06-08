# SEO for Markdown Content

Eldroid SSG provides comprehensive SEO support for markdown content through YAML front matter. You can specify various SEO-related metadata in your markdown files.

## Basic Usage

Add SEO metadata to your markdown files using YAML front matter:

```markdown
---
title: Getting Started with Eldroid SSG
description: Learn how to create blazingly fast static sites with Eldroid SSG
author: Your Name
date: 2025-06-08
tags: [static-site, rust, tutorial]
keywords: [eldroid-ssg, static site generator, rust, tutorial, documentation]
canonical_url: https://example.com/blog/getting-started
image: /images/getting-started-banner.jpg
structured_data: |
  {
    "@context": "https://schema.org",
    "@type": "Article",
    "headline": "Getting Started with Eldroid SSG",
    "author": {
      "@type": "Person",
      "name": "Your Name"
    },
    "datePublished": "2025-06-08",
    "description": "Learn how to create blazingly fast static sites with Eldroid SSG"
  }
---

Your markdown content here...
```

## Available SEO Fields

- `title`: Page title (required)
- `description`: Meta description for search results
- `keywords`: Array of keywords for search engines
- `author`: Content author
- `date`: Publication date (required)
- `tags`: Array of content tags
- `canonical_url`: Canonical URL if content exists in multiple locations
- `image`: Featured image URL (used for social media cards)
- `structured_data`: JSON-LD structured data for rich search results

## Auto-Generated SEO Tags

Eldroid SSG automatically generates:

1. Meta Description
2. Open Graph tags
   - og:title
   - og:description
   - og:image
   - og:type
   - og:url
3. Twitter Card tags
   - twitter:card
   - twitter:title
   - twitter:description
   - twitter:image
4. Schema.org structured data
5. Canonical URLs

## Best Practices

1. Always provide a unique title and description
2. Use high-quality images (1200x630px minimum for social cards)
3. Include relevant keywords naturally
4. Provide structured data for rich search results
5. Use canonical URLs when content is duplicated
