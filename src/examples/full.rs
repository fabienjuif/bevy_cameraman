use bevy::{prelude::*, DefaultPlugins};
use bevy_cameraman::CameraPlugin;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        // --- camera ---
        .add_plugins((
            CameraPlugin,
            CameraDebugPlugin, // can be omited
        ))
        // --- systems ---
        .add_systems(Startup, setup)
        .add_systems(Update, (keyboard_movements))
        .run();
}

#[derive(Component)]
struct LocalPlayer;

fn setup(
    mut commands: Commands,
    teams: Res<Teams>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // 1. spawn your entity to follow
    let entity = commands
        .spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(30.).into()).into(),
                material: materials.add(ColorMaterial::from(team.color)),
                transform: Transform::from_translation(Vec3::new(-150., 0., 0.)),
                ..default()
            },
            LocalPlayer {},
        ))
        .id();

    // 2. spawn your cameraman and make it follow previou entity
    // you should play with the cameraman values!
    commands.spawn(CameraBundle::new(
        Cameraman::new(entity, Vec2::new(50.0, 20.0), Vec3::ONE * 0.8),
        Camera2dBundle::default(),
    ));
}

fn keyboard_movements(
    keyboard_input: Res<Input<KeyCode>>,
    mut query_player: Query<&mut Transform, With<LocalPlayer>>,
    time: Res<Time>,
) {
    for mut transform in &mut query_player {
        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            transform.translation.x -= JOYSTICK_SCALE * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            transform.translation.x += JOYSTICK_SCALE * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            transform.translation.y -= JOYSTICK_SCALE * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            transform.translation.y += JOYSTICK_SCALE * time.delta_seconds();
        }
    }
}
