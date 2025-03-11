use std::cmp::Ordering;

use bevy::prelude::*;
use bevy::time;
use rand::prelude::*;

use crate::collider::Collider;
use crate::rigidbody::Accel;
use crate::MAP_HEIGHT;
use crate::MAP_WIDTH;
use crate::food::Food;
use crate::rigidbody::Velocity;

const INITIAL_CREATURE_COUNT: u32 = 20;
const EAT_DISTANCE: f32 = 10.;

const MIN_CREATURE_ACCEL: f32 = 10.;
const MAX_CREATURE_ACCEL: f32 = 100.;
const MIN_CREATURE_ENERGY: f32 = 3200.;
const MAX_CREATURE_ENERGY: f32 = 6400.;
const MIN_SPAWN_ENERGY_RATE: f32 = 0.1;
const MAX_SPAWN_ENERGY_RATE: f32 = 0.5;
const MIN_SPAWN_SIZE: f32 = 25.;
const MAX_SPAWN_SIZE: f32 = 40.;

const MIN_UPPER_SPEED: f32 = 1.;
const MAX_UPPER_SPEED: f32 = 1000.;

pub struct CreaturePlugin;

//Contient tout ce qui est associé au concept de créature
impl Plugin for CreaturePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_initial_creatures)
            .add_systems(Update, path_find_to_nearest_food)
            .add_systems(Update, eat_food)
            .add_systems(Update, get_tired)
            .add_systems(Update, update_energy_bars);
    }
}

#[derive(Component, Debug, Clone)]
#[require(Transform, Velocity)] //Inclu forcément Transform::Default quand on crée le component Creature. On peut l'override
pub struct Creature {
    energy: f32,
    max_energy: f32,
    accel_factor: f32,
    upper_speed: f32,
}
#[derive(Component, Debug, Clone)]
#[require(Transform)]
pub struct EnergyBar;

fn spawn_creature(
    x: i32, 
    y: i32, 
    accel_factor: f32,
    upper_speed: f32,
    max_energy: f32,
    energy: f32,
    size: f32,
    commands: &mut Commands, 
    sprite: Sprite,
) {
    commands.spawn((
        Creature {
            accel_factor,
            upper_speed,
            max_energy,
            energy,
        },
        Transform::from_xyz(x as f32, y as f32, 0.0),
        Collider { radius: size },
        sprite.clone(),
    )).with_child((
        EnergyBar,
        Sprite {
            color: Color::srgb(0., 0.8, 0.5),
            custom_size: Some(Vec2::new(40.0, 5.0)),
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, 15.0, 1.0),
            ..default()
        },
        ));
}

fn spawn_initial_creatures(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut rng: ThreadRng = rand::rng();

    let sprite = Sprite {
        image: asset_server.load("sprites/creature.png"),
        ..default()
    };

    for _ in 0..INITIAL_CREATURE_COUNT {
        let x = rng.random_range(-MAP_WIDTH / 2..MAP_WIDTH / 2);
        let y = rng.random_range(-MAP_HEIGHT / 2..MAP_HEIGHT / 2);

        let accel_factor = rng.random_range(MIN_CREATURE_ACCEL..MAX_CREATURE_ACCEL);
        let upper_speed = rng.random_range(MIN_UPPER_SPEED..MAX_UPPER_SPEED);
        let max_energy = rng.random_range(MIN_CREATURE_ENERGY..MAX_CREATURE_ENERGY);
        let energy = rng.random_range(MIN_SPAWN_ENERGY_RATE..MAX_SPAWN_ENERGY_RATE) * max_energy;
        let size = rng.random_range(MIN_SPAWN_SIZE..MAX_SPAWN_SIZE);


        spawn_creature(x, y, accel_factor, upper_speed, max_energy, energy, size, &mut commands, sprite.clone());
    }
}

fn path_find_to_nearest_food(
    mut creature_query: Query<(&Transform, &mut Velocity, &mut Accel, &mut Creature)>,
    food_query: Query<&Transform, With<Food>>
) {
    for (transform, mut velocity, mut accel, creature) in creature_query.iter_mut() {
        let Some(closest) = food_query.into_iter().min_by(|lhs, rhs| {
            order_float(
                lhs.translation.distance(transform.translation),
                rhs.translation.distance(transform.translation),
            )
        }) else {
            break;
        };

        // let new_accel = MIN_CREATURE_ACCEL.lerp(MAX_CREATURE_ACCEL, creature.accel);

        accel.0 = (closest.translation - transform.translation)
            .normalize_or_zero()
            .xy()
            * creature.accel_factor;

        if velocity.0.length() > creature.upper_speed {
            velocity.0 = velocity.0.normalize() * creature.upper_speed;
        }
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

        closest_creature.energy = (closest_creature.energy + food.energy).clamp(0., closest_creature.max_energy);
        commands.entity(food_entity).despawn();
    }
}

pub fn update_energy_bars(
    creatures: Query<&Creature, Without<EnergyBar>>, // Accès aux créatures
    mut bars: Query<(&mut Sprite, &Parent), With<EnergyBar>>, // Barres d’énergie
) {
    for (mut sprite, parent) in bars.iter_mut() {
        if let Ok(creature) = creatures.get(parent.get()) {
            if creature.energy > 0. {
                // Calcul de la largeur en fonction de l’énergie restante
                let energy_ratio = creature.energy / creature.max_energy;
                sprite.custom_size = Some(Vec2::new(40.0 * energy_ratio, 5.0)); // Ajuste la taille
            }
        }
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
