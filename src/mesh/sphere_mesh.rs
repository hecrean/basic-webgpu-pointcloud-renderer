use crate::mesh::vertex::Vertex;

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, bytemuck::Pod, bytemuck::Zeroable)]
pub struct SphereMeshVertex {
    position: [f32; 3],
    // normal:  [f32; 3],
    // tangent: [f32; 3],
    // bitangent: [f32; 3],
    // color: [f32; 4],
    // tex_coords_0: [f32; 2],
    // tex_coords_1: [f32; 2],
    // tex_coords_2: [f32; 2],
    // skin_weight: [f32; 3],
    // skin_index: [f32; 3],
}

impl Vertex for SphereMeshVertex {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<SphereMeshVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                // position
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}

pub struct SphereMesh {
    pub vertices: Vec<SphereMeshVertex>,
    pub indices: Vec<u16>,
    pub len: u32,
}

impl SphereMesh {
    pub fn new() -> Self {
        const SIZE: f32 = 0.05;
        let VERTICES = vec![
            SphereMeshVertex {
                position: [-SIZE, -SIZE, -SIZE],
            }, // A
            SphereMeshVertex {
                position: [SIZE, -SIZE, -SIZE],
            }, // B
            SphereMeshVertex {
                position: [SIZE, SIZE, -SIZE],
            }, // C
            SphereMeshVertex {
                position: [-SIZE, SIZE, -SIZE],
            }, // D
            SphereMeshVertex {
                position: [-SIZE, -SIZE, SIZE],
            }, // E
            SphereMeshVertex {
                position: [SIZE, -SIZE, SIZE],
            }, // E
            SphereMeshVertex {
                position: [SIZE, SIZE, SIZE],
            }, // E
            SphereMeshVertex {
                position: [-SIZE, SIZE, SIZE],
            }, // E
        ];

        let length = VERTICES.len();

        let INDICES = vec![
            0, 1, 3, 3, 1, 2, 1, 5, 2, 2, 5, 6, 5, 4, 6, 6, 4, 7, 4, 0, 7, 7, 0, 3, 3, 2, 7, 7, 2,
            6, 4, 5, 0, 0, 5, 1,
        ];

        Self {
            vertices: VERTICES,
            indices: INDICES,
            len: length as u32,
        }
    }
}
