use bevy::prelude::*;

mod creature;
mod simulation_speed;
mod player_camera;

use creature::CreaturePlugin;
use simulation_speed::SimulationSpeedPlugin;
use player_camera::PlayerCameraPlugin;

fn main() -> AppExit{
    App::new()
    .add_plugins((DefaultPlugins, CreaturePlugin, SimulationSpeedPlugin, PlayerCameraPlugin))
    .run()
}
