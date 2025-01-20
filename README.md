# bevy_easy_shared_library

Written initially for Bevy 0.14.2. future support planned for 0.15.

The bevy_easy suite of plugins needs a shared library to operate effectively.
Import the library in your cargo.toml

Example: [cargo.toml]
```toml 
[package]
name = "your_project_here"
version = "0.1.0"
edition = "2021"

[dependencies]
bevy_easy_shared_library = "0.1.0"
```

Generally this crate will be installed via inheritance.