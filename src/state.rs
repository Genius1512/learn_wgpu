use wgpu::util::DeviceExt;
use winit::{event::WindowEvent, window::Window};

use crate::{
    constants::{self, VERTICES},
    Vertex,
};

pub struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    pub(crate) config: wgpu::SurfaceConfiguration,
    pub(crate) size: winit::dpi::PhysicalSize<u32>,
    window: Window,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    num_vertices: u32,
}

impl State {
    pub async fn new(window: Window) -> State {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::Backends::all()); // A wgpu instance
        let surface = unsafe { instance.create_surface(&window) }; // The surface to draw to
        let adapter = instance // Adapter to handle hardware
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(), // Energy efficient graphics card (laptops), High performance GPUs (graphics cards)
                compatible_surface: Some(&surface), // The surface that the adapter has to be compatible with
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter // Connection to the graphics card
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(), // The more features, the more efficent, but also the less supported devices
                    limits: wgpu::Limits::default(), // WebGL does not support everything, has to be changed when building for web
                    label: None,
                },
                None,
            )
            .await
            .unwrap();

        let config = wgpu::SurfaceConfiguration {
            // Defines how the surface will create textures
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT, // -> The textures will be used to write to the screen
            format: surface.get_supported_formats(&adapter)[0], // The formats that the GPU supports
            width: size.width,                             // The width of the surface
            height: size.height,                           // The height of the surface
            present_mode: wgpu::PresentMode::Fifo, // Caps the display rate at the display's framerate (essentially V-Sync)
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
        };
        surface.configure(&device, &config);

        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl")); // Load the shader

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            // Represents a rendering object
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,        // The shader to use as the vertex shader
                entry_point: "vs_main", // The entry point of the vertex shader
                buffers: &[
                    // The formats that will be used for vertices
                    Vertex::desc(), // Vertex::desc() returns a vertex descriptor, which tells wgpu the format of vertices
                ],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,        // The shader to use as the fragment shader
                entry_point: "fs_main", // The entry point of the fragment shader
                targets: &[Some(wgpu::ColorTargetState {
                    // The targets for wgpu to write color to. Currently only the surface
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE), // Replace old pixels with new ones
                    write_mask: wgpu::ColorWrites::ALL, // Draw all color channels: Red, Green, Blue, Alpha
                })],
            }),
            primitive: wgpu::PrimitiveState {
                // Discribe how to convert vertices into triangles
                topology: wgpu::PrimitiveTopology::TriangleList, // -> Every three vertices will correspond to one triangle
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw, // `front_face` and `cull_mode` tell wgpu how to determine wether an object is facing forward. In this case, it is facing forward when the vertices are defined counterclockwise
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None, // This program is not using any stencil
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor { // Create the vertex buffer
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(constants::VERTICES), // Create it from the constant `constants::VERTICES`. Bytemuck cast the vertices to &[u8]
            usage: wgpu::BufferUsages::VERTEX, // -> It is a vertex
        });

        let num_vertices = VERTICES.len() as u32; // Number of vertices per buffer

        State {
            surface,
            config,
            device,
            queue,
            size,
            window,
            render_pipeline,
            vertex_buffer,
            num_vertices,
        }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > constants::MIN_WIDTH && new_size.height > constants::MIN_HEIGHT { // The size is valid
            self.size = new_size; // Set the struct variable to the new size
            self.config.width = new_size.width; // Resize width
            self.config.height = new_size.height; // Resize hight
            self.surface.configure(&self.device, &self.config); // Reconfigure
        }
    }

    pub(crate) fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            _ => false,
        }
    }

    pub(crate) fn update(&mut self) {}

    pub(crate) fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?; // Get the surface to draw to
        let view = output // 
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self // Used for sending commands to the GPU
            .device // The GPU to use
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor { // Begin to render
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color { // Clear the background
                            r: 0.118,
                            g: 0.118,
                            b: 0.18,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&self.render_pipeline); // Specify the pipeline to use
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..)); // Push the vertex buffer specified by constants::VERTEX to location 0, which is picked up by the shader
            render_pass.draw(0..self.num_vertices, 0..1); // Draw
        };

        self.queue.submit(std::iter::once(encoder.finish())); // Send "Done!"
        output.present(); // Show the output on the screen

        Ok(())
    }
}
