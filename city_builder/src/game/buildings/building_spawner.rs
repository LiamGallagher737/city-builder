use bevy::prelude::*;
use rand::{Rng, thread_rng};
use super::super::roads::components::{RoadNetwork, RoadKey};

pub fn spawn_building(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    road_network: Res<RoadNetwork>,
    keys: Res<Input<KeyCode>>,
) {
    if !keys.just_pressed(KeyCode::G) {
        return;
    }

    let mut rng = thread_rng();

    let road_keys: Vec<RoadKey> = road_network.roads.keys().clone().collect();
    let road = road_keys[rng.gen_range(0..road_keys.len())];
    let t = rng.gen_range(0.5..road_network.roads[road].nodes.len() as f32 - 0.5);

    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 4.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_translation(road_network.roads[road].calculate_point_at_distance(t).unwrap().0),
        ..Default::default()
    });
}