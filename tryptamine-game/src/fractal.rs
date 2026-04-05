use bevy::{
    asset::{Asset, Handle},
    ecs::resource::Resource,
    math::{vec2, Vec2},
    reflect::TypePath,
    render::{
        render_asset::RenderAssets,
        render_resource::{AsBindGroup, AsBindGroupShaderType, ShaderType},
        texture::GpuImage,
    },
    shader::ShaderRef,
    sprite_render::Material2d,
};

#[derive(Asset, AsBindGroup, TypePath, Clone)]
#[uniform(0, FractalUniform)]
pub struct FractalMaterial {
    pub c: Vec2,
    pub center: Vec2,
    pub view_radius: f32,
    pub aspect_ratio: f32,
    pub screen_size: Vec2,
}

#[derive(Clone, ShaderType)]
pub struct FractalUniform {
    pub c: Vec2,
    pub center: Vec2,
    pub view_radius: f32,
    pub aspect_ratio: f32,
    pub screen_size: Vec2,
}

impl AsBindGroupShaderType<FractalUniform> for FractalMaterial {
    fn as_bind_group_shader_type(&self, _images: &RenderAssets<GpuImage>) -> FractalUniform {
        FractalUniform {
            c: self.c,
            center: self.center,
            view_radius: self.view_radius,
            aspect_ratio: self.aspect_ratio,
            screen_size: self.screen_size,
        }
    }
}

#[derive(Resource)]
pub struct FractalHandle(pub Handle<FractalMaterial>);

pub const INITIAL_FRACTAL: FractalMaterial = FractalMaterial {
    c: vec2(1.0, 0.5),
    center: vec2(0.0, 0.0),
    view_radius: 3.0,
    aspect_ratio: 16.0 / 9.0,
    screen_size: vec2(1920.0, 1080.0),
};

impl Material2d for FractalMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/fractal.wgsl".into()
    }
}
