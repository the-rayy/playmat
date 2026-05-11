type Mat4 = [f32; 16];

pub fn mat4_mul(a: Mat4, b: Mat4) -> Mat4 {
    let mut out = [0f32; 16];
    for row in 0..4 {
        for col in 0..4 {
            out[row*4+col] =
                a[row*4+0] * b[0*4+col] +
                a[row*4+1] * b[1*4+col] +
                a[row*4+2] * b[2*4+col] +
                a[row*4+3] * b[3*4+col];
        }
    }
    out
}

pub fn rotation_y(a: f32) -> Mat4 {
    let (s, c) = a.sin_cos();
    [ c,  0., s, 0.,
      0., 1., 0., 0.,
     -s,  0., c, 0.,
      0., 0., 0., 1.]
}

pub fn rotation_x(a: f32) -> Mat4 {
    let (s, c) = a.sin_cos();
    [1., 0.,  0., 0.,
     0., c,  -s,  0.,
     0., s,   c,  0.,
     0., 0.,  0., 1.]
}

pub fn translation(x: f32, y: f32, z: f32) -> Mat4 {
    [1., 0., 0., 0.,
     0., 1., 0., 0.,
     0., 0., 1., 0.,
      x,  y,  z, 1.]
}

pub fn perspective(fov_y: f32, aspect: f32, near: f32, far: f32) -> Mat4 {
    let f = 1.0 / (fov_y / 2.0).tan();
    let r = near - far;
    [f/aspect, 0.,              0.,  0.,
           0.,  f,              0.,  0.,
           0., 0., (far+near)/r, -1.,
           0., 0., (2.*far*near)/r, 0.]
}

pub fn compute_mvp(angle: f32, aspect: f32) -> Mat4 {
    let model = mat4_mul(
        rotation_y(angle),
        rotation_x(0.4),
    );
    let view = translation(0., 0., -3.);   // camera pulled back on Z
    let proj = perspective(
        std::f32::consts::FRAC_PI_4,
        aspect,
        0.1,
        100.0,
    );
    mat4_mul(proj, mat4_mul(view, model))
}
