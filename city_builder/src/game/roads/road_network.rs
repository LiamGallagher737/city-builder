use bevy::prelude::*;

use super::components::*;

pub const ROAD_NODE_DISTANCE: f32 = 2.0;
pub const ROAD_NODE_DISTANCE_SQ: f32 = ROAD_NODE_DISTANCE * ROAD_NODE_DISTANCE;

pub fn road_network_startup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    let road_network = RoadNetwork {
        roads: vec![],
        intersections: vec![Intersection::new(Vec3::ZERO)],
    };

    commands.insert_resource(road_network);

    // Cube
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..Default::default()
    })
    .insert(RoadCreator::default());
}