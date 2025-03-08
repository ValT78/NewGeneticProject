use bevy::prelude::*;

pub struct SimulationSpeedPlugin;

impl Plugin for SimulationSpeedPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SimulationSpeed(1.0))
            .add_systems(Update, adjust_simulation_speed);
    }
    
}

#[derive(Resource)]
pub struct  SimulationSpeed(pub f32);

pub fn adjust_simulation_speed(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut time: ResMut<Time<Virtual>>,
    mut speed: ResMut<SimulationSpeed>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyE) {
        speed.0 *= 2.0;
    }
    if keyboard_input.just_pressed(KeyCode::KeyQ) {
        speed.0 /= 2.0;
    }
    time.set_relative_speed(speed.0);

}


