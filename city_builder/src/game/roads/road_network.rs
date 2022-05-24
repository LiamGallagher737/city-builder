use bevy::prelude::*;

pub struct RoadNetwork {
    // roads: HashMap< >
}

impl RoadNetwork {
    fn new() -> Self {
        Self {

        }
    }
}

pub fn road_network_startup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.insert_resource(RoadNetwork::new());

    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
    // cube
    let mesh =  meshes.add(Mesh::from(shape::Cube { size: 1.0 }));
    let mat = materials.add(Color::rgb(0.8, 0.7, 0.6).into());
    let mut i = 0;
    while i < 100 {
        commands.spawn_bundle(PbrBundle {
            mesh: mesh,
            material: mat,
            transform: Transform::from_xyz(0.0, 0.5, -i as f32),
            ..default()
        });
        i += 1;
    }
    // light
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}