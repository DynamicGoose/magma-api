#[derive(Default)]
pub struct Window {
    pub inner_size: Option<winit::dpi::Size>,
    pub min_inner_size: Option<winit::dpi::Size>,
    pub max_inner_size: Option<winit::dpi::Size>,
    pub position: Option<winit::dpi::Position>,
    pub resizable: bool,
    pub title: String,
    pub maximized: bool,
    pub visible: bool,
    pub transparent: bool,
    pub blur: bool,
    pub decorations: bool,
    pub window_icon: Option<winit::window::Icon>,
    pub cursor: Option<winit::window::CursorIcon>,
}
