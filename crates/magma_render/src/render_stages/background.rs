use feufeu::RenderStage;

use crate::sync_window::RenderWindow;

pub struct BackgroundStage;

impl RenderStage for BackgroundStage {
    fn init(_render_state: &mut feufeu::RenderState) {}

    fn run(render_state: &feufeu::RenderState) {
        render_state
            .render_world
            .query::<(RenderWindow,)>()
            .unwrap()
            .iter()
            .for_each(|entity| {
                let mut encoder = render_state.get_device().create_command_encoder(
                    &feufeu::wgpu::CommandEncoderDescriptor {
                        label: Some("Render Encoder"),
                    },
                );
                let mut render_window = entity.get_component_mut::<RenderWindow>().unwrap();
                let texture = render_window.texture.take().unwrap();
                let view = render_window.texture_view.take().unwrap();
                {
                    let _render_pass =
                        encoder.begin_render_pass(&feufeu::wgpu::RenderPassDescriptor {
                            label: Some("RenderPass"),
                            color_attachments: &[Some(feufeu::wgpu::RenderPassColorAttachment {
                                view: &view,
                                resolve_target: None,
                                ops: feufeu::wgpu::Operations {
                                    load: feufeu::wgpu::LoadOp::Clear(feufeu::wgpu::Color {
                                        r: 0.2,
                                        g: 0.0,
                                        b: 0.2,
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
                texture.present();
            });
    }
}
