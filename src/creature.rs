use std::cmp::Ordering;

use bevy::prelude::*;
use bevy::time;
use bevy::utils::HashSet;
use rand::prelude::*;

use crate::creature_state::{BabyCreature, BabyIndicator, CreatureStateIndicator};
use crate::collider::{Collider, CollisionEvent};
use crate::rigidbody::Accel;
use crate::MAP_HEIGHT;
use crate::MAP_WIDTH;
use crate::food::Food;
use crate::rigidbody::Velocity;

const INITIAL_CREATURE_COUNT: u32 = 60;
const SPAWN_ENERGY_RATE: f32 = 0.7;
const PASSIVE_ENERGY_LOSS: f32 = 50.;

const MIN_CREATURE_ENERGY: f32 = 6400.;
const MAX_CREATURE_ENERGY: f32 = 12800.;

const MIN_ACCEL: f32 = 1.;
const MAX_ACCEL: f32 = 500.;
const MIN_UPPER_SPEED: f32 = 1.;
const MAX_UPPER_SPEED: f32 = 1000.;

const MIN_CREATURE_AGGRESSIVENESS: f32 = 0.;
const MAX_CREATURE_AGGRESSIVENESS: f32 = 100.;
const MIN_ATTACK_POWER: f32 = 0.;
const MAX_ATTACK_POWER: f32 = 100.;
const MIN_HITBOX_RADIUS: f32 = 5.;
const MAX_HITBOX_RADIUS: f32 = 100.;
const MIN_HITBOX_REPULSION: f32 = 10.;
const MAX_HITBOX_REPULSION: f32 = 1000.;

const MIN_CREATURE_VISION_RADIUS: f32 = 10.;
const MAX_CREATURE_VISION_RADIUS: f32 = 500.;
const MIN_FOOD_VISION_RADIUS: f32 = 10.;
const MAX_FOOD_VISION_RADIUS: f32 = 3000.;
const MIN_LOVE_VISION_RADIUS: f32 = 10.;
const MAX_LOVE_VISION_RADIUS: f32 = 10000.;

const MIN_WANT_TO_LOVE_RATE: f32 = 0.1;
const MAX_WANT_TO_LOVE_RATE: f32 = 0.9;
const MIN_ENERGY_RATE_GIVEN_TO_BABY: f32 = 0.1;
const MAX_ENERGY_RATE_GIVEN_TO_BABY: f32 = 0.9;
const MIN_IS_A_CHILD_TIME: f32 = 5.;
const MAX_IS_A_CHILD_TIME: f32 = 30.;


pub struct CreaturePlugin;

//Contient tout ce qui est associé au concept de créature
impl Plugin for CreaturePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_initial_creatures)
            .add_systems(Update, path_find_to_nearest_target)
            .add_systems(Update, eat_food)
            .add_systems(Update, get_tired)
            .add_systems(Update, collision_interaction)
            .add_systems(Update, update_energy_bars);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CreatureState {
    Neutral,       // 🚶‍♂️ Avance tout droit
    SeekingFood,   // 🍎 Cherche de la nourriture
    InLove,        // 🧡 Cherche un partenaire
    Attacking,     // 🔥 Attaque une autre créature
    Fleeing,       // 🏃‍♂️ Fuit une créature plus agressive
}

impl Default for CreatureState {
    fn default() -> Self {
        CreatureState::Neutral
    }
}


#[derive(Component, Debug, Clone, Default)]
#[require(Transform, Velocity)] //Inclu forcément Transform::Default quand on crée le component Creature. On peut l'override
pub struct Creature {
    pub energy: f32,
    pub max_energy: f32,

    pub neutral_accel_factor: f32,
    pub eat_accel_factor: f32,
    pub attack_accel_factor: f32,
    pub flee_accel_factor: f32,
    pub love_accel_factor: f32,
    pub upper_speed: f32,

    pub aggressiveness: f32,
    pub attack_power: f32,
    pub hitbox_radius: f32,
    pub hitbox_repulsion: f32,

    pub creature_vision_radius: f32,
    pub food_vision_radius: f32,
    pub love_vision_radius: f32,

    pub want_to_love_rate: f32,
    pub energy_rate_given_to_baby: f32,
    pub is_a_child_time: f32,
    pub is_in_love: bool,

    pub generation: u32,
}

#[derive(Component)]
struct CreatureAllies {
    allies: HashSet<Entity>,
}

#[derive(Component)]
pub struct CreatureStateComponent {
    pub state: CreatureState,
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

    neutral_accel_factor: f32,
    eat_accel_factor: f32,
    attack_accel_factor: f32,
    flee_accel_factor: f32,
    love_accel_factor: f32,
    upper_speed: f32,

    aggressiveness: f32,
    attack_power: f32,
    hitbox_radius: f32,
    hitbox_repulsion: f32,

    creature_vision_radius: f32,
    food_vision_radius: f32,
    love_vision_radius: f32,

    want_to_love_rate: f32,
    energy_rate_given_to_baby: f32,
    is_a_child_time: f32,

    generation: u32,
    parents: Option<(Entity, Entity)>,

    commands: &mut Commands, 
    image: Handle<Image>,
    font: Handle<Font>,
) -> Entity{
    let mut allies = HashSet::new();

    if let Some((parent_a, parent_b)) = parents {
        allies.insert(parent_a);
        allies.insert(parent_b);
    }

    let creature = commands.spawn((
        Creature {
            energy,
            max_energy,
            neutral_accel_factor,
            eat_accel_factor,
            attack_accel_factor,
            flee_accel_factor,
            love_accel_factor,
            upper_speed,
            aggressiveness,
            attack_power,
            hitbox_radius,
            hitbox_repulsion,
            creature_vision_radius,
            food_vision_radius,
            love_vision_radius,
            want_to_love_rate,
            energy_rate_given_to_baby,
            is_a_child_time,
            is_in_love: false,
            generation,
            
        },
        CreatureAllies {
            allies,
        },
        CreatureStateComponent {
            state: CreatureState::Neutral,
        },
        BabyCreature {
            lifetime: Timer::from_seconds(is_a_child_time, TimerMode::Once),
        },
        Transform::from_xyz(x as f32, y as f32, 0.0),
        Collider { radius: hitbox_radius, repulsion_force: hitbox_repulsion },
        Sprite {
            image,
            custom_size: Some(Vec2::new(hitbox_radius * 2.0, hitbox_radius * 2.0)),
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
                translation: Vec3::new(10.0, -10.0, 1.0), // Position au-dessus de la créature
                ..default()
            },
            BabyIndicator, // Marqueur pour supprimer plus tard
        ));

        // 🔷 Indicateur Etat
        parent.spawn((
            Sprite {
                    color: Color::WHITE,
                    custom_size: Some(Vec2::new(10.0, 10.0)), // Petite icône
                    ..default()
                },
            Transform {
                translation: Vec3::new(-10.0, -10.0, 1.0), // Position au-dessus de la créature
                ..default()
            },
            CreatureStateIndicator, // Marqueur pour supprimer plus tard
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
            Text2d::new(format!("Gen {}", generation)), 
            Transform::from_xyz(0., -20., 1.0), 
            TextColor(Color::BLACK), 
            TextFont {font, font_size: 20., ..default()}, 
            TextLayout::default()
        ));


        
    });

    creature
    
}

fn spawn_initial_creatures(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut rng: ThreadRng = rand::rng();

    let image = asset_server.load("sprites/creature.png");
    let font = asset_server.load("fonts/COOPBL.TTF");
    

    for _ in 0..INITIAL_CREATURE_COUNT {
        let x = rng.random_range(-MAP_WIDTH / 2..MAP_WIDTH / 2);
        let y = rng.random_range(-MAP_HEIGHT / 2..MAP_HEIGHT / 2);

        let max_energy = rng.random_range(MIN_CREATURE_ENERGY..MAX_CREATURE_ENERGY);
        let energy = SPAWN_ENERGY_RATE * max_energy;
        
        let neutral_accel_factor = rng.random_range(MIN_ACCEL..MAX_ACCEL);
        let eat_accel_factor = rng.random_range(MIN_ACCEL..MAX_ACCEL);
        let attack_accel_factor = rng.random_range(MIN_ACCEL..MAX_ACCEL);
        let flee_accel_factor = rng.random_range(MIN_ACCEL..MAX_ACCEL);
        let love_accel_factor = rng.random_range(MIN_ACCEL..MAX_ACCEL);
        let upper_speed = rng.random_range(MIN_UPPER_SPEED..MAX_UPPER_SPEED);

        let aggressiveness = rng.random_range(MIN_CREATURE_AGGRESSIVENESS..MAX_CREATURE_AGGRESSIVENESS);
        let attack_power = rng.random_range(MIN_ATTACK_POWER..MAX_ATTACK_POWER);
        let hitbox_radius = rng.random_range(MIN_HITBOX_RADIUS..MAX_HITBOX_RADIUS);
        let hitbox_repulsion = rng.random_range(MIN_HITBOX_REPULSION..MAX_HITBOX_REPULSION);

        let creature_vision_radius = rng.random_range(MIN_CREATURE_VISION_RADIUS..MAX_CREATURE_VISION_RADIUS);
        let food_vision_radius = rng.random_range(MIN_FOOD_VISION_RADIUS..MAX_FOOD_VISION_RADIUS);
        let love_vision_radius = rng.random_range(MIN_LOVE_VISION_RADIUS..MAX_LOVE_VISION_RADIUS);

        let want_to_love_rate = rng.random_range(MIN_WANT_TO_LOVE_RATE..MAX_WANT_TO_LOVE_RATE);
        let energy_rate_given_to_baby = rng.random_range(MIN_ENERGY_RATE_GIVEN_TO_BABY..MAX_ENERGY_RATE_GIVEN_TO_BABY);
        let is_a_child_time = rng.random_range(MIN_IS_A_CHILD_TIME..MAX_IS_A_CHILD_TIME);

        spawn_creature(
            x, y, 
            energy, max_energy, 
            neutral_accel_factor, eat_accel_factor, attack_accel_factor, flee_accel_factor, love_accel_factor, upper_speed, 
            aggressiveness, attack_power, hitbox_radius, hitbox_repulsion, 
            creature_vision_radius, food_vision_radius, love_vision_radius,
            want_to_love_rate, energy_rate_given_to_baby, is_a_child_time, 
            0, None,
            &mut commands, image.clone(), font.clone(),
        );

    }
}

fn path_find_to_nearest_target(
    mut creature_query: Query<(&Transform, &mut Velocity, &mut Accel, &mut CreatureStateComponent, &CreatureAllies, &Creature, )>,
    food_query: Query<&Transform, With<Food>>,
    other_creature_query: Query<(&Transform, &Creature, Entity), Without<Food>>, // Exclure les nourritures
) {
    for (transform, mut velocity, mut accel, mut creature_state, creature_allies, creature) in creature_query.iter_mut() {

        let (target, accel_factor, state) = if creature.is_in_love {
            // 🧡 Mode amoureux : Chercher une autre créature amoureuse la plus proche
            (
                other_creature_query
                    .iter()
                    .filter(|(other_transform, other, _)| {
                        other.is_in_love && other_transform.translation != transform.translation
                        && other_transform.translation.distance(transform.translation) <= creature.love_vision_radius
                    })
                    .min_by(|(lhs_t, _, _), (rhs_t, _, _)| {
                        order_float(
                            lhs_t.translation.distance(transform.translation),
                            rhs_t.translation.distance(transform.translation),
                        )
                    })
                    .map(|(t, _, _)| t.translation),
                creature.love_accel_factor,
                CreatureState::InLove,
            )
        } else if let Some((other_transform, other_creature, _)) = other_creature_query
            .iter()
            .filter(|(other_t, _, other_e)| {
                other_t.translation.distance(transform.translation) <= creature.creature_vision_radius
                && other_t.translation != transform.translation
                && !creature_allies.allies.contains(other_e) // 💥 Ne pas attaquer un allié !
            })
            .min_by(|(lhs_t, _, _), (rhs_t, _, _)| {
                order_float(
                    lhs_t.translation.distance(transform.translation),
                    rhs_t.translation.distance(transform.translation),
                )
            })
        {

            // 🔥 Mode agressif : Attaque ou fuite selon l'aggressivity
            if creature.aggressiveness > other_creature.aggressiveness {
                (
                    Some(other_transform.translation),
                    creature.attack_accel_factor,
                    CreatureState::Attacking,
                ) // Fonce sur la cible
            } else {
                // 🏃‍♂️ Fuit dans la direction opposée
                (
                    Some(other_transform.translation),
                    -creature.flee_accel_factor,
                    CreatureState::Fleeing,
                )
            }
        } else if let Some(food_position) = food_query
            .iter()
            .filter(|t| t.translation.distance(transform.translation) <= creature.food_vision_radius)
            .min_by(|lhs, rhs| {
                order_float(
                    lhs.translation.distance(transform.translation),
                    rhs.translation.distance(transform.translation),
                )
            })
            .map(|t| t.translation)
        {
            print!("{}", food_position.distance(transform.translation));
            // 🍎 Mode nourriture : Cherche la nourriture la plus proche
            (Some(food_position), creature.eat_accel_factor, CreatureState::SeekingFood)
        } else {
            // print!("{}", food_position.distance(transform.translation));
            // 🚶‍♂️ Mode neutre : Avance tout droit
            (
                Some(transform.translation),
                creature.neutral_accel_factor,
                CreatureState::Neutral
                ,
            )
        };
        
        // Mettre à jour l'état de la créature
        creature_state.state = state;
        
        

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

fn collision_interaction(
    mut commands: Commands,
    mut events: EventReader<CollisionEvent>,
    mut creatures: Query<(&mut Creature, &mut CreatureAllies)>,
    transforms: Query<&Transform>,
    asset_server: Res<AssetServer>,
) {
    for event in events.read() {
        let Ok([(mut creature_a, mut allies_a), (mut creature_b, mut allies_b)]) = creatures.get_many_mut([event.entity_a, event.entity_b]) else { continue; };

        // Vérifie si les deux créatures sont en mode "love"
        if creature_a.is_in_love && creature_b.is_in_love {
            let Ok(transform_a) = transforms.get(event.entity_a) else { continue; };
            let Ok(transform_b) = transforms.get(event.entity_b) else { continue; };

            let image = asset_server.load("sprites/creature.png");

            let font = asset_server.load("fonts/COOPBL.TTF");

            let baby_max_energy = (creature_a.max_energy + creature_b.max_energy) / 2.0;

            // Apparition du bébé
            let baby_entity = spawn_creature(
                ((transform_a.translation.x + transform_b.translation.x) / 2.0) as i32,
                ((transform_a.translation.y + transform_b.translation.y) / 2.0) as i32,
                (creature_a.energy + creature_b.energy) / 2.0,
                baby_max_energy,
                (creature_a.neutral_accel_factor + creature_b.neutral_accel_factor) / 2.0,
                (creature_a.eat_accel_factor + creature_b.eat_accel_factor) / 2.0,
                (creature_a.attack_accel_factor + creature_b.attack_accel_factor) / 2.0,
                (creature_a.flee_accel_factor + creature_b.flee_accel_factor) / 2.0,
                (creature_a.love_accel_factor + creature_b.love_accel_factor) / 2.0,
                (creature_a.upper_speed + creature_b.upper_speed) / 2.0,
                (creature_a.aggressiveness + creature_b.aggressiveness) / 2.0,
                (creature_a.attack_power + creature_b.attack_power) / 2.0,
                (creature_a.hitbox_radius + creature_b.hitbox_radius) / 2.0,
                (creature_a.hitbox_repulsion + creature_b.hitbox_repulsion) / 2.0,
                (creature_a.creature_vision_radius + creature_b.creature_vision_radius) / 2.0,
                (creature_a.food_vision_radius + creature_b.food_vision_radius) / 2.0,
                (creature_a.love_vision_radius + creature_b.love_vision_radius) / 2.0,
                (creature_a.want_to_love_rate + creature_b.want_to_love_rate) / 2.0,
                (creature_a.energy_rate_given_to_baby + creature_b.energy_rate_given_to_baby) / 2.0,
                (creature_a.is_a_child_time + creature_b.is_a_child_time) / 2.0,
                u32::max(creature_a.generation, creature_b.generation) + 1,
                Some((event.entity_a, event.entity_b)),
                &mut commands,
                image,
                font,
            );

            // Les parents perdent de l'énergie
            creature_a.energy -= creature_a.energy_rate_given_to_baby * creature_a.energy;
            creature_b.energy -= creature_b.energy_rate_given_to_baby * creature_b.energy;
            allies_a.allies.insert(event.entity_b);
            allies_b.allies.insert(event.entity_a);
            allies_a.allies.insert(baby_entity);
            allies_b.allies.insert(baby_entity);
            creature_a.is_in_love = false;
            creature_b.is_in_love = false;
        }
        else {
            // Vérifier si les créatures sont la leur liste d'alliés
            if allies_a.allies.contains(&event.entity_b) && allies_b.allies.contains(&event.entity_a) {
                continue;
            }
            creature_a.energy -= creature_b.attack_power;
            creature_b.energy -= creature_a.attack_power;
            if creature_a.energy <= 0. {
                creature_b.energy = creature_b.max_energy;
                commands.entity(event.entity_a).despawn_recursive();
            }
            if creature_b.energy <= 0. {
                creature_a.energy = creature_a.max_energy;
                commands.entity(event.entity_b).despawn_recursive();
            }
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
            > closest_creature.hitbox_radius
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
        creature.energy -= (velocity.0.length() + PASSIVE_ENERGY_LOSS) * time.delta_secs() ;

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
