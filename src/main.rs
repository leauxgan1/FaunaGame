
use bevy::prelude::*;
use bevy::math::Vec3;


#[derive(Bundle)]
struct PlayerBundle {
    transform: Transform,
    health: Health,
    stats: PlayerStats,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
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
struct PlayerStats {
    speed: f32,
    jump_height: f32,
    damage: i32,
}

#[derive(Component)]
struct Health {
    health: u32,
    max_health: u32,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let _player_id = commands.spawn(PlayerBundle{
        ..default()
    });
    // Base of map
    commands.spawn(PbrBundle {
        mesh: meshes.add(Circle::new(4.0)),
        material: materials.add(Color::WHITE),
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..default()
    });    
    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
        material: materials.add(Color::srgb_u8(124, 144, 255)),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
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
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.5,4.5,9.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

}

fn update_camera (
    mut camera_query: Query<&mut Transform>
) {
    for mut transform in &mut camera_query {
        transform.translation.x += 0.1;
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup,setup)
        .add_systems(Update,
            (
                update_camera,
            )
        )

        .run();
    println!("Hello, world!");
}
