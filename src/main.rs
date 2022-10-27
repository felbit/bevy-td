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

    pub struct GameAssets {
        pub tower_scene: Handle<Scene>,
        pub construction_site_scene: Handle<Scene>,
        pub target_scene: Handle<Scene>,
    }
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
        .add_startup_system_to_stage(StartupStage::PreStartup, asset_loading)
        .add_startup_system(spawn_basic_scene)
        .add_startup_system(spawn_camera)
        .add_system(bevy::window::close_on_esc)
        .add_system(camera_controls)
        .run();
}

fn asset_loading(mut commands: Commands, assets: Res<AssetServer>) {
    commands.insert_resource(GameAssets {
        tower_scene: assets.load("tower.glb#Scene0"),
        construction_site_scene: assets.load("woodStructure.glb#Scene0"),
        target_scene: assets.load("ufo_red.glb#Scene0"),
    });
}

fn camera_controls(
    keyboard: Res<Input<KeyCode>>,
    mut camera_q: Query<&mut Transform, With<Camera3d>>,
    time: Res<Time>,
) {
    let mut camera = camera_q.single_mut();
    let mut forward = camera.forward();
    let mut left = camera.left();

    // Camera is angled slightly downwards, so we have to zero the Y
    // value and normalize the camera to move forward over the plane.
    forward.y = 0.0;
    forward = forward.normalize();

    left.y = 0.0;
    left = left.normalize();

    let speed = 3.0;
    let rotate_speed = 1.0;

    if keyboard.pressed(KeyCode::W) {
        camera.translation += forward * time.delta_seconds() * speed;
    }

    if keyboard.pressed(KeyCode::S) {
        camera.translation -= forward * time.delta_seconds() * speed;
    }

    if keyboard.pressed(KeyCode::A) {
        camera.translation += left * time.delta_seconds() * speed;
    }

    if keyboard.pressed(KeyCode::D) {
        camera.translation -= left * time.delta_seconds() * speed;
    }

    if keyboard.pressed(KeyCode::Q) {
        camera.rotate_axis(Vec3::Y, -rotate_speed * time.delta_seconds())
    }

    if keyboard.pressed(KeyCode::E) {
        camera.rotate_axis(Vec3::Y, rotate_speed * time.delta_seconds())
    }
}

fn spawn_camera(mut commands: Commands) {
    commands
        .spawn_bundle(Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert_bundle(PickingCameraBundle::default());
}

fn spawn_basic_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    game_assets: Res<GameAssets>,
    mut rapier_cfg: ResMut<RapierConfiguration>,
) {
    // set gravity
    rapier_cfg.gravity = Vec3::ZERO;

    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 25.0 })),
            material: materials.add(Color::rgb(0.337, 0.49, 0.275).into()),
            ..default()
        })
        .insert(Name::new("Ground"));

    commands
        .spawn_bundle(SceneBundle {
            scene: game_assets.construction_site_scene.clone(),
            transform: Transform::from_xyz(-1.0, 0.0, 0.0),
            ..default()
        })
        .insert(Name::new("Construction Site"));

    commands
        .spawn_bundle(SceneBundle {
            scene: game_assets.tower_scene.clone(),
            transform: Transform::from_xyz(1.0, 0.0, 0.0),
            ..default()
        })
        .insert(Tower {
            shooting_range: 2.0,
            shooting_timer: Timer::from_seconds(1.0, true),
            bullet_offset: Vec3::new(0., 1., 0.5),
        })
        .insert(Name::new("Tower"));

    // More Light!
    commands.insert_resource(AmbientLight {
        color: Color::GOLD,
        brightness: 0.2,
    });

    commands
        .spawn_bundle(PointLightBundle {
            point_light: PointLight {
                intensity: 1600.0,
                shadows_enabled: true,
                color: Color::rgb(196. / 255., 170. / 255., 132. / 255.),
                ..default()
            },
            transform: Transform::from_xyz(2.0, 3.0, 1.3),
            ..default()
        })
        .insert(Name::new("Light"));

    // Targets
    // TODO: make spawner for these!
    commands
        .spawn_bundle(SceneBundle {
            scene: game_assets.target_scene.clone(),
            transform: Transform::from_xyz(-2.0, 0.2, 1.5).with_scale(Vec3::new(0.38, 0.38, 0.38)),
            ..default()
        })
        .insert_bundle(PhysicsBundle::moving_entity(Vec3::new(0.2, 0.2, 0.2)))
        .insert(Target { speed: 0.2 })
        .insert(Health { value: 3 })
        .insert(Name::new("Target"));

    commands
        .spawn_bundle(SceneBundle {
            scene: game_assets.target_scene.clone(),
            transform: Transform::from_xyz(-2.0, 1.2, 1.5).with_scale(Vec3::new(0.38, 0.38, 0.38)),
            ..default()
        })
        .insert_bundle(PhysicsBundle::moving_entity(Vec3::new(0.4, 0.4, 0.4)))
        .insert(Target { speed: 0.4 })
        .insert(Health { value: 3 })
        .insert(Name::new("Target"));

    commands
        .spawn_bundle(SceneBundle {
            scene: game_assets.target_scene.clone(),
            transform: Transform::from_xyz(-1.8, 0.1, 1.5).with_scale(Vec3::new(0.38, 0.38, 0.38)),
            ..default()
        })
        .insert_bundle(PhysicsBundle::moving_entity(Vec3::new(0.4, 0.4, 0.4)))
        .insert(Target { speed: 0.3 })
        .insert(Health { value: 3 })
        .insert(Name::new("Target"));

    commands
        .spawn_bundle(SceneBundle {
            scene: game_assets.target_scene.clone(),
            transform: Transform::from_xyz(-2.3, 0.5, 1.5).with_scale(Vec3::new(0.38, 0.38, 0.38)),
            ..default()
        })
        .insert_bundle(PhysicsBundle::moving_entity(Vec3::new(0.4, 0.4, 0.4)))
        .insert(Target { speed: 0.5 })
        .insert(Health { value: 3 })
        .insert(Name::new("Target"));
}
