use bevy::{
    asset::{Asset, Handle},
    ecs::resource::Resource,
    math::{Vec2, vec2},
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderType},
    shader::ShaderRef,
    sprite_render::Material2d,
};

use crate::colors::RgbWrapper;
// TODO: I am wanting to move to having the uniforms passed around in these types:

// Not quite sure how to implement this, probably have each one of these
pub enum FractalAlgorithm {
    SinJulia(Vec2),
    SquareMandelbrot,
    SquareJulia(Vec2),
}

pub struct FractalData {
    pub algo: FractalAlgorithm,
    pub center: Vec2,
    pub view_radius: Vec2,
}

pub struct CanvasData {
    pub aspect_ratio: f32,
    pub screen_size: Vec2,
}
// These should run as a compute shader, and output a canvas of the following datatype, regardless
// of what fractal algorithm was chosen.
#[repr(C)]
pub struct FractalPixelOutput {
    pub basin_identifier: i32, // Gives you the basin id, or is equal to -1 if it did not converge.
    pub iterations: u32,       // Number of iterations it took before it fell into a basin.
}

pub struct ColorData {
    pub trapped_color: RgbWrapper, // Vec4, has a from implementation for this type.
    pub basin_0_colors: Vec<RgbWrapper>, // Typically the infinite basin will be the zero basin. You can add more colors to the cyclic wheel if you want the pattern to repeat less often.
}

// As opposed to these legacy types.

#[derive(ShaderType, Clone, Copy)]
pub struct FractalUniform {
    pub c: Vec2,
    pub center: Vec2,
    pub view_radius: f32,
    pub aspect_ratio: f32,
    pub screen_size: Vec2,
}
impl<'a> From<&'a FractalMaterial> for FractalUniform {
    fn from(value: &'a FractalMaterial) -> Self {
        Self {
            c: value.c,
            center: value.center,
            view_radius: value.view_radius,
            aspect_ratio: 16.0 / 9.0,
            screen_size: vec2(1920.0, 1080.0),
        }
    }
}

#[derive(Asset, AsBindGroup, TypePath, Clone, Copy)]
#[uniform(0, FractalUniform)]
pub struct FractalMaterial {
    pub c: Vec2,
    pub center: Vec2,
    pub view_radius: f32,
}

#[derive(Resource)]
pub struct FractalHandle(pub Handle<FractalMaterial>);

pub const INITIAL_FRACTAL: FractalMaterial = FractalMaterial {
    c: vec2(1.0, 0.5),
    center: vec2(0.0, 0.0),
    view_radius: 3.0,
};

impl Material2d for FractalMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/fractal.wgsl".into()
    }
}
