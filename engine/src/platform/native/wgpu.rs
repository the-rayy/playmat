pub fn instance_descriptor() -> wgpu::InstanceDescriptor {
  wgpu::InstanceDescriptor {
    backends: wgpu::Backends::PRIMARY,
    flags: Default::default(),
    memory_budget_thresholds: Default::default(),
    backend_options: Default::default(),
    display: None,
  }
}

pub fn device_limits() -> wgpu::Limits {
  wgpu::Limits::default()
}
