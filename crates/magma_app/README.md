# magma_app

This crate expands the capabilities of [magma_ecs](https://codeberg.org/DynamicGoose/magma-ecs) and forms the base of the [magma_api](https://codeberg.org/DynamicGoose/magma-api).

## Features

- An app struct, which wraps a `magma_ecs::World` and adds more functionality.
- Adding custom modules (not rust modules) to an app.
- An `update` and `run` method with the ability to specify a custom runner.

## Usage

**This crate can be used standalone, but it is recommended to use it as part of the [magma_api](https://crates.io/crates/magma_api).**

Add this to your `Cargo.toml`:

```toml
[dependencies]
magma_app = "0.2.0-alpha.4"
```

### Entity-Component-System (Magma-ECS)

**Entity:** An *entity* is just an index into the component storage.  
**Component:** A *component* holds some type of data. Entities can have components assigned to them.  
**System:** A *system* is a piece of code (usually a function), that reads and modifies the data.  

Another way to think about this would be *Identifier-Data-Logic*.

### Example

```rust
use magma_app::App;

fn main() {
    let mut app = App::new();
    // run the app
    app.run();
}
```

### Cargo Features

currently no features

## Disclaimer

This is *not* production ready.
