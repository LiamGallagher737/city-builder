use bevy::prelude::*;
use slotmap::SlotMap;

use super::components::*;

pub const ROAD_NODE_DISTANCE: f32 = 2.0;
pub const ROAD_NODE_DISTANCE_SQ: f32 = ROAD_NODE_DISTANCE * ROAD_NODE_DISTANCE;
pub const INTERSECTION_RADIUS: f32 = 4.0;
pub const INTERSECTION_RADIUS_SQ: f32 = INTERSECTION_RADIUS * INTERSECTION_RADIUS;

pub fn road_network_startup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    let road_network = RoadNetwork {
        roads: SlotMap::with_key(),
        intersections: SlotMap::with_key(),
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