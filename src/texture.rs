// Wireframe texture
use std::num::NonZeroU32;

// use anyhow::*;
// use image::GenericImageView;
// use image::*;
// use multiarray::*;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ColorCube (pub [[u8; 4]; 24]);

impl ColorCube {
    pub fn new() -> Self {
        let colors: [[u8; 4]; 6] = [
            [255,   0, 0, 0], [0, 255,   0, 0], [0,   0, 255, 0], 
            [255, 255, 0, 0], [255, 0, 255, 0], [0, 255, 255, 0], 
        ];

        // println!("colors = {:?}", colors);
        let mut color2: [[u8; 4]; 4] = [[0; 4]; 4];
        color2[0] = colors[0];
        color2[1] = colors[0];
        color2[2] = colors[1];
        color2[3] = colors[1];
        // println!("color2 = {:?}", color2);

        // let mut color_cube: [[[[f32; 4]; 2]; 2]; 6] = [[[[0.0; 4]; 2]; 2]; 6];
        let mut color_cube: [[u8; 4]; 24] = [[0; 4]; 24];
        // let mut color_cube = Array4D::new([4, 2, 2, 6], 0.0);
        for face in 0..6 {
            for corner in 0..4 {
                color_cube[4 * face + corner] = colors[face];
            }
        }
        Self (color_cube)
    }
    pub fn extents(&self) -> [u32; 3] { [2, 2, 6] }
}

// Texture with a bind group
pub struct Texture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
    pub bind_layout: wgpu::BindGroupLayout,
    pub bind: wgpu::BindGroup,
}

impl Texture {
    // pub fn from_bytes(
    //     device: &wgpu::Device,
    //     queue: &wgpu::Queue,
    //     bytes: &[u8],
    //     label: &str,
    // ) -> Result<Self> {
    //     let img = image::load_from_memory(bytes)?;
    //     Self::from_image(device, queue, &img, label)
    // }

    pub fn mk_cube(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        // img: &image::DynamicImage,
        color_cube: ColorCube,
        label: &str,
    ) -> Self {
        // println!("{:?}", color_cube);

        // let rgba = img.to_rgba8();
        let dimensions = color_cube.extents();

        let size = wgpu::Extent3d {
            width: dimensions[0],
            height: dimensions[1],
            depth_or_array_layers: dimensions[2],
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            // All textures are stored as 3D, we represent our 2D texture
            // by setting depth to 1.
            size,
            mip_level_count: 1, // We'll talk about this a little later
            sample_count: 1,
            // dimension: wgpu::TextureDimension::D2,
            dimension: wgpu::TextureDimension::D2,
            // Most images are stored using sRGB so we need to reflect that here.
            // format: wgpu::TextureFormat::Rgba8UnormSrgb,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            // TEXTURE_BINDING tells wgpu that we want to use this texture in shaders
            // COPY_DST means that we want to copy data to this texture
            usage: wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::COPY_DST,
            label: Some(label),
            // This is the same as with the SurfaceConfig. It
            // specifies what texture formats can be used to
            // create TextureViews for this texture. The base
            // texture format (Rgba8UnormSrgb in this case) is
            // always supported. Note that using a different
            // texture format is not supported on the WebGL2
            // backend.
            view_formats: &[],
        });


        queue.write_texture(
            wgpu::ImageCopyTexture {
                aspect: wgpu::TextureAspect::All,
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            bytemuck::cast_slice(&[color_cube]),
            wgpu::ImageDataLayout {
                offset: 0,
                // bytes_per_row: NonZeroU32::new(4 * dimensions[0]),
                // rows_per_image: NonZeroU32::new(dimensions[1]),
                bytes_per_row: Some(4 * dimensions[0]),
                rows_per_image: Some(dimensions[1]),
            },
            size,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor{
            label: Some("texture::Texture::mk_cube TextureViewDescriptor"),
            format: Some(wgpu::TextureFormat::Rgba8UnormSrgb),
            dimension: Some(wgpu::TextureViewDimension::Cube),
            aspect: wgpu::TextureAspect::All,
            base_mip_level: 0,
            mip_level_count: None,
            base_array_layer: 0,
            array_layer_count: Some(6),
        });
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });

        let mut bgl_label = String::from(label);
        bgl_label.push_str(" bind_group_layout");

        let bind_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::Cube,
                            sample_type: wgpu::TextureSampleType::Float {
                                filterable: true },
                                // filterable: false },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(
                            wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
                label: Some(&bgl_label),
            });
        

        let mut bg_label = String::from(label);
        bg_label.push_str(" bind_group");

        let bind = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
            label: Some(&bg_label),
        });
        Self {
            texture,
            view,
            sampler,
            bind_layout,
            bind
        }
    }
}

//  Depth texture.
pub struct Depth {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
}

impl Depth {
    pub const DEPTH_FORMAT: wgpu::TextureFormat =
        wgpu::TextureFormat::Depth32Float; // 1.
    
    pub fn create(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        label: &str
    ) -> Self {
        let size = wgpu::Extent3d { // 2.
            width: config.width,
            height: config.height,
            depth_or_array_layers: 1,
        };
        let desc = wgpu::TextureDescriptor {
            label: Some(label),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: Self::DEPTH_FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT // 3.
                | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        };
        let texture = device.create_texture(&desc);

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(
            &wgpu::SamplerDescriptor { // 4.
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Linear,
                mipmap_filter: wgpu::FilterMode::Nearest,
                compare: Some(wgpu::CompareFunction::LessEqual), // 5.
                lod_min_clamp: 0.0,
                lod_max_clamp: 100.0,
                ..Default::default()
            }
        );

        Self { texture, view, sampler }
    }
}

