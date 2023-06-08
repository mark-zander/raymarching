use wgpu::util::DeviceExt;
use crate::cli;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct DivSize(pub [f32; 2]);

impl DivSize {
    pub fn new(x: f32, y: f32) -> Self { Self{ 0: [x, y] } }
    pub fn unwrap(&self) -> [f32; 2] { self.0 }
}

pub struct Data {
    pub div_size: DivSize,
    pub layout: wgpu::BindGroupLayout,
    pub bind: wgpu::BindGroup,
}

impl Data {
    pub fn new(
        size: winit::dpi::PhysicalSize<u32>,
        device: &wgpu::Device,
    ) -> Self {
        let div_size = DivSize::new(
            2.0 / size.width as f32,
            -2.0 / size.height as f32,
        );

        let layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: Some("mesh::Data bind_group_layout"),
        });

        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("mesh::Data buffer"),
            contents: bytemuck::cast_slice(&[div_size]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
            label: Some("mesh::Data bind_group"),
        });
        Self {
            div_size,
            layout,
            bind,
        }
    }
}
