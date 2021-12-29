Modular Development Guide
====

## Motivation

This is a research project that how to develop a program which could inter-connect to different environments with a small impact.

When the inter-connect code is small enough, we could switch code only with flag and macro.  
However, when the code is too big to switch by flag or macro, the code should be packaged as separated packages.

## Basic Concepts

* Use branch `env_*` for minimal change
  * Changing branch `env_*` changes `cargo.toml`
    * For local development, a `cargo.toml` of some branch directs plugin cargo not by cargo repo but by local path
  * Branches
    * `env_a`: Use `*_plugin_a` at cargo repo
    * `env_a_path`: Use `*_plugin_a` at local
    * `env_b`: Use `*_plugin_b` at cargo repo
    * `env_b_path`: Use `*_plugin_b` at local
  * Changing branch `env_*` does not change source code
  * Import and re-export different package as same name
    * Re-exporting module should be wrapper
    * The flag selects each plug-in
* Use package(crate) for big change
  * Import different **plug-in** package

### How to import these packages

Select plug-in of outer package by

```toml
[features]
default = []
the_flag = [ "the_outer_package/the_internal_flag", ]

[dependencies]
the_outer_package = "0.1"
```
