use bevy::prelude::*;

#[derive(Component, Default)] // Default permet d'ajouter la valeur apr défaut d'un type quand on initialise
#[require(Accel, Transform)]
pub struct Velocity(pub Vec2);

#[derive(Component, Default)]
pub struct Accel(pub Vec2);


pub struct RigidbodyPlugin;

impl Plugin for RigidbodyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_rigidbody_position);
    }
}

fn update_rigidbody_position(mut query: Query<(&mut Velocity, &Accel, &mut Transform)>, time: Res<Time>) {
    for (mut velocity, accel, mut transform) in query.iter_mut() {
        velocity.0 += accel.0 * time.delta_secs();
        transform.translation += velocity.0.extend(0.) * time.delta_secs();
    }
}

