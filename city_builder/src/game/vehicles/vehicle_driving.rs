use bevy::prelude::*;
use rand::prelude::ThreadRng;
use rand::{thread_rng, Rng};

use crate::game::buildings::components::Address;
use crate::game::roads::components::RoadKey;

use super::super::roads::components::RoadNetwork;
use super::components::*;

pub fn vehicle_drive_system(
    road_network: Res<RoadNetwork>,
    keys: bevy::prelude::Res<bevy::input::Input<bevy::prelude::KeyCode>>,
    mut query: Query<(&mut Transform, &mut Vehicle)>,
) {
    for (mut _tf, vehicle) in query.iter_mut() {
        if keys.just_pressed(bevy::prelude::KeyCode::H) {
            println!("attempting pathfinding");
            let mut rng = thread_rng();
            road_network.calculate_path(
                &random_address(&mut rng, &road_network).unwrap(),
                &random_address(&mut rng, &road_network).unwrap(),
            );
        }

        if vehicle.route.is_empty() {
            continue;
        }
    }
}

fn random_address(rng: &mut ThreadRng, road_network: &Res<RoadNetwork>) -> Option<Address> {
    let road_keys: Vec<RoadKey> = road_network.roads.keys().collect();

    if road_keys.is_empty() {
        return None;
    }

    let road = road_keys[rng.gen_range(0..road_keys.len())];
    let t = rng.gen_range(0.0..(road_network.roads[road].nodes.len() as f32 - 1.0));

    Some(Address { road, t })
}
