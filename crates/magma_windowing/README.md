# Magma-Windowing

Provides platform agnostic representations for windows and monitors for the [magma_api](https://crates.io/crates/magma_api).

## Features

- ecs representation for OS windows
- ecs representation for monitors
- window events

## Usage

**This crate should be used together with the rest of the [magma_api](https://crates.io/crates/magma_api).**

Add this to your `Cargo.toml`:

```toml
[dependencies]
magma_windowing = "0.1.0-alpha.3"
```

### Example (with magma_api)

```rust
use magma_api::App;
use magma_api::magma_windowing::WindowingModule;

fn main() {
    let mut app = App::new();
    // Add the module
    app.add_module(WindowingModule);
    // run the app
    app.run();
}
```

**Note that the `WindowingModule` alone only provides the interface for window management. The backend is implemented in [magma_winit](https://crates.io/crates/magma_winit).**

### Cargo Features

currently no features

## Disclaimer

This crate is *not* production ready.
