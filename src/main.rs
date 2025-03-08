use bevy::prelude::*;

mod creature;
mod food;
mod player_camera;
mod rigidbody;
mod simulation_speed;

use creature::CreaturePlugin;
use food::FoodPlugin;
use player_camera::PlayerCameraPlugin;
use rigidbody::RigidbodyPlugin;
use simulation_speed::SimulationSpeedPlugin;

const MAP_WIDTH: i32 = 256;
const MAP_HEIGHT: i32 = 256;

fn main() -> AppExit {
    App::new()
        .add_plugins((
            DefaultPlugins,
            CreaturePlugin,
            SimulationSpeedPlugin,
            PlayerCameraPlugin,
            FoodPlugin,
            RigidbodyPlugin,
        ))
        .run()
}
