//! The Magma-API crate is a container crate to combine all the featrues that are in seperate crates.
pub use magma_app;
pub use magma_audio;
pub use magma_ui;
pub use magma_winit;

#[cfg(test)]
mod tests {}
