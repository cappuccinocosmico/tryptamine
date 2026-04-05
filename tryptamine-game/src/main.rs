use std::time::Duration;

use bevy::{input::keyboard::KeyboardInput, prelude::*, sprite_render::Material2dPlugin};

use crate::fractal::{FractalHandle, FractalMaterial, INITIAL_FRACTAL};
mod fractal;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(Material2dPlugin::<FractalMaterial>::default())
        .add_systems(Startup, setup)
        .add_systems(Update, fractal_controls)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<FractalMaterial>>,
) {
    commands.spawn(Camera2d::default());

    let handle = materials.add(INITIAL_FRACTAL);
    commands.insert_resource(FractalHandle(handle.clone()));

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(2.0, 2.0))),
        MeshMaterial2d(handle),
        Transform::from_scale(Vec3::new(1000.0, 1000.0, 1.0)),
    ));
}
fn fractal_controls(
    mut events: MessageReader<KeyboardInput>,
    mut materials: ResMut<Assets<FractalMaterial>>,
    time: Res<Time>,
    handle: Res<FractalHandle>,
) {
    for e in events.read() {
        if e.state.is_pressed()
            && let Some(direction) = MovementDirection::from_keycode(&e.key_code)
            && let Some(mat) = materials.get_mut(&handle.0)
        {
            direction.move_fractal(mat, &time.delta());
        }
    }
}

enum MovementDirection {
    Up,
    Down,
    Left,
    Right,
    ZoomIn,
    ZoomOut,
}
impl MovementDirection {
    fn from_keycode(keycode: &KeyCode) -> Option<Self> {
        match keycode {
            KeyCode::KeyW => Some(Self::Up),
            KeyCode::KeyA => Some(Self::Left),
            KeyCode::KeyS => Some(Self::Down),
            KeyCode::KeyD => Some(Self::Right),
            KeyCode::KeyE => Some(Self::ZoomIn),
            KeyCode::KeyR => Some(Self::ZoomOut),
            _ => None,
        }
    }
    fn move_fractal(&self, fractal_data: &mut FractalMaterial, delta_time: &Duration) {
        const SPEED: f32 = 1.0;
        let move_displacement = 0.2 * SPEED * fractal_data.view_radius * delta_time.as_secs_f32();
        let move_scale: f32 = 1.1 * SPEED * delta_time.as_secs_f32();
        match self {
            Self::Up => fractal_data.center.y += move_displacement,
            Self::Down => fractal_data.center.y -= move_displacement,
            Self::Right => fractal_data.center.x += move_displacement,
            Self::Left => fractal_data.center.x -= move_displacement,
            Self::ZoomOut => fractal_data.view_radius *= move_scale,
            Self::ZoomIn => fractal_data.view_radius /= move_scale,
        }
    }
}
