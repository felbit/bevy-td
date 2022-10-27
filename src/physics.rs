use crate::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Bundle)]
pub struct PhysicsBundle {
    flags: ActiveEvents,
    active_collision_types: ActiveCollisionTypes,
    collider: Collider,
    colliding_entities: CollidingEntities,
    rigid_body: RigidBody,
    rotation_constraint: LockedAxes,
    velocity: Velocity,
}

impl PhysicsBundle {
    pub fn moving_entity(size: Vec3) -> Self {
        Self {
            flags: ActiveEvents::COLLISION_EVENTS,
            active_collision_types: ActiveCollisionTypes::default()
                | ActiveCollisionTypes::KINEMATIC_KINEMATIC,
            collider: Collider::cuboid(size.x / 2.0, size.y / 2.0, size.z / 2.0),
            colliding_entities: CollidingEntities::default(),
            rigid_body: RigidBody::KinematicPositionBased,
            rotation_constraint: LockedAxes::ROTATION_LOCKED,
            velocity: Velocity::zero(),
        }
    }
}

#[derive(Default)]
pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(bullet_collision_detection);
    }
}

fn bullet_collision_detection(
    mut commands: Commands,
    bullets: Query<Entity, With<Bullet>>,
    mut targets: Query<(&mut Health, &CollidingEntities), With<Target>>,
) {
    for (mut health, colliding_targets) in targets.iter_mut() {
        for bullet in bullets.iter() {
            if colliding_targets.contains(bullet) {
                commands.entity(bullet).despawn_recursive();
                health.value -= 1;
            }
        }
    }
}
