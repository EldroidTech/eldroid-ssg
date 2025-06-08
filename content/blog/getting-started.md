---
title: Getting Started with Eldroid SSG
description: Learn how to build lightning-fast static websites using Eldroid SSG, a modern static site generator written in Rust. This comprehensive guide covers installation, basic usage, and best practices.
author: Roy
date: 2025-06-08T11:00:00Z
tags:
  - tutorial
  - eldroid-ssg
  - static-site
keywords:
  - eldroid-ssg
  - static site generator
  - rust
  - tutorial
  - documentation
  - quick start
canonical_url: https://eldroid-ssg.dev/blog/getting-started
image: /images/getting-started-hero.jpg
structured_data: |
  {
    "@context": "https://schema.org",
    "@type": "TechArticle",
    "headline": "Getting Started with Eldroid SSG",
    "description": "Learn how to build lightning-fast static websites using Eldroid SSG, a modern static site generator written in Rust.",
    "author": {
      "@type": "Person",
      "name": "Roy"
    },
    "datePublished": "2025-06-08T11:00:00Z",
    "image": "/images/getting-started-hero.jpg",
    "publisher": {
      "@type": "Organization",
      "name": "Eldroid SSG",
      "logo": {
        "@type": "ImageObject",
        "url": "/images/eldroid-logo.png"
      }
    }
  }
---

# Getting Started with Eldroid SSG

In this post, I'll walk you through creating your first website with Eldroid SSG. You'll learn about the basic concepts and how to set up your development environment.

## Prerequisites

Before we begin, make sure you have:

- Rust installed (1.75 or later)
- Basic knowledge of HTML and Markdown
- A code editor (VS Code recommended)

## Installation

Install Eldroid SSG using Cargo:

```bash
cargo install eldroid-ssg
```

## Project Structure

Create a new project:

```bash
mkdir my-site
cd my-site

# Create required directories
mkdir -p content/blog components static
```

Your project structure should look like this:

```
my-site/
├── content/          # Content files (.html, .md)
│   ├── index.html    # Site homepage
│   └── blog/         # Blog posts
├── components/       # Reusable components
│   └── blog_layout.html
└── static/          # Static assets
    ├── css/
    ├── js/
    └── images/
```

## Next Steps

In the next tutorial, we'll cover:

- Creating custom components
- Styling your site
- Adding advanced features
- Deploying to production
