---
layout: post
title: "Understanding Stack Memory Layout in Rust"
date: 2024-12-28
categories: rust memory stack
---

# Stack Memory Layout

In this post, we'll explore how Rust lays out variables in stack memory.

## Alignment and Padding

When you declare variables like this:
```rust
let a: u8 = 1;
let b: u64 = 2;
let c: u32 = 3;
```

The memory layout is NOT sequential! Here's why...

## Debugging with LLDB

To see the actual addresses:
```bash
lldb target/debug/rust-memory-lab
(lldb) b stack_layout.rs:2
(lldb) run
(lldb) frame variable -L
```

(Add your content here - copy from your existing markdown files)

