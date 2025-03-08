use bevy::prelude::*;

use crate::simulation_speed;

const CAMERA_SPEED: f32 = 500.0;

pub struct PlayerCameraPlugin;

#[derive(Component)]
#[require(Camera2d)]
struct PlayerCamera;

impl Plugin for PlayerCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, move_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(PlayerCamera);
}

fn move_camera(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<PlayerCamera>>,
    simulation_speed: Res<simulation_speed::SimulationSpeed>,
) {
    let mut transform = query.single_mut();
    let mut direction = Vec3::ZERO;

    if keyboard_input.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }

    transform.translation +=
        direction.normalize_or_zero() * CAMERA_SPEED * time.delta_secs() / simulation_speed.0;
}
