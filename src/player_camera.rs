use bevy::{input::mouse::MouseWheel, prelude::*};

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
    mut mouse_wheel_events: EventReader<MouseWheel>, // 🎯 Capture la molette
    time: Res<Time>,
    mut query: Query<&mut Transform, With<PlayerCamera>>,
    simulation_speed: Res<simulation_speed::SimulationSpeed>,
) {
    let mut transform = query.single_mut();
    let mut direction = Vec3::ZERO;

    // 🎮 Déplacement classique (WASD)
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

    // 🖱️ Gestion du zoom
    for event in mouse_wheel_events.read() {
        let zoom_speed = 0.1; // Facteur de zoom
        let min_zoom = 0.5;   // Zoom minimum
        let max_zoom = 3.0;   // Zoom maximum

        if event.y > 0.0 {
            // 🔍 Zoom avant
            transform.scale = (transform.scale * (1.0 - zoom_speed)).max(Vec3::splat(min_zoom));
        } else if event.y < 0.0 {
            // 🔎 Zoom arrière
            transform.scale = (transform.scale * (1.0 + zoom_speed)).min(Vec3::splat(max_zoom));
        }
    }

    // 📏 Ajuster la vitesse en fonction du zoom
    let zoom_factor = transform.scale.x; // La valeur de scale x représente le zoom
    let adjusted_speed = CAMERA_SPEED * zoom_factor; // Vitesse ajustée

    // 🏃‍♂️ Déplacement avec vitesse adaptée
    transform.translation +=
        direction.normalize_or_zero() * adjusted_speed * time.delta_secs() / simulation_speed.0;
}

