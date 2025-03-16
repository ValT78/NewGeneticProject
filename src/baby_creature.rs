use bevy::prelude::*;

use crate::creature::Creature;

pub struct BabyCreaturePlugin;

//Contient tout ce qui est associé au concept de créature
impl Plugin for BabyCreaturePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_baby_status);
    }
}

#[derive(Component)]
#[require(Creature)]
pub struct BabyCreature {
    pub lifetime: Timer,
}

#[derive(Component)]
pub struct BabyIndicator;

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
