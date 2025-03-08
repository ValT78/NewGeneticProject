use bevy::prelude::*;

pub struct FoodPlugin;

impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App) {
        
    }
}

#[derive(Component)]
#[require(Transform)]
pub struct Food {
    pub energy: f32,
}