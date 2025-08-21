# magma_input

This crate handles user input for the Magma3D engine.

## Features

- [x] Keyboard & Mouse
- [ ] Gamepad
- [ ] Touch
- [ ] VR Controls

## Usage

**This crate should be used together with the rest of the [magma_api](https://crates.io/crates/magma_api).**

Add this to your `Cargo.toml`:

```toml
[dependencies]
magma_input = "0.1.0-alpha.2"
```

### Example (with magma_api)

```rust
use magma_api::App;
use magma_api::magma_input::InputModule;

fn main() {
    let mut app = App::new();
    // Add the input module
    app.add_module(InputModule);
    // run the app
    app.run();
}
```

**Note that the `InputModule` alone only provides the interface for input management. The backend is implemented in [magma_winit](https://crates.io/crates/magma_winit).**

### Cargo Features

currently no features

## Disclaimer

This crate is *not* production ready.
