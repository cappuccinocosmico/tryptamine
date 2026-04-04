struct SceneUniforms {
    c: vec2<f32>,
    center: vec2<f32>,
    view_radius: f32,
    aspect_ratio: f32,
};

@group(0) @binding(0) var<uniform> scene: SceneUniforms;

@fragment
fn fs_main(@location(0) uv: vec2<f32>) -> @location(0) vec4<f32> {
    let fractal_pos = derive_position(uv);
    let iterations = perform_fractal_iterations(fractal_pos);
    return generate_fractal_color(iterations);
}

fn derive_position(uv: vec2<f32>) -> vec2<f32> {
    let centered = uv * 2.0 - 1.0;
    let aspect_corrected = vec2<f32>(centered.x * scene.aspect_ratio, centered.y);
    return scene.center + scene.view_radius * aspect_corrected;
}

fn generate_fractal_color(iterations: i32) -> vec4<f32> {
    if iterations == -1 {
        return vec4<f32>(0.0, 0.0, 0.0, 1.0);
    }
    let colorscheme = array<vec4<f32>, 10>(
        vec4<f32>(0.514, 0.227, 0.706, 1.0),
        vec4<f32>(0.633, 0.199, 0.558, 1.0),
        vec4<f32>(0.753, 0.171, 0.410, 1.0),
        vec4<f32>(0.873, 0.142, 0.262, 1.0),
        vec4<f32>(0.992, 0.114, 0.114, 1.0),
        vec4<f32>(0.991, 0.258, 0.153, 1.0),
        vec4<f32>(0.990, 0.402, 0.192, 1.0),
        vec4<f32>(0.990, 0.546, 0.231, 1.0),
        vec4<f32>(0.989, 0.690, 0.271, 1.0),
        vec4<f32>(0.988, 0.750, 0.300, 1.0)
    );
    let index = u32(iterations % 10);
    return colorscheme[index];
}

fn perform_fractal_iterations(init_pos: vec2<f32>) -> i32 {
    var mut_pos: vec2<f32> = init_pos;
    for (var i = 1; i < 20; i++) {
        mut_pos = iteration(mut_pos);
        if dot(mut_pos, mut_pos) > 9.0 {
            return i;
        }
    }
    return -1;
}

fn iteration(input: vec2<f32>) -> vec2<f32> {
    return cmul(scene.c, csin(input));
}

fn cmul(a: vec2<f32>, b: vec2<f32>) -> vec2<f32> {
    return vec2<f32>(a.x * b.x - a.y * b.y, a.x * b.y + a.y * b.x);
}

fn csin(input: vec2<f32>) -> vec2<f32> {
    return vec2<f32>(sin(input.x) * cosh(input.y), cos(input.x) * sinh(input.y));
}
