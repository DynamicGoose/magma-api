use magma_app::{World, magma_ecs::world::UnsafeWorldMut};
use pollster::FutureExt;
use wgpu::{Adapter, Device, DeviceDescriptor, Instance, Queue, RequestAdapterOptions};

use crate::sync_windows::RenderWindow;

pub struct RenderState<'a, 'b> {
    wgpu_instance: Instance,
    wgpu_adapter: (Adapter, RequestAdapterOptions<'a, 'b>),
    wgpu_device: (Device, DeviceDescriptor<'a>),
    wgpu_queue: Queue,
    pub renderer: fn(&mut RenderState),
    pub render_world: World,
}

impl<'a, 'b> Default for RenderState<'a, 'b> {
    fn default() -> Self {
        let wgpu_instance = Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::VULKAN,
            flags: wgpu::InstanceFlags::default(),
            memory_budget_thresholds: wgpu::MemoryBudgetThresholds::default(),
            backend_options: wgpu::BackendOptions::default(),
            display: None, // not final
        });

        let adapter_options = wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            force_fallback_adapter: false,
            compatible_surface: None,
            apply_limit_buckets: false,
        };

        let wgpu_adapter = wgpu_instance
            .request_adapter(&adapter_options)
            .block_on()
            .unwrap();

        let device_desc = wgpu::DeviceDescriptor {
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::default(),
            label: Some("magma_render device"),
            memory_hints: Default::default(),
            trace: wgpu::Trace::Off,
            experimental_features: wgpu::ExperimentalFeatures::disabled(),
        };

        let (wgpu_device, wgpu_queue) = wgpu_adapter
            .request_device(&device_desc)
            .block_on()
            .unwrap();

        let render_world = World::default();

        Self {
            wgpu_instance,
            wgpu_adapter: (wgpu_adapter, adapter_options),
            wgpu_device: (wgpu_device, device_desc),
            wgpu_queue,
            renderer,
            render_world,
        }
    }
}

impl<'a, 'b> RenderState<'a, 'b> {
    pub fn new() -> Self {
        Self::default()
    }

    /**
    Used to change options of the wgpu Instance.
    ```
    use magma_render::RenderState;

    let mut render_state = RenderState::new();
    render_state.set_instance(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::VULKAN,
        flags: wgpu::InstanceFlags::default(),
        memory_budget_thresholds: wgpu::MemoryBudgetThresholds::default(),
        backend_options: wgpu::BackendOptions::default(),
        display: None,
    });
    ```
    */
    pub fn set_instance(&mut self, desc: wgpu::InstanceDescriptor) {
        let instance = Instance::new(desc);
        let adapter = instance
            .request_adapter(&self.wgpu_adapter.1)
            .block_on()
            .unwrap();
        let (device, queue) = adapter
            .request_device(&self.wgpu_device.1)
            .block_on()
            .unwrap();

        self.wgpu_instance = instance;
        self.wgpu_adapter.0 = adapter;
        self.wgpu_device.0 = device;
        self.wgpu_queue = queue;
    }

    /**
    Used to change options of the wgpu Adapter.
    ```
    use magma_render::RenderState;

    let mut render_state = RenderState::new();
    render_state.set_adapter(wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::HighPerformance,
        force_fallback_adapter: false,
        compatible_surface: None,
        apply_limit_buckets: false,
    }).unwrap();
    ```
    */
    pub fn set_adapter(
        &mut self,
        options: wgpu::RequestAdapterOptions<'a, 'b>,
    ) -> Result<(), wgpu::RequestAdapterError> {
        let adapter = self.wgpu_instance.request_adapter(&options).block_on();

        match adapter {
            Err(e) => return Err(e),
            Ok(adapter) => {
                let (device, queue) = adapter
                    .request_device(&self.wgpu_device.1)
                    .block_on()
                    .unwrap();

                self.wgpu_adapter = (adapter, options);
                self.wgpu_device.0 = device;
                self.wgpu_queue = queue;
                return Ok(());
            }
        }
    }

    /**
    Used to change options of the wgpu Device.
    ```
    use magma_render::RenderState;

    let mut render_state = RenderState::new();
    render_state.set_device(wgpu::DeviceDescriptor {
        required_features: wgpu::Features::empty(),
        required_limits: wgpu::Limits::default(),
        label: Some("FeuFeu device"),
        memory_hints: Default::default(),
        trace: wgpu::Trace::Off,
        experimental_features: wgpu::ExperimentalFeatures::disabled(),
    }).unwrap();
    ```
    */
    pub fn set_device(
        &mut self,
        desc: wgpu::DeviceDescriptor<'a>,
    ) -> Result<(), wgpu::RequestDeviceError> {
        let (device, queue) = self.wgpu_adapter.0.request_device(&desc).block_on()?;
        self.wgpu_device = (device, desc);
        self.wgpu_queue = queue;
        Ok(())
    }

    /// Get a reference to the wgpu Instance.
    pub fn get_instance(&self) -> &Instance {
        &self.wgpu_instance
    }

    /// Get a reference to the wgpu Adapter.
    pub fn get_adapter(&self) -> &Adapter {
        &self.wgpu_adapter.0
    }

    /// Get a renference to the wgpu Device.
    pub fn get_device(&self) -> &Device {
        &self.wgpu_device.0
    }

    /// Get a reference to the wgpu Queue.
    pub fn get_queue(&self) -> &Queue {
        &self.wgpu_queue
    }
}

fn renderer(render_state: &mut RenderState) {
    for (_, render_window) in
        unsafe { UnsafeWorldMut::new(&mut render_state.render_world).get_mut() }
            .component_store
            .get_components_mut::<RenderWindow>()
            .unwrap()
    {
        if let Some(texture) = render_window.texture.take() {
            let view = render_window.texture_view.take().unwrap();
            let mut encoder =
                render_state
                    .get_device()
                    .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                        label: Some("Render Encoder"),
                    });
            {
                let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("RenderPass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: 0.2,
                                g: 0.0,
                                b: 0.2,
                                a: 1.0,
                            }),
                            store: wgpu::StoreOp::Store,
                        },
                        depth_slice: None,
                    })],
                    depth_stencil_attachment: None,
                    occlusion_query_set: None,
                    timestamp_writes: None,
                    multiview_mask: None,
                });
            }
            render_state
                .get_queue()
                .submit(std::iter::once(encoder.finish()));
            render_state.get_queue().present(texture);
        }
    }
}
