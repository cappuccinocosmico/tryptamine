struct SceneUniforms {
    c: vec2<f32>,
    center: vec2<f32>,
    view_radius: f32,
    unit_interval_resolution: f32,
};

@group(0) @binding(0) var<uniform> scene: SceneUniforms;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) fractal_pos: vec2<f32>,
}
fn derive_position(pos: vec2<f32>) -> vec2<f32> {
    return vec2<f32>(scene.center.x + scene.view_radius * pos.x, scene.center.y + scene.view_radius * pos.y);
}
@vertex
fn vs_main(@builtin(vertex_index) vertex_idx: u32) -> VertexOutput {
    var out: VertexOutput;

    // Generate a triangle that covers the full screen:
    // Index 0: (-1, -1), Index 1: (3, -1), Index 2: (-1, 3)
    let x = f32((i32(vertex_idx) << 1u) & 2) - 1.0;
    let y = f32(i32(vertex_idx) & 2) - 1.0;

    out.position = vec4<f32>(x, y, 0.0, 1.0);

    // Pass the raw coordinate (-1.0 to 1.0 range) to the position derivation, then compute it out.
    out.fractal_pos = derive_position(vec2<f32>(x, y));

    return out;
};
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let iterations = perform_fractal_iterations(in.fractal_pos);
    return generate_fractal_color(iterations);
};

fn generate_fractal_color(iterations: i32) -> vec4<f32> {
    if iterations == -1 {
        return vec4<f32>(0.0, 0.0, 0.0, 1.0);
    }
    // take this css gradient
    // background: linear-gradient(90deg,rgba(131, 58, 180, 1) 0%, rgba(253, 29, 29, 1) 50%, rgba(252, 176, 69, 1) 100%);
    // And generate 10 colors out of it, then return the color: colorscheme[iterations % 10]
    // LLM:
    // Normalized rgba(131, 58, 180) -> rgba(253, 29, 29) -> rgba(252, 176, 69)
    let colorscheme = array<vec4<f32>, 10>(
        vec4<f32>(0.514, 0.227, 0.706, 1.0), // 0%
        vec4<f32>(0.633, 0.199, 0.558, 1.0),
        vec4<f32>(0.753, 0.171, 0.410, 1.0),
        vec4<f32>(0.873, 0.142, 0.262, 1.0),
        vec4<f32>(0.992, 0.114, 0.114, 1.0), // ~50%
        vec4<f32>(0.991, 0.258, 0.153, 1.0),
        vec4<f32>(0.990, 0.402, 0.192, 1.0),
        vec4<f32>(0.990, 0.546, 0.231, 1.0),
        vec4<f32>(0.989, 0.690, 0.271, 1.0), // ~90%
        vec4<f32>(0.988, 0.750, 0.300, 1.0)// 100% (Approximated)
    );

    // Using u32 cast for array indexing and ensuring positive index
    let index = u32(iterations % 10);
    return colorscheme[index];
};

fn perform_fractal_iterations(init_pos: vec2<f32>) -> i32 {
    var mut_pos: vec2<f32> = init_pos;
    for (var i = 1; i < 20; i++) {
        mut_pos = iteration(mut_pos);
        if dot(mut_pos, mut_pos) > 9.0 {
            return i;
        }
    };
    return -1;
};

fn iteration(input: vec2<f32>) -> vec2<f32> {
    return cmul(scene.c, csin(input));
};

fn cmul(a: vec2<f32>, b: vec2<f32>) -> vec2<f32> {
    return vec2<f32>(a.x * b.x - a.y * b.y, a.x * b.y + a.y * b.x);
}

fn csin(input: vec2<f32>) -> vec2<f32> {
    return vec2<f32>(sin(input.x) * cosh(input.y), cos(input.x) * sinh(input.y));
}

// There are probably much better ways to do this lmao, both with inbuilt tools, and also computationally.
// Nevermind, they have inbuilts that work even better
fn dirty_sin(val: f32) -> f32 {
    return val - val * val * val / 6.0 + val * val * val * val * val / 120.0;
}
fn dirty_sinh(val: f32) -> f32 {
    return val + val * val * val / 6.0 + val * val * val * val * val / 120.0;
}

fn dirty_cos(val: f32) -> f32 {
    return 1 - val * val / 2.0 + val * val * val * val / 24.0;
}
fn dirty_cosh(val: f32) -> f32 {
    return 1 + val * val / 2.0 + val * val * val * val / 24.0;
}

fn dirty_exp(val: f32) -> f32 {
    return 1 + val + val * val / 2.0 + val * val * val / 6.0 + val * val * val * val / 24.0 + val * val * val * val * val / 120.0;
}
