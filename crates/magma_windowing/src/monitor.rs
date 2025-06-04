use magma_math::{IVec2, UVec2};

/// An ECS component representing a monitor
#[derive(Debug)]
pub struct Monitor {
    /// Optional name of the monitor
    pub name: Option<String>,
    /// Physical height of the monitor
    pub height: u32,
    /// Physical width of the monitor
    pub width: u32,
    /// Physical position of the monitor
    pub position: IVec2,
    /// Refresh rate of the monitor in *millihertz*
    pub refresh_rate: Option<u32>,
    /// Scale factor for conversion between physical and logical pixels
    pub scale_factor: f64,
    /// The monitor's video modes
    pub video_modes: Vec<VideoMode>,
    /// The monitors id in the backend
    pub id: usize,
}

impl Monitor {
    /// Returns the physical size of the monitor.
    pub fn size(&self) -> UVec2 {
        UVec2::new(self.width, self.height)
    }
}

/// Marker component for the primary monitor
pub struct PrimaryMonitor;

/// Representation for a video mode
#[derive(Debug)]
pub struct VideoMode {
    /// Resolution of the video mode
    pub size: UVec2,
    /// Bit depth of the video mode
    pub bit_depth: u16,
    /// Refresh rate in *millihertz*
    pub refresh_rate: u32,
}
