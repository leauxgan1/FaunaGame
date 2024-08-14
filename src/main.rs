use bevy::prelude::*;
use bevy_rapier3d::control::KinematicCharacterController;
use bevy_rapier3d::prelude::*;
use bevy::math::Vec3;
use bevy::prelude::Time;


#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    health: Health,
    stats: PlayerStats,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            player: Player,
            health: Health {
                health: 100,
                max_health: 100,
            },
            stats: PlayerStats {
                speed: 1.0,
                jump_height: 5.0,
                damage: 1,
            }
        }
    }
}
#[derive(Component)]
struct Player;

#[allow(dead_code)]
#[derive(Component)]
struct PlayerStats {
    speed: f32,
    jump_height: f32,
    damage: i32,
}

#[allow(dead_code)]
#[derive(Component)]
struct Health {
    health: u32,
    max_health: u32,
}


fn handle_input(
    mut app_exit_events: ResMut<Events<bevy::app::AppExit>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        app_exit_events.send(bevy::app::AppExit::Success);
    }
}


fn update_camera (
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>, 
    player_query: Query<&Transform, With<Player>>,
) {
    // Update camera transform to match player transform
    if let Ok(mut camera_transform) = camera_query.get_single_mut() {
        if let Ok(player_transform) = player_query.get_single() {
            camera_transform.translation = player_transform.translation;
            camera_transform.rotation = player_transform.rotation;
        }
    }

}
fn player_movement (
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut kinematic_bodies: Query<(&mut Velocity, &Transform), With<Player>>,
) {
    if let Ok((mut velocity, transform)) = kinematic_bodies.get_single_mut() {
        let mut direction = Vec3::ZERO;
        if keys.pressed(KeyCode::KeyW) {
            direction += *transform.forward();
        }
        if keys.pressed(KeyCode::KeyA) {
            direction -= *transform.right();
        }
        if keys.pressed(KeyCode::KeyS) {
            direction -= *transform.forward();
        }
        if keys.pressed(KeyCode::KeyD) {
            direction += *transform.right();
        }

        
        let mut rotation_vec = Vec3::ZERO;
        if keys.pressed(KeyCode::ArrowLeft) {
            rotation_vec += Vec3::new(0.0,1.0,0.0)
        }
        if keys.pressed(KeyCode::ArrowRight) {
            rotation_vec += Vec3::new(0.0,-1.0,0.0)
        }
        if keys.pressed(KeyCode::ArrowUp) {

            rotation_vec += Vec3::new(1.0,0.0,0.0)
        }
        if keys.pressed(KeyCode::ArrowDown) {
            rotation_vec += Vec3::new(-1.0,0.0,0.0)
        }
        // Rotate character based on arrow keys
        let movement_vel = direction.normalize_or_zero() * 500.0 * time.delta_seconds(); 
        velocity.linvel = Vec3::new(movement_vel.x,velocity.linvel.y,movement_vel.z);
        velocity.angvel = rotation_vec.normalize_or_zero() * 100.0 * time.delta_seconds();
        

    }
}

fn setup (
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let _player_id = commands
        .spawn(RigidBody::Dynamic)
        .insert(Velocity { linvel: Vec3::ZERO, angvel: Vec3::ZERO } )
        .insert(LockedAxes::ROTATION_LOCKED_X | LockedAxes::ROTATION_LOCKED_Z)
        .insert(Collider::ball(0.5))
        .insert(PlayerBundle {
            ..default()
        })
        .insert( PbrBundle {
            transform: Transform::from_xyz(0.0,1.0,0.0),
            mesh: meshes.add(Cuboid::new(1.0,1.0,1.0)),
            material: materials.add(Color::WHITE),
            ..default()
        })
        .id();
    // Base of map
    commands
        .spawn(Collider::cuboid(100.0, 0.1, 100.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -2.0, 0.0)))
        .insert(PbrBundle { 
            mesh: meshes.add(Cuboid::new(100.0,0.1,100.0)),
            material: materials.add(Color::WHITE),
            ..default()
        });
    // cube
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::cuboid(0.5, 0.5, 0.5))
        .insert(Restitution::coefficient(0.5))
        .insert(PbrBundle { 
            transform: Transform::from_xyz(0.0,4.0,-5.0),
            mesh: meshes.add(Cuboid::new(1.1,1.1,1.1)),
            material: materials.add(Color::WHITE),
            ..default()
        });
    // Light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0,8.0,4.0),
        ..default()
    });

    // Camera
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(0.0,15.0,10.0).looking_at(Vec3::new(0.0,0.0,0.0),Vec3::Y),
            ..default()
        });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default() )
        .add_systems(Startup,setup)
        .add_systems(Update,
            (
                handle_input,
                player_movement,
                update_camera,
            ).chain(),
        )
        .run();
}
