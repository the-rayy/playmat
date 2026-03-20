pub fn descriptor() -> wgpu::InstanceDescriptor {
  wgpu::InstanceDescriptor {
    backends: wgpu::Backends::PRIMARY,
    ..Default::default()
  }
}
