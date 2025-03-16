use bevy::prelude::*;

mod creature;
mod creature_state;
mod food;
mod player_camera;
mod rigidbody;
mod simulation_speed;
mod collider;

use creature::CreaturePlugin;
use creature_state::CreatureStatePlugin;
use food::FoodPlugin;
use player_camera::PlayerCameraPlugin;
use rigidbody::RigidbodyPlugin;
use simulation_speed::SimulationSpeedPlugin;
use collider::ColliderPlugin;

const MAP_WIDTH: i32 = 5000;
const MAP_HEIGHT: i32 = 3000;

fn main() -> AppExit {
    App::new()
        .add_plugins((
            DefaultPlugins,
            CreaturePlugin,
            CreatureStatePlugin,
            SimulationSpeedPlugin,
            PlayerCameraPlugin,
            FoodPlugin,
            RigidbodyPlugin,
            ColliderPlugin,
        ))
        .run()
}
