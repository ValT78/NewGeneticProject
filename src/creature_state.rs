use bevy::prelude::*;

use crate::creature::{CreatureState, CreatureStateComponent};

pub struct CreatureStatePlugin;

//Contient tout ce qui est associé au concept de créature
impl Plugin for CreatureStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_baby_status)
            .add_systems(Update, update_state_indicator);
    }
}

#[derive(Component)]
pub struct BabyCreature {
    pub lifetime: Timer,
}


#[derive(Component)]
pub struct BabyIndicator;

#[derive(Component)]
pub struct CreatureStateIndicator;

fn update_baby_status(
    mut commands: Commands,
    mut baby_creatures: Query<(Entity, &mut BabyCreature, &Children)>,
    time: Res<Time>,
    baby_indicators: Query<Entity, With<BabyIndicator>>,
) {
    for (entity, mut baby, children) in baby_creatures.iter_mut() {
        baby.lifetime.tick(time.delta());

        if baby.lifetime.finished() {
            commands.entity(entity).remove::<BabyCreature>(); // Enlève le statut bébé

            // Supprime l’indicateur visuel
            for &child in children.iter() {
                if baby_indicators.get(child).is_ok() {
                    commands.entity(child).despawn();
                }
            }
        }
    }
}

fn update_state_indicator(
    creature_query: Query<(&CreatureStateComponent, &Children)>,
    mut indicator_query: Query<&mut Sprite, With<CreatureStateIndicator>>,
) {
    for (creature_state, children) in creature_query.iter() {
        for &child in children.iter() {
            if let Ok(mut sprite) = indicator_query.get_mut(child) {
                sprite.color = match creature_state.state {
                    CreatureState::Neutral => Color::WHITE,                                      // 🚶‍♂️ Blanc
                    CreatureState::SeekingFood => Color::srgb(0.0, 1.0, 0.0),   // 🍎 Vert
                    CreatureState::InLove => Color::srgb(1.0, 0.75, 0.8),       // 🧡 Rose
                    CreatureState::Attacking => Color::srgb(1.0, 0.0, 0.0),     // 🔥 Rouge
                    CreatureState::Fleeing => Color::srgb(0.0, 0.0, 1.0),       // 🏃‍♂️ Bleu
                };
            }
        }
    }
}