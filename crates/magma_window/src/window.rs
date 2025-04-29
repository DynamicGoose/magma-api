use std::num::NonZero;

use magma_math::{IVec2, UVec2};

#[derive(Default)]
pub struct Window {
    title: String,
    name: Option<String>,
    position: WindowPosition,
    resolution: WindowResolution,
    resizable: bool,
    resize_limit: WindowResizeLimit,
    mode: WindowMode,
    cursor_mode: CursorMode,
    cursor_visible: bool,
    decorations: bool,
    titlebar_buttons: TitlebarButtons,
    present_mode: PresentMode, // wgpu
    alpha_mode: AlphaMode,     // wgpu
    transparent: bool,
    focused: bool,
    default_event_handling: bool, // internal
    window_theme: WindowTheme,
    desired_maximum_frame_latency: Option<NonZero<u32>>, // wgpu::SurfaceConfiguration::desired_maximum_frame_latency

    /// True if the backend has created a window for this component.
    pub has_window: bool,
    /// True if this component was modified in the current update. This does not include changes that the backend makesto sync windows.
    pub changed_attr: bool,
}

// TODO: Emit WindowAttributesChange event, when things are modified.
impl Window {
    pub fn new() -> Self {
        Self {
            title: "Magma Window".to_owned(),
            resizable: true,
            cursor_visible: true,
            decorations: true,
            default_event_handling: true,
            desired_maximum_frame_latency: NonZero::new(2_u32),
            ..Default::default()
        }
    }

    pub fn title(&self) -> String {
        self.title.to_owned()
    }

    pub fn set_title(&mut self, title: &str) {
        self.title = title.to_owned();
        self.changed_attr = true;
    }

    pub fn name(&self) -> Option<String> {
        self.name.to_owned()
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = Some(name.to_owned());
        self.changed_attr = true;
    }

    pub fn position(&self) -> WindowPosition {
        self.position
    }

    pub fn set_position(&mut self, position: WindowPosition) {
        self.position = position;
        self.changed_attr = true;
    }

    pub fn resolution(&self) -> WindowResolution {
        self.resolution
    }

    pub fn set_resolution(&mut self, resolution: WindowResolution) {
        self.resolution = resolution;
        self.changed_attr = true;
    }

    pub fn resizable(&self) -> bool {
        self.resizable
    }

    pub fn set_resizable(&mut self, resizable: bool) {
        self.resizable = resizable;
        self.changed_attr = true;
    }

    pub fn resize_limit(&self) -> WindowResizeLimit {
        self.resize_limit
    }

    pub fn set_resize_limit(&mut self, resize_limit: WindowResizeLimit) {
        self.resize_limit = resize_limit;
        self.changed_attr = true;
    }

    pub fn mode(&self) -> WindowMode {
        self.mode
    }

    pub fn set_mode(&mut self, mode: WindowMode) {
        self.mode = mode;
        self.changed_attr = true;
    }

    pub fn cursor_mode(&self) -> CursorMode {
        self.cursor_mode
    }

    pub fn set_cursor_mode(&mut self, cursor_mode: CursorMode) {
        self.cursor_mode = cursor_mode;
        self.changed_attr = true;
    }

    pub fn cursor_visible(&self) -> bool {
        self.cursor_visible
    }

    pub fn set_cursor_visible(&mut self, cursor_visible: bool) {
        self.cursor_visible = cursor_visible;
        self.changed_attr = true;
    }

    pub fn decorations(&self) -> bool {
        self.decorations
    }

    pub fn set_decorations(&mut self, decorations: bool) {
        self.decorations = decorations;
        self.changed_attr = true;
    }

    pub fn titlebar_buttons(&self) -> TitlebarButtons {
        self.titlebar_buttons
    }

    pub fn set_titlebar_buttons(&mut self, titlebar_buttons: TitlebarButtons) {
        self.titlebar_buttons = titlebar_buttons;
        self.changed_attr = true;
    }

    pub fn present_mode(&self) -> PresentMode {
        self.present_mode
    }

    pub fn set_present_mode(&mut self, present_mode: PresentMode) {
        self.present_mode = present_mode;
        self.changed_attr = true;
    }

    pub fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }

    pub fn set_alpha_mode(&mut self, alpha_mode: AlphaMode) {
        self.alpha_mode = alpha_mode;
        self.changed_attr = true;
    }

    pub fn transparent(&self) -> bool {
        self.transparent
    }

    pub fn set_transparent(&mut self, transparent: bool) {
        self.transparent = transparent;
        self.changed_attr = true;
    }

    pub fn focused(&self) -> bool {
        self.focused
    }

    pub fn set_focused(&mut self, focused: bool) {
        self.focused = focused;
        self.changed_attr = true;
    }

    pub fn default_event_handling(&self) -> bool {
        self.default_event_handling
    }

    pub fn set_default_event_handling(&mut self, default_event_handling: bool) {
        self.default_event_handling = default_event_handling;
        self.changed_attr = true;
    }

    pub fn window_theme(&self) -> WindowTheme {
        self.window_theme
    }

    pub fn set_window_theme(&mut self, window_theme: WindowTheme) {
        self.window_theme = window_theme;
        self.changed_attr = true;
    }

    pub fn desired_maximum_frame_latency(&self) -> Option<NonZero<u32>> {
        self.desired_maximum_frame_latency
    }

    pub fn set_desired_maximum_frame_latency(
        &mut self,
        desired_maximum_frame_latency: Option<NonZero<u32>>,
    ) {
        self.desired_maximum_frame_latency = desired_maximum_frame_latency;
        self.changed_attr = true;
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
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
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
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

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
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

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum WindowMode {
    #[default]
    Windowed,
    BorderlessFullscreen(Monitor),
    Fullscreen(Monitor, VideoMode),
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum WindowTheme {
    #[default]
    Auto,
    Light,
    Dark,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum Monitor {
    #[default]
    Current,
    Primary,
    Index(usize),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum VideoMode {
    #[default]
    Current,
    Specific {
        size: UVec2,
        bit_depth: u16,
        refresh_rate_millihertz: u32,
    },
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum CursorMode {
    #[default]
    Free,
    // confined to window, MacOS doesn't support, will be locked on MacOS
    Confined,
    // locked to a position in the window, Windows doesn't support, will be confined on Windows
    Locked,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum PresentMode {
    Vsync,
    NoVsync,
    #[default]
    Fifo,
    RelaxedFifo,
    Mailbox,
    Immediate,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum AlphaMode {
    #[default]
    Auto,
    Opaque,
    PreMultiplied,
    PostMultiplied,
    Inherit,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
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
