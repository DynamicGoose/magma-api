use std::num::NonZero;

use magma_math::{IVec2, UVec2};

pub struct Window {
    pub title: String,
    pub name: Option<String>,
    pub position: WindowPosition,
    pub resolution: WindowResolution,
    pub resizable: bool,
    pub resize_limit: WindowResizeLimit,
    pub mode: WindowMode,
    pub cursor_mode: CursorMode,
    pub cursor_visible: bool,
    pub decorations: bool,
    pub titlebar_buttons: TitlebarButtons,
    pub present_mode: PresentMode,
    pub alpha_mode: AlphaMode,
    pub transparent: bool,
    pub focused: bool,
    pub no_default_event_handling: bool,
    pub window_theme: WindowTheme,
    pub desired_maximum_frame_latency: Option<NonZero<u32>>, // wgpu::SurfaceConfiguration::desired_maximum_frame_latency
    pub pinch_gesture: bool,
    pub rotation_gesture: bool,
    pub doubletap_gesture: bool,
    pub pan_gesture: bool,

    /// true if the backend has created a window for this component
    pub has_window: bool,
}

// TODO: Emit WindowAttributesChange event, when things are modified.
impl Window {
    pub const fn get_title(&self) -> &String {
        &self.title
    }

    pub const fn get_title_mut(&mut self) -> &mut String {
        &mut self.title
    }

    pub fn set_title(&mut self, title: &str) {
        self.title = title.to_owned();
    }

    pub const fn name(&self) -> &Option<String> {
        &self.name
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = Some(name.to_owned());
    }
}

#[derive(Clone, Copy, Default)]
pub enum WindowPosition {
    #[default]
    Auto,
    Center,
    Pos(IVec2),
}

impl WindowPosition {
    pub fn set_pos(&mut self, pos: IVec2) {
        *self = Self::Pos(pos);
    }

    pub fn center(&mut self) {
        *self = Self::Center;
    }
}

/// Window resolution in physical pixels
pub struct WindowResolution {
    width: u32,
    height: u32,
}

impl Default for WindowResolution {
    fn default() -> Self {
        Self {
            width: 1280,
            height: 720,
        }
    }
}

impl WindowResolution {
    pub const fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    pub const fn width(&self) -> u32 {
        self.width
    }

    pub const fn height(&self) -> u32 {
        self.height
    }

    pub const fn size(&self) -> UVec2 {
        UVec2::new(self.width(), self.height())
    }

    pub const fn set(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }
}

pub struct WindowResizeLimit {
    min_width: u32,
    min_height: u32,
    max_width: u32,
    max_height: u32,
}

impl Default for WindowResizeLimit {
    fn default() -> Self {
        Self {
            min_width: 144,
            min_height: 256,
            max_width: u32::MAX,
            max_height: u32::MAX,
        }
    }
}

impl WindowResizeLimit {
    pub const fn new(min_width: u32, min_height: u32, max_width: u32, max_height: u32) -> Self {
        Self {
            min_width,
            min_height,
            max_width,
            max_height,
        }
    }

    pub const fn min_width(&self) -> u32 {
        self.min_width
    }

    pub const fn min_height(&self) -> u32 {
        self.min_height
    }

    pub const fn max_width(&self) -> u32 {
        self.max_width
    }

    pub const fn max_height(&self) -> u32 {
        self.max_height
    }

    pub const fn min_size(&self) -> UVec2 {
        UVec2::new(self.min_width, self.min_height)
    }

    pub const fn max_size(&self) -> UVec2 {
        UVec2::new(self.max_width, self.max_height)
    }

    pub const fn set_min(&mut self, width: u32, height: u32) {
        self.min_width = width;
        self.min_height = height;
    }

    pub const fn set_max(&mut self, width: u32, height: u32) {
        self.max_width = width;
        self.max_height = height;
    }
}

#[derive(Default)]
pub enum WindowMode {
    #[default]
    Windowed,
    BorderlessFullscreen(Monitor),
    Fullscreen(Monitor),
}

#[derive(Default)]
pub enum WindowTheme {
    #[default]
    Auto,
    Light,
    Dark,
}

#[derive(Default)]
pub enum Monitor {
    #[default]
    Current,
    Primary,
    Index(usize),
}

#[derive(Default)]
pub enum CursorMode {
    #[default]
    Free,
    // confined to window, MacOS doesn't support
    Confined,
    // locked to a position in the window, Windows doesn't support
    Locked,
}

#[derive(Default)]
pub enum PresentMode {
    Vsync,
    NoVsync,
    #[default]
    Fifo,
    RelaxedFifo,
    Mailbox,
    Immediate,
}

#[derive(Default)]
pub enum AlphaMode {
    #[default]
    Auto,
    Opaque,
    PreMultiplied,
    PostMultiplied,
    Inherit,
}

pub struct TitlebarButtons {
    pub minimize: bool,
    pub maximize: bool,
    pub close: bool,
}

impl Default for TitlebarButtons {
    fn default() -> Self {
        Self {
            minimize: true,
            maximize: true,
            close: true,
        }
    }
}
