use bevy::prelude::*;
use rand::prelude::*;
use super::components::*;
use super::super::roads::components::{RoadNetwork, RoadKey};

pub fn spawn_building(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    road_network: Res<RoadNetwork>,
) {
    let mut rng = thread_rng();

    if let Some(address) = random_address(&mut rng, road_network) {

    }
}

fn random_address(rng: &mut ThreadRng, road_network: Res<RoadNetwork>) -> Option<Address> {
    let road_keys: Vec<RoadKey> = road_network.roads.keys().collect();

    if road_keys.is_empty() {
        return None;
    }

    let road = road_keys[rng.gen_range(0..road_keys.len())];
    let t = rng.gen_range(0.0..road_network.roads[road].nodes.len() as f32);

    Some(Address {
        road,
        t,
    })
}