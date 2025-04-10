# magma_app

This crate expands the capabilities of [magma_ecs]() and forms the base of the [magma_api]().

## Features

- An app struct, which wraps a `magma_ecs::World` and adds more functionality.
- Adding custom modules (not rust modules) to an app.
- An `update` and `run` method with the ability to specify a custom runner.

## Usage

**This crate can be used standalone, but it is recommended to use it as part of the [magma_api]().**

Add this to your `Cargo.toml`:

```toml
[dependencies]
magma_app = "0.1.0-beta"
```

## Disclaimer

This is *not* production ready.
