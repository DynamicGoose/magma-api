use feufeu::RenderStage;

use crate::sync_window::RenderWindow;

pub struct BackgroundStage;

impl RenderStage for BackgroundStage {
    fn init(_render_state: &mut feufeu::RenderState) {}

    fn run(render_state: &feufeu::RenderState) {
        let mut encoder = render_state.get_device().create_command_encoder(
            &feufeu::wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            },
        );
        let outputs = render_state
            .render_world
            .query::<(RenderWindow,)>()
            .unwrap()
            .iter()
            .map(|entity| {
                let render_window = entity.get_component::<RenderWindow>().unwrap();
                let texture = render_window.texture.to_owned().unwrap();
                let view = render_window.texture_view.to_owned().unwrap();
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
                texture
            })
            .collect::<Vec<_>>();
        render_state
            .get_queue()
            .submit(std::iter::once(encoder.finish()));
        outputs.into_iter().for_each(|o| o.present());
    }
}
