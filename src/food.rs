use bevy::prelude::*;
use rand::prelude::*;

use crate::MAP_HEIGHT;
use crate::MAP_WIDTH;

const FOOD_SPAWN_INTERVAL: f32 = 0.3;

pub struct FoodPlugin;

#[derive(Component)]
#[require(Transform)]
pub struct Food {
    pub energy: f32,
}

#[derive(Resource)]
struct FoodSpriteHandle(Handle<Image>);

#[derive(Resource)]
struct FoodSpawnTimer(Timer);

impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App) {
        let food_spawn_timer = FoodSpawnTimer(Timer::from_seconds(
            FOOD_SPAWN_INTERVAL,
            TimerMode::Repeating,
        ));

        app.add_systems(Startup, init_food_sprite_handle)
            .add_systems(Update, spawn_foods)
            .insert_resource(food_spawn_timer);
    }
}

fn init_food_sprite_handle(mut commands: Commands, asset_server: Res<AssetServer>) {
    let image = asset_server.load("sprites/food.png");
    commands.insert_resource(FoodSpriteHandle(image));
}

fn spawn_foods(
    mut commands: Commands,
    time: Res<Time>,
    food_sprite_handle: Res<FoodSpriteHandle>,
    mut timer: ResMut<FoodSpawnTimer>,
) {
    timer.0.tick(time.delta());
    if !timer.0.finished() {
        return;
    }
    let mut rng = rand::rng();
    let x = rng.random_range(-MAP_WIDTH / 2..MAP_WIDTH / 2);
    let y = rng.random_range(-MAP_HEIGHT / 2..MAP_HEIGHT / 2);

    let sprite = Sprite {
        image: food_sprite_handle.0.clone(),
        ..default()
    };
    commands.spawn((
        Food { energy: 80. },
        Transform::from_xyz(x as f32, y as f32, 0.0),
        sprite,
    ));
}
