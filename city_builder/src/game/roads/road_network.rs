use bevy::prelude::*;

use super::components::*;

pub const ROAD_NODE_DISTANCE: u8 = 2;
pub const ROAD_NODE_DISTANCE_USIZE: usize = ROAD_NODE_DISTANCE as usize;
pub const ROAD_NODE_DISTANCE_SQ: u8 = ROAD_NODE_DISTANCE * ROAD_NODE_DISTANCE;

pub fn road_network_startup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.insert_resource(RoadNetwork::new());

    // Cube
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..Default::default()
    })
    .insert(RoadCreator::default());
}