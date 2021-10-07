use crate::mesh::vertex::Vertex;
use cgmath::{Matrix4, Quaternion, Vector3};

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Instance {
    model_matrix: [[f32; 4]; 4],
    color: [f32; 4],
}
impl Instance {
    pub fn new(model_matrix: [[f32; 4]; 4], color: [f32; 4]) -> Self {
        Self {
            model_matrix,
            color,
        }
    }
}

impl Vertex for Instance {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Instance>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                // A mat4 takes up 4 vertex slots as it is technically 4 vec4s.
                // model_matrix_0
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 10,
                    format: wgpu::VertexFormat::Float32x4,
                },
                // model_matrix_1
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 11,
                    format: wgpu::VertexFormat::Float32x4,
                },
                // model_matrix_2
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 12,
                    format: wgpu::VertexFormat::Float32x4,
                },
                // model_matrix_3
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 12]>() as wgpu::BufferAddress,
                    shader_location: 13,
                    format: wgpu::VertexFormat::Float32x4,
                },
                // color
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 16]>() as wgpu::BufferAddress,
                    shader_location: 14,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }
    }
}

impl From<RawInstance> for Instance {
    fn from(raw_instance: RawInstance) -> Self {
        Instance {
            model_matrix: (Matrix4::from_translation(Vector3::from(raw_instance.position))
                * Matrix4::from(Quaternion::from(raw_instance.rotation)))
            .into(),
            color: raw_instance.color,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, bytemuck::Pod, bytemuck::Zeroable)]
pub struct RawInstance {
    pub position: [f32; 3],
    pub rotation: [f32; 4],
    pub color: [f32; 4],
    pub segmatation_id: u32,
    pub captured_time: u32,
    pub capturer_id: u32,
}

impl RawInstance {
    pub fn new(
        &self,
        position: [f32; 3],
        rotation: [f32; 4],
        color: [f32; 4],
        segmatation_id: u32,
        captured_time: u32,
        capturer_id: u32,
    ) -> Self {
        Self {
            position,
            rotation,
            color,
            segmatation_id,
            captured_time,
            capturer_id,
        }
    }
    pub fn set_position<'a>(&'a mut self, position: [f32; 3]) -> &'a mut Self {
        self.position = position;
        self
    }

    pub fn set_rotation<'a>(&'a mut self, rotation: [f32; 4]) -> &'a mut Self {
        self.rotation = rotation;
        self
    }
    pub fn set_color<'a>(&'a mut self, color: [f32; 4]) -> &'a mut Self {
        self.color = color;
        self
    }
    pub fn set_captured_time<'a>(&'a mut self, captured_time: u32) -> &'a mut Self {
        self.captured_time = captured_time;
        self
    }
    pub fn set_capturer_id<'a>(&'a mut self, capturer_id: u32) -> &'a mut Self {
        self.capturer_id = capturer_id;
        self
    }
}
