#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

impl Vertex {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress, // Specifies how wide on vertex is (How many bytes of the array are ONE vertex)
            step_mode: wgpu::VertexStepMode::Vertex, // Per-vertex data
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0, // Is at the first position
                    shader_location: 0, // location(0) in the shader
                    format: wgpu::VertexFormat::Float32x3, // The format. It is a [f32; 3]
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress, // location(0) is [f32; 3], so location(1) is size_of([f32; 3]) offsetted
                    shader_location: 1, // location(1) in the shader
                    format: wgpu::VertexFormat::Float32x3, // Is r, g, b, so f32 x 3
                }
            ]
        }
    }    
}