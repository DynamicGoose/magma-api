# Magma-API

Magma-API contains all the engine functionality for Magma3D. It is based on Magma-ECS.
  
You will be able to use this in combination with the upcoming graphical editor for Magma3D, but it can be used standalone without any sacrifices.

## Features

- **magma_app:** Core functionality
    - [x] Expands [magma-ecs](https://codeberg.org/DynamicGoose/magma-ecs)
    - [x] Creating apps with a built-in update loop
    - [x] Add custom modules to apps to expand functionality
    - [x] A powerfull event system which let's you define custom events
- **magma_window:** Backend-agnostic windowing system
    - [x] The `WindowModule` adds respective events and components to the app
    - [x] Handle window events
    - [x] Add `Window` components to entities to easily create windows
- **magma_input:** Backend-agnostic input system
    - [ ] Mouse & keyboard input support
    - [ ] Gamepad & controller support
    - [ ] VR input support
- **magma_winit:** Winit integration for magma_window and magma_input
    - [x] backend for magma_window
    - [ ] backend for magma_input
- **magma_render:** Rendering system
    - [x] [FeuFeu](https://codeberg.org/DynamicGoose/feufeu) rendering library based on [wgpu](https://wgpu.rs/)
    - [ ] Vertex/Voxel hybrid renderer
- **magma_asset:** Asset loading
    - [ ] load glTF models
    - [ ] images
    - [ ] audio files
- **magma_math:** Usefull math utilities
    - [x] reexports [glam](https://crates.io/crates/glam)
    - [ ] other useful math
- **magma_ui:** UI management
    - [ ] [iced](https://iced.rs/) integration
- **magma_physics:** Powerful physics engine
- **magma_audio:** Physically based audio
- **magma_scene:** Loading and saving state

## Usage

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
magma_api = "0.2.0-alpha"
```

### Entity-Component-System (Magma-ECS)

**Entity:** An *entity* is just an index into the component storage.  
**Component:** A *component* holds some type of data. Entities can have components assigned to them.  
**System:** A *system* is a piece of code (usually a function), that reads and modifies the data.  

Another way to think about this would be *Identifier-Data-Logic*.

### Example

```rust
use magma_api::App;
use magma_api::DefaultModules;

fn main() {
    let mut app = App::new();

    // add default functionality like windowing and rendering
    app.add_module(DefaultModules);
    // run the app
    app.run();
}
```

### Cargo Features

currently no features

## Disclaimer

This is still in developement and *not* production ready.
