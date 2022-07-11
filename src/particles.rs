use bevy::prelude::*;

pub struct ParticlesPlugin;

impl Plugin for ParticlesPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_velocity).add_system(destroy_after);
    }
}

#[derive(Component)]
pub struct DestroyAfter {
    time_to_destroy: f32,
}

impl DestroyAfter {
    pub fn new(time_to_destroy: f32) -> Self {
        Self { time_to_destroy }
    }
}

#[derive(Component)]
pub struct Velocity(pub Vec2);

fn update_velocity(time: Res<Time>, mut q_vel: Query<(&mut Transform, &Velocity)>) {
    for (mut t, vel) in q_vel.iter_mut() {
        t.translation += (vel.0 * time.delta_seconds()).extend(0f32);
    }
}

fn destroy_after(mut commands: Commands, time: Res<Time>, q_des: Query<(Entity, &DestroyAfter)>) {
    for (e, d) in q_des.iter() {
        if d.time_to_destroy <= time.seconds_since_startup() as f32 {
            commands.entity(e).despawn();
        }
    }
}
