use wgpu::{
    Backends, BindGroupDescriptor, BindGroupLayoutDescriptor, BindGroupLayoutEntry, Device,
    DeviceDescriptor, Instance, InstanceDescriptor, RequestAdapterOptions, SamplerDescriptor,
};

#[tokio::main]
async fn main() {
    let instance = Instance::new(InstanceDescriptor {
        backends: Backends::all(),
        ..Default::default()
    });

    let adapter = instance
        .request_adapter(&RequestAdapterOptions::default())
        .await
        .unwrap();

    let (device1, queue1) = adapter
        .request_device(
            &DeviceDescriptor {
                label: Some("dev1"),
                ..Default::default()
            },
            None,
        )
        .await
        .unwrap();

    device1.on_uncaptured_error(Box::new(|e| println!("DEV1: {e:?}")));

    let (device2, queue2) = adapter
        .request_device(
            &DeviceDescriptor {
                label: Some("dev2"),
                ..Default::default()
            },
            None,
        )
        .await
        .unwrap();

    device2.on_uncaptured_error(Box::new(|e| println!("DEV2: {e:?}")));

    sampler_mismatch(&device1, &device2);
    println!("-----------");
    tex_mismatch(&device1, &device2);
}

fn tex_mismatch(device1: &Device, device2: &Device) {
    let texture_size = wgpu::Extent3d {
        width: 4,
        height: 4,
        depth_or_array_layers: 1,
    };

    let tex1 = device1.create_texture(&wgpu::TextureDescriptor {
        size: texture_size,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::TEXTURE_BINDING,
        label: None,
        mip_level_count: 1,
        view_formats: &[],
    });

    let tex2 = device2.create_texture(&wgpu::TextureDescriptor {
        size: texture_size,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::TEXTURE_BINDING,
        label: None,
        mip_level_count: 1,
        view_formats: &[],
    });

    let tex1_view = tex1.create_view(&wgpu::TextureViewDescriptor::default());
    let tex2_view = tex2.create_view(&wgpu::TextureViewDescriptor::default());

    let bgl = device1.create_bind_group_layout(&BindGroupLayoutDescriptor {
        label: None,
        entries: &[
            BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    multisampled: false,
                    view_dimension: wgpu::TextureViewDimension::D2,
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                },
                count: None,
            },
            BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    multisampled: false,
                    view_dimension: wgpu::TextureViewDimension::D2,
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                },
                count: None,
            },
        ],
    });

    let bg = device1.create_bind_group(&BindGroupDescriptor {
        label: None,
        layout: &bgl,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&tex1_view),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::TextureView(&tex2_view),
            },
        ],
    });
    tex2.destroy(); // causes Set must be allocated from this allocator
}

fn sampler_mismatch(device1: &Device, device2: &Device) {
    let sampler1 = device1.create_sampler(&SamplerDescriptor {
        ..Default::default()
    });

    let sampler2 = device2.create_sampler(&SamplerDescriptor {
        ..Default::default()
    });

    let bgl = device1.create_bind_group_layout(&BindGroupLayoutDescriptor {
        label: None,
        entries: &[
            BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
            BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
        ],
    });

    let bg = device1.create_bind_group(&BindGroupDescriptor {
        label: None,
        layout: &bgl,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Sampler(&sampler1),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Sampler(&sampler2),
            },
        ],
    });
}
