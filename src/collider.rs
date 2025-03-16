use bevy::prelude::*;

use crate::rigidbody::Accel;

#[derive(Component)]
pub struct Collider {
    pub radius: f32,
    pub repulsion_force: f32,
}


#[derive(Event)]
pub struct CollisionEvent {
    pub entity_a: Entity,
    pub entity_b: Entity,
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
                let a_repulsion_force = dir.normalize() * (min_dist - dist) * entities[j].2.repulsion_force;
                let b_repulsion_force = -dir.normalize() * (min_dist - dist) * entities[i].2.repulsion_force;

                let (left, right) = entities.split_at_mut(j);
                match (&mut left[i].3, &mut right[0].3) {
                    (Some(a), Some(b)) => {
                        // Si les deux entités peuvent bouger, elles se repoussent
                        a.0 -= a_repulsion_force;
                        b.0 += b_repulsion_force;
                    }
                    (Some(a), None) => {
                        // Si SEULEMENT A peut bouger, il prend toute la force
                        a.0 -= a_repulsion_force;
                    }
                    (None, Some(b)) => {
                        // Si SEULEMENT B peut bouger, il prend toute la force
                        b.0 += b_repulsion_force;
                    }
                    (None, None) => {
                        // Si aucun ne peut bouger, on ne fait rien
                    }
                }

                // Émettre un événement de collision
                collision_events.send(CollisionEvent {
                    entity_a: entities[i].0,
                    entity_b: entities[j].0,
                });
            }
        }
    }
}
