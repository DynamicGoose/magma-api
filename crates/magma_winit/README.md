# magma_winit

This crate provides backends for [magma_windowing](https://crates.io/crates/magma_windowing) as well as [magma_input](https://crates.io/crates/magma_input).

## Features

- windowing backend
- input backend

## Usage

**This crate should be used together with the rest of the [magma_api](https://crates.io/crates/magma_api).**

Add this to your `Cargo.toml`:

```toml
[dependencies]
magma_winit = "0.1.0-alpha.9"
```

### Example (with magma_api)

```rust
use magma_api::App;
use magma_api::magma_winit::WinitModule;

fn main() {
    let mut app = App::new();
    // Add the module (also adds InputModule and WindowingModule automatically)
    app.add_module(WinitModule);
    // run the app
    app.run();
}
```

### Cargo Features

currently no features

## Disclaimer

This crate is *not* production ready.
