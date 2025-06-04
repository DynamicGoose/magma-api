use std::num::NonZero;

use magma_app::entities::Entity;
use magma_math::{IVec2, UVec2};

/// The Window Component
#[derive(Clone, PartialEq, Eq, Debug)]
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

impl Default for Window {
    fn default() -> Self {
        Self {
            title: "Magma Window".to_owned(),
            name: None,
            position: Default::default(),
            resolution: Default::default(),
            resizable: true,
            resize_limit: Default::default(),
            mode: Default::default(),
            cursor_mode: Default::default(),
            cursor_visible: true,
            decorations: true,
            titlebar_buttons: Default::default(),
            present_mode: Default::default(),
            alpha_mode: Default::default(),
            transparent: false,
            focused: true,
            default_event_handling: true,
            window_theme: Default::default(),
            desired_maximum_frame_latency: NonZero::new(2_u32),
            has_window: false,
            changed_attr: false,
        }
    }
}

impl Window {
    pub fn new() -> Self {
        Self::default()
    }

    /// Create the window with a custom title.
    pub fn with_title(mut self, title: &str) -> Self {
        self.title = title.to_owned();
        self
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

    /// Create the window with a custom (optional) name.
    pub fn with_name(mut self, name: &str) -> Self {
        self.name = Some(name.to_owned());
        self
    }

    /// Get the window's name.
    pub fn name(&self) -> Option<String> {
        self.name.to_owned()
    }

    /// Set the window's optional name. This can only be set after first creating the window.
    pub fn set_name(&mut self, name: &str) {
        self.name = Some(name.to_owned());
        self.changed_attr = true;
    }

    /// Create the window with specified [`WindowPosition`].
    pub fn with_position(mut self, position: WindowPosition) -> Self {
        self.position = position;
        self
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

    /// Create the window with specified [`WindowResolution`].
    pub fn with_resolution(mut self, resolution: WindowResolution) -> Self {
        self.resolution = resolution;
        self
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

    /// Set if the window should be resizable on creation.
    pub fn with_resizable(mut self, resizable: bool) -> Self {
        self.resizable = resizable;
        self
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

    /// Create the window with specified [`WindowResizeLimit`].
    pub fn with_resize_limit(mut self, resize_limit: WindowResizeLimit) -> Self {
        self.resize_limit = resize_limit;
        self
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

    /// Create the window with specified [`WindowMode`].
    pub fn with_mode(mut self, mode: WindowMode) -> Self {
        self.mode = mode;
        self
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

    /// Create the window with specified [`CursorMode`].
    pub fn with_cursor_mode(mut self, cursor_mode: CursorMode) -> Self {
        self.cursor_mode = cursor_mode;
        self
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

    /// Set if the cursor should be visible at window creation.
    pub fn with_cursor_visible(mut self, cursor_visible: bool) -> Self {
        self.cursor_visible = cursor_visible;
        self
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

    /// Set if window decorations should be enabled at window creation.
    pub fn with_decorations(mut self, decorations: bool) -> Self {
        self.decorations = decorations;
        self
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

    /// Set which [`TitlebarButtons`] should be enabled at window creation.
    pub fn with_titlebar_buttons(mut self, titlebar_buttons: TitlebarButtons) -> Self {
        self.titlebar_buttons = titlebar_buttons;
        self
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

    /// Create the window with specified [`PresentMode`].
    pub fn with_present_mode(mut self, present_mode: PresentMode) -> Self {
        self.present_mode = present_mode;
        self
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

    /// Create the Window with specified [`AlphaMode`].
    pub fn with_alpha_mode(mut self, alpha_mode: AlphaMode) -> Self {
        self.alpha_mode = alpha_mode;
        self
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

    /// Set if the window should be transparent at window creation.
    pub fn with_transparent(mut self, transparent: bool) -> Self {
        self.transparent = transparent;
        self
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

    /// Create the window with specified focus.
    pub fn with_focused(mut self, focused: bool) -> Self {
        self.focused = focused;
        self
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

    /// Set if default event handling should be enabled at window creation.
    pub fn with_default_event_handling(mut self, default_event_handling: bool) -> Self {
        self.default_event_handling = default_event_handling;
        self
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

    /// Create the window with specified [`WindowTheme`].
    pub fn with_window_theme(mut self, window_theme: WindowTheme) -> Self {
        self.window_theme = window_theme;
        self
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

    /// Create the window disired maximum frame latency.
    pub fn with_desired_maximum_frame_latency(
        mut self,
        desired_maximum_frame_latency: Option<NonZero<u32>>,
    ) -> Self {
        self.desired_maximum_frame_latency = desired_maximum_frame_latency;
        self
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
    /// Returns a [`WindowResolution`] with specified physical width and height.
    pub const fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    /// Get the [`WindowResolution`]'s physical width.
    pub const fn width(&self) -> u32 {
        self.width
    }

    /// Get the [`WindowResolution`]'s physical height.
    pub const fn height(&self) -> u32 {
        self.height
    }

    /// Get the [`WindowResolution`]'s physical size as a [`UVec2`].
    pub const fn size(&self) -> UVec2 {
        UVec2::new(self.width, self.height)
    }
}

/// Resize limit of a window.
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
    /// Returns a [`WindowResizeLimit`] with specified constraints.
    pub const fn new(min_width: u32, min_height: u32, max_width: u32, max_height: u32) -> Self {
        Self {
            min_width,
            min_height,
            max_width,
            max_height,
        }
    }

    /// Get the minimum width.
    pub const fn min_width(&self) -> u32 {
        self.min_width
    }

    /// Get the minimum height.
    pub const fn min_height(&self) -> u32 {
        self.min_height
    }

    /// Get the maximum width.
    pub const fn max_width(&self) -> u32 {
        self.max_width
    }

    /// Get the maximum height.
    pub const fn max_height(&self) -> u32 {
        self.max_height
    }

    /// Get the minimum size as a [`UVec2`].
    pub const fn min_size(&self) -> UVec2 {
        UVec2::new(self.min_width, self.min_height)
    }

    /// Get the maximum size as a [`UVec2`].
    pub const fn max_size(&self) -> UVec2 {
        UVec2::new(self.max_width, self.max_height)
    }
}

/// The windowing mode of the window.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum WindowMode {
    /// The window should be the size of it's resolution.
    #[default]
    Windowed,
    /// The window along with its resolution gets upscaled to fit the screen.
    BorderlessFullscreen(MonitorSelection),
    /// True fullscreen mode. The window occupies the whole screen, its resolution is not modified.
    Fullscreen(MonitorSelection, VideoModeSelection),
}

/// The theme variant to use
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum WindowTheme {
    /// The window will use the system's global theme variant.
    #[default]
    Auto,
    /// Use the light theme variant.
    Light,
    /// Use the dark theme variant.
    Dark,
}

/// The monitor to use for a window.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum MonitorSelection {
    /// Use the currently focused monitor.
    #[default]
    Current,
    /// Use the system's primary monitor.
    Primary,
    /// Specify monitor by it's entity.
    Entity(Entity),
}

/// Specifies the window's video mode.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum VideoModeSelection {
    /// Use the current monitor's viodeo mode
    #[default]
    Current,
    /// Specify a video mode to use.
    Specific {
        size: UVec2,
        bit_depth: u16,
        refresh_rate_millihertz: u32,
    },
}

/// The window's curosr mode.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum CursorMode {
    /// The cursor can freely move in and outside the window.
    #[default]
    Free,
    /**
    The cursor will be confined to the window.

    # Support

    MacOS doesn't support this mode, therfore on MacOS this will be converted to locked cursor mode.
    */
    Confined,
    /**
    The cursor will be locked in one place.

    # Support

    Windows doesn't support the mode, therefore on Windows this will be converted to confined cursor mode.
    */
    Locked,
}

/// The window's present mode
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum PresentMode {
    /// Chooses [`RelaxedFifo`](Self::RelaxedFifo) -> [`Fifo`](Self::Fifo) however available.
    Vsync,
    /// Chooses [`Immediate`](Self::Immediate) -> [`Mailbox`](Self::Mailbox) however available.
    NoVsync,
    /**
    Presentation frames are kept in a First-In-First-Out queue approximately 3 frames
    long. Every vertical blanking period, the presentation engine will pop a frame
    off the queue to display. If there is no frame to display, it will present the same
    frame again until the next vblank.

    - When a present command is executed on the gpu, the presented image is added on the queue.
    - no tearing
    - traditionally "VSync"
    */
    #[default]
    Fifo,
    /**
    Presentation frames are kept in a First-In-First-Out queue approximately 3 frames
    long. Every vertical blanking period, the presentation engine will pop a frame
    off the queue to display. If there is no frame to display, it will present the
    same frame until there is a frame in the queue. The moment there is a frame in the
    queue, it will immediately pop the frame off the queue.

    - When a present command is executed on the gpu, the presented image is added on the queue.
    - Tearing, if frames last more than one vblank as the front buffer.
    - supported on AMD + Vulkan
    - traditionally "Adaptive Vsync"
    */
    RelaxedFifo,
    /**
    Presentation frames are kept in a single-frame queue. Every vertical blanking period,
    the presentation engine will pop a frame from the queue. If there is no frame to display,
    it will present the same frame again until the next vblank.

    When a present command is executed on the gpu, the frame will be put into the queue.
    If there was already a frame in the queue, the new frame will _replace_ the old frame
    on the queue.

    - no tearing
    - supported on DX11/12 + Windows 10, NVidia + Vulkan and Wayland + Vulkan.
    - traditionally "Fast Vsync"
    */
    Mailbox,
    /**
    Presentation frames are not queued at all. The moment a present command
    is executed on the GPU, the presented image is swapped onto the front buffer
    immediately.

    - tearing
    - supported on most platforms except older DX12 + Wayland.
    - traditionally "Vsync Off".
    */
    Immediate,
}

/// The window's alpha mode
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum AlphaMode {
    /// Automatically determine alpha mode.
    #[default]
    Auto,
    /// The window will always be opaque.
    Opaque,
    /// The alpha channel of textures is respected when compositing.
    /// The non-alpha channels should already be multiplied by the alpha channel.
    PreMultiplied,
    /// The alpha channel of textures is respected when compositing.
    /// The non-alpha channels should _not_ already be multiplied by the alpha channel.
    PostMultiplied,
    /// The alpha channel of textures is unknown for compositing.
    Inherit,
}

/// Used for specifying which titlebar buttons should be enabled on a window.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct TitlebarButtons {
    minimize: bool,
    maximize: bool,
    close: bool,
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

impl TitlebarButtons {
    /// Get [`TitlebarButtons`] with specified buttons activated.
    pub const fn new(minimize: bool, maximize: bool, close: bool) -> Self {
        Self {
            minimize,
            maximize,
            close,
        }
    }

    /// Get [`TitlebarButtons`] with all buttons enabled.
    pub const fn all_enabled() -> Self {
        Self {
            minimize: true,
            maximize: true,
            close: true,
        }
    }

    /// Is teh minimize button enabled?
    pub const fn minimize(&self) -> bool {
        self.minimize
    }

    /// Is the maximize button enabled?
    pub const fn maximize(&self) -> bool {
        self.maximize
    }

    /// Is the close button enabled?
    pub const fn close(&self) -> bool {
        self.close
    }
}
