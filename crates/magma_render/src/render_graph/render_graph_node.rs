pub struct RenderGraphNode {
    pub kind: RenderGraphNodeKind,
}

pub enum RenderGraphNodeKind {
    Render,
    Compute,
}
