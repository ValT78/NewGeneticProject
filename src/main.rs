use bevy::prelude::*;

mod creature;
mod simulation_speed;
mod player_camera;
mod food;

use creature::CreaturePlugin;
use simulation_speed::SimulationSpeedPlugin;
use player_camera::PlayerCameraPlugin;
use food::FoodPlugin;

const MAP_WIDTH: i32 = 256;
const MAP_HEIGHT: i32 = 256;

fn main() -> AppExit{
    App::new()
    .add_plugins((DefaultPlugins, CreaturePlugin, SimulationSpeedPlugin, PlayerCameraPlugin, FoodPlugin))
    .run()
}
