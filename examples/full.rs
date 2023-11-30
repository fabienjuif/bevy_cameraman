use bevy::sprite::MaterialMesh2dBundle;
use bevy::{prelude::*, DefaultPlugins};
use bevy_cameraman::{CameraBundle, CameraDebugPlugin, CameraPlugin, Cameraman, Target};

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        // --- camera ---
        .add_plugins((
            CameraPlugin,
            // CameraDebugPlugin, // uncomment this to see debug mode
        ))
        // --- systems ---
        .add_systems(Startup, (setup_example, setup_rectangles))
        .add_systems(Update, (keyboard_movements, update_axes))
        .run();
}

#[derive(Component)]
struct LocalPlayer;

fn setup_example(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // 1. spawn your entity to follow
    let entity = commands
        .spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(30.).into()).into(),
                material: materials.add(ColorMaterial::from(Color::rgb(0.8, 0.3, 0.3))),
                transform: Transform::from_translation(Vec3::new(-150., 0., 0.)),
                ..default()
            },
            LocalPlayer {},
            // you need this to tell the camera to focus this entity
            Target,
        ))
        .id();

    // 2. spawn your cameraman and make it follow previou entity
    // you should play with the cameraman values!
    commands.spawn(CameraBundle::new(
        Cameraman::new(entity, Vec2::new(50.0, 20.0), Vec3::ONE * 0.8),
        Camera2dBundle::default(),
    ));
}

// -- for the demo -- //
fn setup_rectangles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(30.).into()).into(),
        material: materials.add(ColorMaterial::from(Color::rgb(0.3, 0.3, 0.8))),
        transform: Transform::from_translation(Vec3::new(-150., -200., 0.)),
        ..default()
    });

    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(30.).into()).into(),
        material: materials.add(ColorMaterial::from(Color::rgb(0.3, 0.3, 0.8))),
        transform: Transform::from_translation(Vec3::new(300., 120., 0.)),
        ..default()
    });
}

fn keyboard_movements(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query_player: Query<&mut Transform, With<LocalPlayer>>,
) {
    for mut transform in &mut query_player {
        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            transform.translation.x -= 200. * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            transform.translation.x += 200. * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            transform.translation.y -= 200. * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            transform.translation.y += 200. * time.delta_seconds();
        }
    }
}

fn update_axes(
    time: Res<Time>,
    gamepads: Res<Gamepads>,
    axes: Res<Axis<GamepadAxis>>,
    mut query: Query<&mut Transform, With<LocalPlayer>>,
) {
    for gamepad in gamepads.iter() {
        let left_stick_x = axes
            .get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickX))
            .unwrap();
        let mut moved = false;
        if left_stick_x.abs() > 0.1 {
            moved = true;
            for mut transform in &mut query {
                transform.translation.x += left_stick_x * 200. * time.delta_seconds();
            }
        }

        let left_stick_y = axes
            .get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickY))
            .unwrap();
        if left_stick_y.abs() > 0.1 {
            moved = true;
            for mut transform in &mut query {
                transform.translation.y += left_stick_y * 200. * time.delta_seconds();
            }
        }

        if moved {
            for mut transform in &mut query {
                transform.rotation = Quat::from_axis_angle(
                    Vec3::new(0., 0., 1.),
                    (-left_stick_x).atan2(left_stick_y),
                );
            }
        }
    }
}
