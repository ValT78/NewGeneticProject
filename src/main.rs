use bevy::prelude::*;

mod creature;
mod baby_creature;
mod food;
mod player_camera;
mod rigidbody;
mod simulation_speed;
mod collider;

use creature::CreaturePlugin;
use baby_creature::BabyCreaturePlugin;
use food::FoodPlugin;
use player_camera::PlayerCameraPlugin;
use rigidbody::RigidbodyPlugin;
use simulation_speed::SimulationSpeedPlugin;
use collider::ColliderPlugin;

const MAP_WIDTH: i32 = 2500;
const MAP_HEIGHT: i32 = 1500;

fn main() -> AppExit {
    App::new()
        .add_plugins((
            DefaultPlugins,
            CreaturePlugin,
            BabyCreaturePlugin,
            SimulationSpeedPlugin,
            PlayerCameraPlugin,
            FoodPlugin,
            RigidbodyPlugin,
            ColliderPlugin,
        ))
        .run()
}
