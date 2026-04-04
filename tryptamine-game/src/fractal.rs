use bevy::{
    asset::Asset,
    math::{vec2, Vec2},
    reflect::TypePath,
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
    sprite_render::Material2d,
};

#[derive(AsBindGroup, Asset, TypePath, Clone)]
pub struct FractalMaterial {
    #[uniform(0)]
    pub c: Vec2,
    #[uniform(0)]
    pub center: Vec2,
    #[uniform(0)]
    pub view_radius: f32,
    #[uniform(0)]
    pub aspect_ratio: f32,
}

pub const INITIAL_FRACTAL: FractalMaterial = FractalMaterial {
    c: vec2(1.0, 0.5),
    center: vec2(0.0, 0.0),
    view_radius: 3.0,
    aspect_ratio: 16.0 / 9.0,
};

impl Material2d for FractalMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/fractal.wgsl".into()
    }
}
