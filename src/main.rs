mod bullet;
mod physics;
mod target;
mod tower;

mod prelude {
    pub use bevy::{prelude::*, utils::FloatOrd};
    pub use bevy_inspector_egui::WorldInspectorPlugin;
    pub use bevy_rapier3d::{
        prelude::{NoUserData, RapierConfiguration, RapierPhysicsPlugin},
        render::RapierDebugRenderPlugin,
    };

    pub const HEIGHT: f32 = 720.0;
    pub const WIDTH: f32 = 1280.0;

    pub use crate::bullet::*;
    pub use crate::physics::*;
    pub use crate::target::*;
    pub use crate::tower::*;
}

use prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .insert_resource(WindowDescriptor {
            width: WIDTH,
            height: HEIGHT,
            title: "Bevy Tower Defense".to_string(),
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        //.add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(TowerPlugin)
        .add_plugin(BulletPlugin)
        .add_plugin(TargetPlugin)
        .add_plugin(PhysicsPlugin)
        .add_startup_system(spawn_basic_scene)
        .add_startup_system(spawn_camera)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn spawn_basic_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut rapier_cfg: ResMut<RapierConfiguration>,
) {
    // set gravity
    rapier_cfg.gravity = Vec3::ZERO;

    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 10.0 })),
            material: materials.add(Color::rgb(0.337, 0.49, 0.275).into()),
            ..default()
        })
        .insert(Name::new("Ground"));

    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.67, 0.84, 0.92).into()),
            transform: Transform::from_xyz(1.0, 0.5, 0.0),
            ..default()
        })
        .insert(Tower {
            shooting_range: 2.0,
            shooting_timer: Timer::from_seconds(1.0, true),
            bullet_offset: Vec3::new(0.0, 0.2, 0.5),
        })
        .insert(Name::new("Tower"));

    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.67, 0.84, 0.92).into()),
            transform: Transform::from_xyz(-1.0, 0.5, 0.0),
            ..default()
        })
        .insert(Tower {
            shooting_range: 2.0,
            shooting_timer: Timer::from_seconds(1.0, true),
            bullet_offset: Vec3::new(0.0, 0.2, 0.5),
        })
        .insert(Name::new("Tower"));

    commands
        .spawn_bundle(PointLightBundle {
            point_light: PointLight {
                intensity: 1500.0,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(4.0, 8.0, 4.0),
            ..default()
        })
        .insert(Name::new("Light"));

    // Targets
    // TODO: make spawner for these!
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.3 })),
            material: materials.add(Color::rgb(0.67, 0.84, 0.92).into()),
            transform: Transform::from_xyz(-2.0, 0.2, 1.5),
            ..default()
        })
        .insert_bundle(PhysicsBundle::moving_entity(Vec3::new(0.2, 0.2, 0.2)))
        .insert(Target { speed: 0.2 })
        .insert(Health { value: 3 })
        .insert(Name::new("Target"));

    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.3 })),
            material: materials.add(Color::rgb(0.67, 0.84, 0.92).into()),
            transform: Transform::from_xyz(-2.0, 1.2, 1.5),
            ..default()
        })
        .insert_bundle(PhysicsBundle::moving_entity(Vec3::new(0.4, 0.4, 0.4)))
        .insert(Target { speed: 0.4 })
        .insert(Health { value: 3 })
        .insert(Name::new("Target"));

    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.3 })),
            material: materials.add(Color::rgb(0.67, 0.84, 0.92).into()),
            transform: Transform::from_xyz(-1.8, 0.1, 1.5),
            ..default()
        })
        .insert_bundle(PhysicsBundle::moving_entity(Vec3::new(0.4, 0.4, 0.4)))
        .insert(Target { speed: 0.3 })
        .insert(Health { value: 3 })
        .insert(Name::new("Target"));

    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.3 })),
            material: materials.add(Color::rgb(0.67, 0.84, 0.92).into()),
            transform: Transform::from_xyz(-2.3, 0.5, 1.5),
            ..default()
        })
        .insert_bundle(PhysicsBundle::moving_entity(Vec3::new(0.4, 0.4, 0.4)))
        .insert(Target { speed: 0.5 })
        .insert(Health { value: 3 })
        .insert(Name::new("Target"));
}
