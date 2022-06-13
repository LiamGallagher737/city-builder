use bevy::prelude::*;
use bevy::window::PresentMode;

mod game;
mod lib;

fn main() {

    App::new()

        // Window Settings
        .insert_resource(WindowDescriptor {
            title: "city_builder.exe".to_string(),
            present_mode: PresentMode::Immediate,
            // width: 750.0,
            // height: 500.0,
            ..Default::default()
        })

        // Bevy Plugins
        .add_plugins(DefaultPlugins)

        // Game Plugins
        .add_plugins(game::GamePlugins)

        // Performance Metrics
        // .add_plugin(bevy::diagnostic::LogDiagnosticsPlugin::default())
        // .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())

        // Debugging
        .add_plugin(bevy::pbr::wireframe::WireframePlugin)

        // Startup Settings
        .add_startup_system(scene_setup) 

        // Run the app
        .run();
}

fn scene_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut wireframe_config: ResMut<bevy::pbr::wireframe::WireframeConfig>, // Wireframe Mode Debugging
) {

    wireframe_config.global = true; // Wireframe Mode Debugging

    // Camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(0.0, 10.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Directional Light
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight { 
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            rotation: Quat::from_euler(EulerRot::XYZ, -140.0, 120.0, 0.0),
            ..default()
        },
        ..default()
    });

    // Ground Plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 50.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
}