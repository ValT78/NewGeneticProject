use bevy::prelude::*;

use crate::rigidbody::Accel;

const FORCE_FACTOR: f32 = 10.0;

#[derive(Component)]
pub struct Collider {
    pub radius: f32,
}


#[derive(Event)]
pub struct CollisionEvent {
    pub entity_a: Entity,
    pub entity_b: Entity,
    pub size_a: f32,
    pub size_b: f32,
}


pub struct ColliderPlugin;

impl Plugin for ColliderPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CollisionEvent>()
        .add_systems(Update, resolve_collisions);
    }
}

fn resolve_collisions(
    mut query: Query<(Entity, &Transform, &Collider, Option<&mut Accel>)>,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    let mut entities: Vec<_> = query.iter_mut().collect();

    for i in 0..entities.len() {
        for j in (i + 1)..entities.len() {
            let dir = entities[j].1.translation.xy() - entities[i].1.translation.xy();
            let min_dist = entities[i].2.radius + entities[j].2.radius;
            let dist = dir.length();

            if dist < min_dist && dist > 0.0 {
                // Calculer la force de répulsion
                let repulsion_force = dir.normalize() * (min_dist - dist) * FORCE_FACTOR;

                let (left, right) = entities.split_at_mut(j);
                match (&mut left[i].3, &mut right[0].3) {
                    (Some(a), Some(b)) => {
                        // Si les deux entités peuvent bouger, elles se repoussent équitablement
                        a.0 -= repulsion_force;
                        b.0 += repulsion_force;
                    }
                    (Some(a), None) => {
                        // Si SEULEMENT A peut bouger, il prend toute la force
                        a.0 -= repulsion_force;
                    }
                    (None, Some(b)) => {
                        // Si SEULEMENT B peut bouger, il prend toute la force
                        b.0 += repulsion_force;
                    }
                    (None, None) => {
                        // Si aucun ne peut bouger, on ne fait rien
                    }
                }

                // Émettre un événement de collision
                collision_events.send(CollisionEvent {
                    entity_a: entities[i].0,
                    entity_b: entities[j].0,
                    size_a: entities[i].2.radius,
                    size_b: entities[j].2.radius,
                });
            }
        }
    }
}
