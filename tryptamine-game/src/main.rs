use bevy::{
    input::keyboard::KeyboardInput, prelude::*, sprite::Anchor, sprite_render::Material2dPlugin,
};
use bevy_prng::WyRand;
use bevy_rand::{global::GlobalRng, plugin::EntropyPlugin};
use rand_core::Rng;

use crate::fractal::{FractalMaterial, INITIAL_FRACTAL};
mod fractal;

#[derive(Resource)]
struct ObstacleSpawningTimer(Timer);
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EntropyPlugin::<WyRand>::default())
        .add_plugins(Material2dPlugin::<FractalMaterial>::default())
        .add_systems(Startup, setup)
        .add_systems(Update, (jump, apply_gravity, player_movement))
        .add_systems(
            Update,
            (
                spawn_obstacles,
                move_obstacles,
                // detect_collision,
                // render_health_info,
                // check_health,
            ),
        )
        .insert_resource(ObstacleSpawningTimer(Timer::from_seconds(
            SPAWN_INTERVAL,
            TimerMode::Repeating,
        )))
        .run();
}
const GROUND_LEVEL: f32 = -100.0;
const PLAYER_X: f32 = -300.0;
const GROUND_EDGE: f32 = 400.0;
const GAME_SPEED: f32 = 500.0;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Velocity(Vec3);

fn setup(
    mut commands: Commands,

    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<FractalMaterial>>,
) {
    commands.spawn(Camera2d::default());

    // Player
    commands.spawn((
        Player,
        Mesh2d(meshes.add(Circle::new(50.0))),
        MeshMaterial2d(materials.add(INITIAL_FRACTAL)),
        Anchor::BOTTOM_CENTER,
        Transform::from_xyz(PLAYER_X, GROUND_LEVEL, 0.0),
        Velocity(Vec3::ZERO),
    ));

    // Ground
    commands.spawn((
        Sprite {
            color: Color::srgb(0.5, 0.5, 0.5),
            custom_size: Some(Vec2::new(2.0 * GROUND_EDGE, 10.0)),
            ..default()
        },
        Anchor::TOP_LEFT,
        Transform::from_xyz(-GROUND_EDGE, GROUND_LEVEL, 0.0),
    ));
}

const JUMP_IMPULSE: f32 = 600.0;
fn jump(
    mut msgs: MessageReader<KeyboardInput>,
    mut query: Query<(&mut Velocity, &Transform), With<Player>>,
) {
    for e in msgs.read() {
        if e.state.is_pressed()
            && e.key_code == KeyCode::Space
            && let Ok((mut velocity, transform)) = query.single_mut()
            && transform.translation.y <= GROUND_LEVEL
        {
            velocity.0.y = JUMP_IMPULSE;
        }
    }
}

fn player_movement(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Velocity), With<Player>>,
) {
    for (mut transform, mut velocity) in query.iter_mut() {
        transform.translation.y += velocity.0.y * time.delta_secs();
        if transform.translation.y <= GROUND_LEVEL {
            transform.translation.y = GROUND_LEVEL;
            velocity.0.y = 0.0;
        }
    }
}

const GRAVITY: f32 = -800.0;
fn apply_gravity(time: Res<Time>, mut query: Query<&mut Velocity, With<Player>>) {
    for mut velocity in query.iter_mut() {
        velocity.0.y += GRAVITY * time.delta_secs();
    }
}
const OBSTACLE_COLOR: Color = Color::srgb(1.0, 0.5, 0.5);
const SPAWN_INTERVAL: f32 = 1.0;
#[derive(Component)]
struct Obstacle;
fn spawn_obstacles(
    mut commands: Commands,
    time: Res<Time>,
    mut spawn_timer: ResMut<ObstacleSpawningTimer>,
    mut rng: Single<&mut WyRand, With<GlobalRng>>,
) {
    spawn_timer.0.tick(time.delta());
    if spawn_timer.0.is_finished() {
        let obstacle_x = GROUND_EDGE;
        let obstacle_y = GROUND_LEVEL + (rng.next_u32() % 50) as f32;
        commands.spawn((
            Obstacle,
            Sprite {
                color: OBSTACLE_COLOR,
                custom_size: Some(Vec2::new(30.0, 50.0)),
                ..default()
            },
            Anchor::BOTTOM_CENTER,
            Transform::from_xyz(obstacle_x, obstacle_y, 0.0),
        ));
    }
}
fn move_obstacles(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform), With<Obstacle>>,
) {
    for (entity, mut transform) in query.iter_mut() {
        transform.translation.x -= GAME_SPEED * time.delta_secs();

        // Remove obstacles once they're off-screen
        if transform.translation.x < -GROUND_EDGE {
            commands.entity(entity).despawn();
        }
    }
}
