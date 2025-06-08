---
title: Welcome to My Blog
author: Roy
date: 2025-06-08T10:00:00Z
tags:
  - welcome
  - first-post
description: Welcome to my new blog built with Eldroid SSG
---

# Welcome to My Blog

Welcome to my new blog! This site is built using Eldroid SSG, a modern static site generator written in Rust. In this first post, I'll share some thoughts about static site generators and why I chose to build one in Rust.

## Why Rust?

Rust is a systems programming language that guarantees memory safety and thread safety. This makes it an excellent choice for building high-performance tools like static site generators. Some key benefits include:

- Zero-cost abstractions
- Memory safety without garbage collection
- Fearless concurrency
- Pattern matching
- Package management with Cargo

## Features of This Blog

This blog includes several modern features:

1. Markdown support with frontmatter
2. Code syntax highlighting
3. Responsive design
4. Fast navigation
5. Automatic table of contents

### Code Example

Here's a simple Rust example:

```rust
fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2)
    }
}
```

## What's Next?

In upcoming posts, I'll be sharing:

- Deep dives into Rust programming
- Web development best practices
- Performance optimization tips
- Project updates and tutorials

Stay tuned!
