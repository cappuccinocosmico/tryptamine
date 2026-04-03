use std::ops::{Add, Mul};

use bevy::{
    asset::Asset,
    math::{Vec2, vec2},
    reflect::TypePath,
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
    sprite_render::Material2d,
};

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[cfg_attr(
    feature = "zerocopy",
    derive(FromBytes, Immutable, IntoBytes, KnownLayout)
)]
struct Complex {
    re: f32,
    im: f32,
}

impl Add for Complex {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

impl Mul for Complex {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Complex {
            re: self.re * rhs.re - self.im * rhs.im,
            im: self.im * rhs.re + self.re * rhs.im,
        }
    }
}

// Your material struct will look like:
#[derive(AsBindGroup, Asset, TypePath, Clone)]
pub struct FractalMaterial {
    #[uniform(0)]
    pub c: Vec2,
    #[uniform(0)]
    pub center: Vec2,
    #[uniform(0)]
    pub view_radius: f32,
    #[uniform(0)]
    pub unit_interval_resolution: f32,
}

pub const INITIAL_FRACTAL: FractalMaterial = FractalMaterial {
    c: vec2(1.0, 0.5),
    center: vec2(0.0, 0.0),
    view_radius: 3.0,
    unit_interval_resolution: 100.0, // Idk what this should be lmao
};
impl Material2d for FractalMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/fractal.wgsl".into()
    }
}
