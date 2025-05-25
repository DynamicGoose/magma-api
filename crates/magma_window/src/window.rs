use std::num::NonZero;

use magma_math::{IVec2, UVec2};

#[derive(Clone, PartialEq, Eq, Debug, Default)]
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

    /// Get the window's display title.
    pub fn title(&self) -> String {
        self.title.to_owned()
    }

    /// Set the window's display title.
    pub fn set_title(&mut self, title: &str) {
        self.title = title.to_owned();
        self.changed_attr = true;
    }

    /// Get the window's optional name.
    pub fn name(&self) -> Option<String> {
        self.name.to_owned()
    }

    /// Set the window's optional name. This can only be set after first creating the window.
    pub fn set_name(&mut self, name: &str) {
        self.name = Some(name.to_owned());
        self.changed_attr = true;
    }

    /// Get the current [`WindowPosition`].
    pub fn position(&self) -> WindowPosition {
        self.position
    }

    /// Set the [`WindowPosition`].
    pub fn set_position(&mut self, position: WindowPosition) {
        self.position = position;
        self.changed_attr = true;
    }

    /// Get the current [`WindowResolution`].
    pub fn resolution(&self) -> WindowResolution {
        self.resolution
    }

    /// Set the [`WindowResolution`].
    pub fn set_resolution(&mut self, resolution: WindowResolution) {
        self.resolution = resolution;
        self.changed_attr = true;
    }

    /// Is the window resizable?
    pub fn resizable(&self) -> bool {
        self.resizable
    }

    /// Set if the window should be resizable.
    pub fn set_resizable(&mut self, resizable: bool) {
        self.resizable = resizable;
        self.changed_attr = true;
    }

    /// Get the current [`WindowResizeLimit`].
    pub fn resize_limit(&self) -> WindowResizeLimit {
        self.resize_limit
    }

    /// Set the [`WindowResizeLimit`].
    pub fn set_resize_limit(&mut self, resize_limit: WindowResizeLimit) {
        self.resize_limit = resize_limit;
        self.changed_attr = true;
    }

    /// Get the current [`WindowMode`]
    pub fn mode(&self) -> WindowMode {
        self.mode
    }

    /// Set the [`WindowMode`].
    pub fn set_mode(&mut self, mode: WindowMode) {
        self.mode = mode;
        self.changed_attr = true;
    }

    /// Get the current [`CursorMode`].
    pub fn cursor_mode(&self) -> CursorMode {
        self.cursor_mode
    }

    /// Set the [`CursorMode`].
    pub fn set_cursor_mode(&mut self, cursor_mode: CursorMode) {
        self.cursor_mode = cursor_mode;
        self.changed_attr = true;
    }

    /// Is the cursor visible?
    pub fn cursor_visible(&self) -> bool {
        self.cursor_visible
    }

    /// Set if the cursor should be visible.
    pub fn set_cursor_visible(&mut self, cursor_visible: bool) {
        self.cursor_visible = cursor_visible;
        self.changed_attr = true;
    }

    /// Are window decorations enabled?
    pub fn decorations(&self) -> bool {
        self.decorations
    }

    /// Set if window decorations should be enabled.
    pub fn set_decorations(&mut self, decorations: bool) {
        self.decorations = decorations;
        self.changed_attr = true;
    }

    /// Get enabled [`TitlebarButtons`].
    pub fn titlebar_buttons(&self) -> TitlebarButtons {
        self.titlebar_buttons
    }

    /// Set which [`TitlebarButtons`] should be enabled.
    pub fn set_titlebar_buttons(&mut self, titlebar_buttons: TitlebarButtons) {
        self.titlebar_buttons = titlebar_buttons;
        self.changed_attr = true;
    }

    /// Get the current [`PresentMode`].
    pub fn present_mode(&self) -> PresentMode {
        self.present_mode
    }

    /// Set the [`PresentMode`].
    pub fn set_present_mode(&mut self, present_mode: PresentMode) {
        self.present_mode = present_mode;
        self.changed_attr = true;
    }

    /// Get the current [`AlphaMode`].
    pub fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }

    /// Set the [`AlphaMode`].
    pub fn set_alpha_mode(&mut self, alpha_mode: AlphaMode) {
        self.alpha_mode = alpha_mode;
        self.changed_attr = true;
    }

    /// Is the window transparent?
    pub fn transparent(&self) -> bool {
        self.transparent
    }

    /// Set if the window should be transparent.
    pub fn set_transparent(&mut self, transparent: bool) {
        self.transparent = transparent;
        self.changed_attr = true;
    }

    /// Is the window focused?
    pub fn focused(&self) -> bool {
        self.focused
    }

    /// Set if the window should be focused.
    pub fn set_focused(&mut self, focused: bool) {
        self.focused = focused;
        self.changed_attr = true;
    }

    /// Is default event handling enabled for this window?
    pub fn default_event_handling(&self) -> bool {
        self.default_event_handling
    }

    /// Set if default event handling should be enabled.
    pub fn set_default_event_handling(&mut self, default_event_handling: bool) {
        self.default_event_handling = default_event_handling;
        self.changed_attr = true;
    }

    /// Get the current [`WindowTheme`].
    pub fn window_theme(&self) -> WindowTheme {
        self.window_theme
    }

    /// Set the [`WindowTheme`].
    pub fn set_window_theme(&mut self, window_theme: WindowTheme) {
        self.window_theme = window_theme;
        self.changed_attr = true;
    }

    /// Get the desired maximum frame latency (see [`wgpu::SurfaceConfiguration::desired_maximum_frame_latency`](https://docs.rs/wgpu/latest/wgpu/type.SurfaceConfiguration.html#structfield.desired_maximum_frame_latency)).
    pub fn desired_maximum_frame_latency(&self) -> Option<NonZero<u32>> {
        self.desired_maximum_frame_latency
    }

    /// Set the disired maximum frame latency.
    pub fn set_desired_maximum_frame_latency(
        &mut self,
        desired_maximum_frame_latency: Option<NonZero<u32>>,
    ) {
        self.desired_maximum_frame_latency = desired_maximum_frame_latency;
        self.changed_attr = true;
    }
}

/// Marks a window that has been requested to close
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct ClosingWindow;

/// Position of a window
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum WindowPosition {
    /// Automatically set an initial position for the [`Window`]. This will be converted into [`WindowPosition::Pos`] once the window has been created.
    #[default]
    Auto,
    /// Center the [`Window`] on the screen. This will be converted to [`WindowPosition::Pos`] once the window has been created.
    Center,
    /// Physical position of a window starting from the top left corner of the screen.
    Pos(IVec2),
}

/// Window resolution in physical pixels.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct WindowResolution {
    pub width: u32,
    pub height: u32,
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
        UVec2::new(self.width, self.height)
    }
}

/// Resize limit of a window.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct WindowResizeLimit {
    pub min_width: u32,
    pub min_height: u32,
    pub max_width: u32,
    pub max_height: u32,
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

/// The windowing mode of the window.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum WindowMode {
    /// The window should be the size of it's resolution.
    #[default]
    Windowed,
    /// The window along with its resolution gets upscaled to fit the screen.
    BorderlessFullscreen(Monitor),
    /// True fullscreen mode. The window occupies the whole screen, its resolution is not modified.
    Fullscreen(Monitor, VideoMode),
}

/// The theme variant to use
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
