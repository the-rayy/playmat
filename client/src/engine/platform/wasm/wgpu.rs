pub fn instance_descriptor() -> wgpu::InstanceDescriptor {
  wgpu::InstanceDescriptor {
    backends: wgpu::Backends::GL,
    ..Default::default()
  }
}

pub fn device_limits() -> wgpu::Limits {
  wgpu::Limits::downlevel_webgl2_defaults()
}
