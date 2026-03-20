pub fn instance_descriptor() -> wgpu::InstanceDescriptor {
  wgpu::InstanceDescriptor {
    backends: wgpu::Backends::PRIMARY,
    ..Default::default()
  }
}

pub fn device_limits() -> wgpu::Limits {
  wgpu::Limits::default()
}
