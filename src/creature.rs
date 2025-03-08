use bevy::prelude::*;
use rand::prelude::*;

use crate::MAP_WIDTH;
use crate::MAP_HEIGHT;

pub struct CreaturePlugin;

const INITIAL_CREATURE_COUNT: u32 = 10;
//Contient tout ce qui est associé au concept de créature
impl Plugin for CreaturePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_initial_creatures);
    }
}

#[derive(Component, Debug, Clone)]
#[require(Transform)]    //Inclu forcément Transform::Default quand on crée le component Creature. On peut l'override
struct Creature;

fn spawn_initial_creatures(mut commands: Commands, asset_server: Res<AssetServer>) {
    let image = asset_server.load("sprites/creature.png");
    
    let sprite = Sprite {
        image,
        ..default()
    };

    let mut rng = rand::rng();
    for _ in 0..INITIAL_CREATURE_COUNT {
        let x = rng.random_range(-MAP_WIDTH/2..MAP_WIDTH/2);
        let y = rng.random_range(-MAP_HEIGHT/2..MAP_HEIGHT/2);

        //Quand on veut créer une entité, on utilise une commande
        //On précise tous les components de cette entité
        commands.spawn((Creature, Transform::from_xyz(x as f32, y as f32, 0.0), sprite.clone()));
    }
}