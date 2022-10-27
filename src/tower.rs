use crate::prelude::*;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Tower {
    pub shooting_range: f32,
    pub shooting_timer: Timer,
    pub bullet_offset: Vec3,
}

#[derive(Default)]
pub struct TowerPlugin;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Tower>().add_system(tower_shooting);
    }
}

fn tower_shooting(
    mut commands: Commands,
    mut towers: Query<(Entity, &mut Tower, &GlobalTransform)>,
    targets: Query<&GlobalTransform, With<Target>>,
    //    bullet_assets: Res<GameAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    time: Res<Time>,
) {
    for (tower_entity, mut tower, transform) in &mut towers {
        tower.shooting_timer.tick(time.delta());
        if tower.shooting_timer.just_finished() {
            let bullet_spawn = transform.translation() + tower.bullet_offset;

            // get direction of closest target
            let direction = targets
                .iter()
                .min_by_key(|target_transform| {
                    FloatOrd(Vec3::distance(target_transform.translation(), bullet_spawn))
                })
                .map(|closest_target| closest_target.translation() - bullet_spawn);

            // if there is at least one target, start shooting into its direction
            if let Some(direction) = direction {
                commands.entity(tower_entity).with_children(|commands| {
                    commands
                        // .spawn_bundle(SceneBundle {
                        //     scene: bullet_assets.bullet_scene.clone(),
                        //     transform: Transform::from_translation(tower.bullet_offset),
                        //     ..Default::default()
                        // })
                        .spawn_bundle(PbrBundle {
                            mesh: meshes.add(Mesh::from(shape::Icosphere {
                                radius: 0.05,
                                ..default()
                            })),
                            material: materials.add(Color::rgb(0.97, 0.67, 0.67).into()),
                            transform: Transform::from_translation(tower.bullet_offset),
                            ..default()
                        })
                        .insert(Lifetime {
                            timer: Timer::from_seconds(1000.5, false),
                        })
                        .insert(Bullet {
                            direction,
                            speed: 2.5,
                        })
                        .insert(Name::new("Bullet"))
                        .insert_bundle(PhysicsBundle::moving_entity(Vec3::new(0.2, 0.2, 0.2)));
                });
            }
        }
    }
}
