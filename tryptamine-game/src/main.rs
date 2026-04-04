use bevy::prelude::*;

use crate::fractal::{FractalMaterial, INITIAL_FRACTAL};
mod fractal;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(Material2dPlugin::<FractalMaterial>::default())
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<FractalMaterial>>,
) {
    commands.spawn(Camera2d::default());

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(2.0, 2.0))),
        MeshMaterial2d(materials.add(INITIAL_FRACTAL)),
        Transform::from_scale(Vec3::new(1000.0, 1000.0, 1.0)),
    ));
}
