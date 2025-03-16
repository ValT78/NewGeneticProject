use std::cmp::Ordering;

use bevy::prelude::*;
use bevy::time;
use rand::prelude::*;

use crate::baby_creature::{BabyCreature, BabyIndicator};
use crate::collider::{Collider, CollisionEvent};
use crate::rigidbody::Accel;
use crate::MAP_HEIGHT;
use crate::MAP_WIDTH;
use crate::food::Food;
use crate::rigidbody::Velocity;

const INITIAL_CREATURE_COUNT: u32 = 40;
const PASSIVE_ENERGY_LOSS: f32 = 1.;

const MIN_CREATURE_ACCEL: f32 = 1.;
const MAX_CREATURE_ACCEL: f32 = 500.;
const MIN_CREATURE_ENERGY: f32 = 3200.;
const MAX_CREATURE_ENERGY: f32 = 6400.;
const MIN_SPAWN_ENERGY_RATE: f32 = 0.1;
const MAX_SPAWN_ENERGY_RATE: f32 = 0.5;
const MIN_WANT_TO_LOVE_RATE: f32 = 0.1;
const MAX_WANT_TO_LOVE_RATE: f32 = 0.9;
const MIN_ENERGY_RATE_GIVEN_TO_BABY: f32 = 0.1;
const MAX_ENERGY_RATE_GIVEN_TO_BABY: f32 = 0.9;
const MIN_IS_A_CHILD_TIME: f32 = 5.;
const MAX_IS_A_CHILD_TIME: f32 = 30.;
const MIN_SPAWN_SIZE: f32 = 5.;
const MAX_SPAWN_SIZE: f32 = 100.;

const MIN_UPPER_SPEED: f32 = 1.;
const MAX_UPPER_SPEED: f32 = 1000.;

pub struct CreaturePlugin;

//Contient tout ce qui est associé au concept de créature
impl Plugin for CreaturePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_initial_creatures)
            .add_systems(Update, path_find_to_nearest_target)
            .add_systems(Update, eat_food)
            .add_systems(Update, get_tired)
            .add_systems(Update, handle_reproduction)
            .add_systems(Update, update_energy_bars);
    }
}

#[derive(Component, Debug, Clone, Default)]
#[require(Transform, Velocity)] //Inclu forcément Transform::Default quand on crée le component Creature. On peut l'override
pub struct Creature {
    pub energy: f32,
    pub max_energy: f32,
    pub want_to_love_rate: f32,
    pub eat_accel_factor: f32,
    pub love_accel_factor: f32,
    pub energy_rate_given_to_baby: f32,
    pub is_a_child_time: f32,
    pub upper_speed: f32,
    pub radius: f32,
    pub generation: u32,
    pub is_in_love: bool,
}

#[derive(Component, Debug, Clone)]
#[require(Transform)]
pub struct EnergyBar;

#[derive(Component, Debug, Clone)]
#[require(Creature)]
pub struct GenerationLabel;

pub fn spawn_creature(
    x: i32, 
    y: i32, 
    energy: f32,
    max_energy: f32,
    want_to_love_rate: f32,
    eat_accel_factor: f32,
    love_accel_factor: f32,
    energy_rate_given_to_baby: f32,
    is_a_child_time: f32,
    upper_speed: f32,
    radius: f32,
    generation: u32,
    commands: &mut Commands, 
    image: Handle<Image>,
    font: Handle<Font>,
) {
    let creature = commands.spawn((
        Creature {
            energy,
            max_energy,
            want_to_love_rate,
            eat_accel_factor,
            love_accel_factor,
            energy_rate_given_to_baby,
            is_a_child_time,
            upper_speed,
            radius,
            generation,
            is_in_love: false,
        },
        BabyCreature {
            lifetime: Timer::from_seconds(is_a_child_time, TimerMode::Once),
        },
        Transform::from_xyz(x as f32, y as f32, 0.0),
        Collider { radius },
        Sprite {
            image,
            custom_size: Some(Vec2::new(radius, radius)),
            ..default()
        },
    )).id();

    commands.entity(creature).with_children(|parent| {
        // 👶 Indicateur bébé
        parent.spawn((
            Sprite {
                    color: Color::srgb(1.0, 0.8, 0.2), // Jaune/orange pour le bébé
                    custom_size: Some(Vec2::new(10.0, 10.0)), // Petite icône
                    ..default()
                },
            Transform {
                translation: Vec3::new(0.0, -10.0, 1.0), // Position au-dessus de la créature
                ..default()
            },
            BabyIndicator, // Marqueur pour supprimer plus tard
        ));

        parent.spawn((
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

        parent.spawn((
            Text2d::new(
                format!("Gen {}", generation), // Affichage du texte
            ),
            TextFont {
                font,
                font_size: 14.0,
                ..default()
            },
            Transform {
                translation: Vec3::new(0.0, 20.0, 1.0),
                ..default()
            },
            TextColor(Color::WHITE),
            TextLayout::new_with_justify(JustifyText::Center),
            GenerationLabel, // Composant pour identifier ce texte
        ));

        
    });
    
}

fn spawn_initial_creatures(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut rng: ThreadRng = rand::rng();

    let image = asset_server.load("sprites/creature.png");
    let font = asset_server.load("fonts/COOPBL.TTF");

    for _ in 0..INITIAL_CREATURE_COUNT {
        let x = rng.random_range(-MAP_WIDTH / 2..MAP_WIDTH / 2);
        let y = rng.random_range(-MAP_HEIGHT / 2..MAP_HEIGHT / 2);

        let eat_accel_factor = rng.random_range(MIN_CREATURE_ACCEL..MAX_CREATURE_ACCEL);
        let love_accel_factor = rng.random_range(MIN_CREATURE_ACCEL..MAX_CREATURE_ACCEL);
        let upper_speed = rng.random_range(MIN_UPPER_SPEED..MAX_UPPER_SPEED);
        let max_energy = rng.random_range(MIN_CREATURE_ENERGY..MAX_CREATURE_ENERGY);
        let energy = rng.random_range(MIN_SPAWN_ENERGY_RATE..MAX_SPAWN_ENERGY_RATE) * max_energy;
        let want_to_love_rate = rng.random_range(MIN_WANT_TO_LOVE_RATE..MAX_WANT_TO_LOVE_RATE);
        let energy_rate_given_to_baby = rng.random_range(MIN_ENERGY_RATE_GIVEN_TO_BABY..MAX_ENERGY_RATE_GIVEN_TO_BABY);
        let is_a_child_time = rng.random_range(MIN_IS_A_CHILD_TIME..MAX_IS_A_CHILD_TIME);
        let size = rng.random_range(MIN_SPAWN_SIZE..MAX_SPAWN_SIZE);



        spawn_creature(x, y, energy, max_energy, want_to_love_rate, eat_accel_factor, love_accel_factor, energy_rate_given_to_baby, is_a_child_time, upper_speed, size, 0, &mut commands, image.clone(), font.clone());
    }
}

fn path_find_to_nearest_target(
    mut creature_query: Query<(&Transform, &mut Velocity, &mut Accel, &Creature)>,
    food_query: Query<&Transform, With<Food>>,
    other_creature_query: Query<(&Transform, &Creature), Without<Food>>, // Exclure les nourritures
) {
    for (transform, mut velocity, mut accel, creature) in creature_query.iter_mut() {

        let (target, accel_factor) = if !creature.is_in_love {
            // Si l'énergie est inférieure à 50%, chercher la nourriture
            (
            food_query
                .iter()
                .min_by(|lhs, rhs| {
                    order_float(
                        lhs.translation.distance(transform.translation),
                        rhs.translation.distance(transform.translation),
                    )
                })
                .map(|t| t.translation),
                creature.eat_accel_factor,
            )
        } else {
            // Sinon, chercher une autre créature ayant aussi beaucoup d'énergie
            (
            other_creature_query
                .iter()
                .filter(|(other_transform, other)| {
                    other.is_in_love && other_transform.translation != transform.translation
                }) // Filtrer sur l'énergie et exclure la créature concernée
                .min_by(|(lhs_t, _), (rhs_t, _)| {
                    order_float(
                        lhs_t.translation.distance(transform.translation),
                        rhs_t.translation.distance(transform.translation),
                    )
                })
                .map(|(t, _)| t.translation),
                creature.love_accel_factor,
            )
        };        

        if let Some(target_position) = target {
            accel.0 = (target_position - transform.translation)
                .normalize_or_zero()
                .xy()
                * accel_factor;
        }

        // Limiter la vitesse de la créature
        if velocity.0.length() > creature.upper_speed {
            velocity.0 = velocity.0.normalize() * creature.upper_speed;
        }
    }
}

fn handle_reproduction(
    mut commands: Commands,
    mut events: EventReader<CollisionEvent>,
    mut creatures: Query<&mut Creature>,
    transforms: Query<&Transform>,
    asset_server: Res<AssetServer>,
) {
    for event in events.read() {
        let Ok([mut creature_a, mut creature_b]) = creatures.get_many_mut([event.entity_a, event.entity_b]) else { continue; };

        // Vérifie si les deux créatures sont en mode "love"
        if creature_a.is_in_love && creature_b.is_in_love {
            let Ok(transform_a) = transforms.get(event.entity_a) else { continue; };
            let Ok(transform_b) = transforms.get(event.entity_b) else { continue; };

            let image = asset_server.load("sprites/creature.png");

            let font = asset_server.load("fonts/COOPBL.TTF");

            let baby_max_energy = (creature_a.max_energy + creature_b.max_energy) / 2.0;

            // Apparition du bébé
            spawn_creature(
                ((transform_a.translation.x + transform_b.translation.x) / 2.0) as i32,
                ((transform_a.translation.y + transform_b.translation.y) / 2.0) as i32,
                (creature_a.energy * creature_a.energy_rate_given_to_baby + creature_b.energy * creature_b.energy_rate_given_to_baby),
                baby_max_energy,
                (creature_a.want_to_love_rate + creature_b.want_to_love_rate) / 2.0,
                (creature_a.eat_accel_factor + creature_b.eat_accel_factor) / 2.0,
                (creature_a.love_accel_factor + creature_b.love_accel_factor) / 2.0,
                (creature_a.energy_rate_given_to_baby + creature_b.energy_rate_given_to_baby) / 2.0,
                (creature_a.is_a_child_time + creature_b.is_a_child_time) / 2.0,
                (creature_a.upper_speed + creature_b.upper_speed) / 2.0,
                (event.size_a + event.size_b) / 2.0,
                creature_a.generation + 1,
                &mut commands,
                image.clone(),
                font.clone(),
            );

            // Les parents perdent de l'énergie
            creature_a.energy -= creature_a.energy_rate_given_to_baby * creature_a.energy;
            creature_b.energy -= creature_b.energy_rate_given_to_baby * creature_b.energy;
            creature_a.is_in_love = false;
            creature_b.is_in_love = false;
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
            > closest_creature.radius
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
    mut creature_query: Query<(&mut Creature, &Velocity, Entity, Option<&BabyCreature>)>,
    mut commands: Commands,
    time: Res<time::Time>,
) {
    for (mut creature, velocity, creature_entity, baby_creature) in creature_query.iter_mut() {
        creature.energy -= velocity.0.length() * time.delta_secs() + PASSIVE_ENERGY_LOSS;

        //Vérifier si l'entité possède babyCreature
        if let Some(_baby_creature) = baby_creature {
            creature.is_in_love = false;
        }
        else {
            creature.is_in_love = creature.energy / creature.max_energy > creature.want_to_love_rate;
        }

        if creature.energy <= 0. {
            commands.entity(creature_entity).despawn_recursive();
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
