use feufeu::{
    RenderStage,
    wgpu::{Surface, SurfaceConfiguration},
};

pub struct BackgroundStage;

impl RenderStage for BackgroundStage {
    fn init(render_state: &mut feufeu::RenderState) {
        render_state.render_world.register_component::<Surface>();
        render_state
            .render_world
            .register_component::<SurfaceConfiguration>();
    }

    fn run(render_state: &feufeu::RenderState) {
        let surface_entity = render_state
            .render_world
            .query::<(Surface, SurfaceConfiguration)>()
            .unwrap()[0];
        let surface = surface_entity.get_component::<Surface>().unwrap();
        let output = surface.get_current_texture().unwrap();
        let view = output
            .texture
            .create_view(&feufeu::wgpu::TextureViewDescriptor::default());
        let mut encoder = render_state.get_device().create_command_encoder(
            &feufeu::wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            },
        );
        {
            let _render_pass = encoder.begin_render_pass(&feufeu::wgpu::RenderPassDescriptor {
                label: Some("RenderPass"),
                color_attachments: &[Some(feufeu::wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: feufeu::wgpu::Operations {
                        load: feufeu::wgpu::LoadOp::Clear(feufeu::wgpu::Color {
                            r: 0.5,
                            g: 0.2,
                            b: 0.5,
                            a: 1.0,
                        }),
                        store: feufeu::wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });
        }

        render_state
            .get_queue()
            .submit(std::iter::once(encoder.finish()));
        output.present();
    }
}
