use bevy::prelude::*;

use crate::rigidbody::Accel;

const FORCE_FACTOR: f32 = 10.0;

#[derive(Component)]
pub struct Collider {
    pub radius: f32,
}

pub struct ColliderPlugin;

impl Plugin for ColliderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, resolve_collisions);
    }
}

fn resolve_collisions(
    mut query: Query<(&mut Accel, &Transform, &Collider)>,
) {
    let mut entities: Vec<_> = query.iter_mut().collect();

    for i in 0..entities.len() {
        for j in (i + 1)..entities.len() {
            let dir = entities[j].1.translation.xy() - entities[i].1.translation.xy();
            let min_dist = entities[i].2.radius + entities[j].2.radius;
            let dist = dir.length();
            
            if dist < min_dist && dist > 0.0 {
                let repulsion_force = dir.normalize() * (min_dist - dist) * FORCE_FACTOR; // Facteur de force

                entities[i].0.0 -= repulsion_force;
                entities[j].0.0 += repulsion_force;
            }
        }
    }
}