use std::cmp::Ordering;

use bevy::prelude::*;
use bevy::time;
use rand::prelude::*;

use crate::rigidbody::Accel;
use crate::MAP_HEIGHT;
use crate::MAP_WIDTH;
use crate::food::Food;
use crate::rigidbody::Velocity;

const INITIAL_CREATURE_COUNT: u32 = 10;
const EAT_DISTANCE: f32 = 10.;

const MIN_CREATURE_ACCEL: f32 = 10.;
const MAX_CREATURE_ACCEL: f32 = 100.;
const MIN_CREATURE_ENERGY: f32 = 3200.;
const MAX_CREATURE_ENERGY: f32 = 6400.;

pub struct CreaturePlugin;

//Contient tout ce qui est associé au concept de créature
impl Plugin for CreaturePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_initial_creatures)
            .add_systems(Update, path_find_to_nearest_food)
            .add_systems(Update, eat_food)
            .add_systems(Update, get_tired);
    }
}

#[derive(Component, Debug, Clone)]
#[require(Transform, Velocity)] //Inclu forcément Transform::Default quand on crée le component Creature. On peut l'override
struct Creature {
    energy: f32,
    accel: f32,
}

fn spawn_initial_creatures(mut commands: Commands, asset_server: Res<AssetServer>) {
    let image = asset_server.load("sprites/creature.png");

    let sprite = Sprite { image, ..default() };

    let mut rng = rand::rng();
    for _ in 0..INITIAL_CREATURE_COUNT {
        let x = rng.random_range(-MAP_WIDTH / 2..MAP_WIDTH / 2);
        let y = rng.random_range(-MAP_HEIGHT / 2..MAP_HEIGHT / 2);

        //Quand on veut créer une entité, on utilise une commande
        //On précise tous les components de cette entité
        commands.spawn((
            Creature {
                accel: rng.random_range(0.0..1.0),
                energy: rng.random_range(0.0..1.0),
            },
            Transform::from_xyz(x as f32, y as f32, 0.0),
            sprite.clone(),
        ));
    }
}

fn path_find_to_nearest_food(
    mut creature_query: Query<(&Transform, &mut Accel, &mut Creature)>,
    food_query: Query<&Transform, With<Food>>
) {
    for (transform, mut accel, creature) in creature_query.iter_mut() {
        let Some(closest) = food_query.into_iter().min_by(|lhs, rhs| {
            order_float(
                lhs.translation.distance(transform.translation),
                rhs.translation.distance(transform.translation),
            )
        }) else {
            break;
        };

        let new_accel = MIN_CREATURE_ACCEL.lerp(MAX_CREATURE_ACCEL, creature.accel);

        accel.0 = (closest.translation - transform.translation)
            .normalize_or_zero()
            .xy()
            * new_accel;
    }
}

fn eat_food(
    food_query: Query<(&Transform, &Food, Entity)>,
    mut creature_query: Query<(&Transform, &mut Creature)>,
    mut commands: Commands,
) {
    for (food_transform, food, food_entity) in food_query.iter() {
        let Some((creature_transform, mut closest_creature)) =
            creature_query.iter_mut().min_by(|lhs, rhs| {
                order_float(
                    lhs.0.translation.distance(food_transform.translation),
                    rhs.0.translation.distance(food_transform.translation),
                )
            })
        else {
            break;
        };
        if creature_transform
            .translation
            .distance(food_transform.translation)
            > EAT_DISTANCE
        {
            continue;
        }

        closest_creature.energy = (closest_creature.energy + food.energy).clamp(0., 1.);
        commands.entity(food_entity).despawn();
    }
}

fn get_tired(
    mut creature_query: Query<(&mut Creature, &Velocity, Entity)>,
    mut commands: Commands,
    time: Res<time::Time>,
) {
    for (mut creature, velocity, creature_entity) in creature_query.iter_mut() {
        creature.energy -= velocity.0.length() * time.delta_secs();
        if creature.energy <= 0. {
            commands.entity(creature_entity).despawn();
        }
    }
}


fn order_float(lhs: f32, rhs: f32) -> Ordering {
    match (lhs, rhs) {
        _ if lhs < rhs => Ordering::Less,
        _ if lhs > rhs => Ordering::Greater,
        _ => Ordering::Equal,
    }
}
