pub fn descriptor() -> wgpu::InstanceDescriptor {
  wgpu::InstanceDescriptor {
    backends: wgpu::Backends::GL,
    ..Default::default()
  }
}
