use std::num::NonZero;

use glam::{IVec2, UVec2};
use magma_app::magma_ecs::{Component, epoch::Epoch};

/// The Window Component
#[derive(Component, Clone, PartialEq, Debug)]
pub struct Window {
    pub title: String,
    pub name: Option<String>,
    pub position: WindowPosition,
    pub resolution: WindowResolution,
    pub resizable: bool,
    pub resize_limit: WindowResizeLimit,
    pub mode: WindowMode,
    pub cursor_mode: CursorMode,
    pub cursor_position: CursorPosition,
    pub cursor_visible: bool,
    pub decorations: bool,
    pub titlebar_buttons: TitlebarButtons,
    pub present_mode: PresentMode, // wgpu
    pub alpha_mode: AlphaMode,     // wgpu
    pub transparent: bool,
    pub focused: bool,
    pub default_event_handling: bool, // internal
    pub window_theme: WindowTheme,
    pub desired_maximum_frame_latency: Option<NonZero<u32>>, // wgpu::SurfaceConfiguration::desired_maximum_frame_latency
    change_epoch: Epoch,
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
            cursor_position: Default::default(),
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
            change_epoch: Epoch::start(),
        }
    }
}

impl Window {
    pub fn new() -> Self {
        Self::default()
    }

    /// After changing window properties, the window has to be updated
    pub fn update(&mut self, epoch: Epoch) {
        self.change_epoch.update(epoch);
    }
}

/// Marks a window that has been requested to close
#[derive(Component, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
pub struct ClosingWindow;

/// Position of a window
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
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
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
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
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct WindowResizeLimit {
    min_width: u32,
    min_height: u32,
    max_width: u32,
    max_height: u32,
}

impl Default for WindowResizeLimit {
    fn default() -> Self {
        Self {
            min_width: u32::MIN,
            min_height: u32::MIN,
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

/// Cursor position in the window
#[derive(Clone, Copy, PartialEq, PartialOrd, Debug, Default)]
pub struct CursorPosition {
    pub x: f64,
    pub y: f64,
}

/// The windowing mode of the window.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, Default)]
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
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
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
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
pub enum MonitorSelection {
    /// Use the currently focused monitor.
    #[default]
    Current,
    /// Use the system's primary monitor.
    Primary,
    /// Specify monitor by it's entity.
    Entity(usize),
}

/// Specifies the window's video mode.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
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
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
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
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
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
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
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
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
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
