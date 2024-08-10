use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy::math::Vec3;
use bevy::prelude::Time;


#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    transform: Transform,
    health: Health,
    stats: PlayerStats,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            player: Player,
            transform: Transform::from_xyz(0.0,0.0,0.0),
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

fn setup (
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let _player_id = commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::cuboid(0.5, 0.5, 0.5))
        .insert(Restitution::coefficient(0.5))
        .insert(PlayerBundle {
            transform: Transform::from_xyz(0.0,0.0,0.0).looking_at(Vec3::new(0.0,4.0,0.0),Vec3::new(0.0,1.0,0.0)),
            ..default()
        }
    );
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
            mesh: meshes.add(Cuboid::new(0.5,0.5,0.5)),
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
            transform: Transform::default(),
            ..default()
        });

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
    mut player_query: Query<(&mut Transform,&PlayerStats), With<Player>>,
) {

    let mut player_info = player_query.get_single_mut().unwrap();
    // let mut movement_vec = Vec3::new(0.0,0.0,0.0);
    let mut rotation_quat = Quat::IDENTITY;

    // Move the player based on keyboard input
    // if keys.pressed(KeyCode::KeyW) {
    //     movement_vec.z -= 1.0;
    // }
    // if keys.pressed(KeyCode::KeyA) {
    //     movement_vec.x -= 1.0;
    // }
    // if keys.pressed(KeyCode::KeyS) {
    //     movement_vec.z += 1.0;
    // }
    // if keys.pressed(KeyCode::KeyD) {
    //     movement_vec.x += 1.0;
    // }
    // Rotate the player based on keyboard input
    if keys.pressed(KeyCode::ArrowLeft) {
        rotation_quat = Quat::from_rotation_y(1.0);
    }
    if keys.pressed(KeyCode::ArrowRight) {
        rotation_quat = Quat::from_rotation_y(-1.0);
    }
    if keys.pressed(KeyCode::ArrowUp) {
        rotation_quat = Quat::from_rotation_x(1.0);
    }
    if keys.pressed(KeyCode::ArrowDown) {
        rotation_quat = Quat::from_rotation_x(-1.0);
    }


    // let scalar = time.delta_seconds() * player_info.1.speed;
    let rotation_scalar = time.delta_seconds() * 10.0;
    // let rotated_vec = player_info.0.rotation.mul_vec3(movement_vec) * scalar;
    // let xy_vec = Vec3::new(rotated_vec.x,0.0,rotated_vec.z);
    // player_info.0.translation += xy_vec;
    player_info.0.rotation *= rotation_quat * rotation_scalar;
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
    println!("Hello, world!");
}
