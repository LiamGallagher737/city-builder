use bevy::prelude::*;
use rand::{Rng, thread_rng};
use rand::prelude::ThreadRng;

use crate::game::buildings::components::Address;
use crate::game::roads::components::RoadKey;
use crate::game::roads::road_pathfinding::RoutePoint::{Intersection, Road};

use super::components::*;
use super::super::roads::components::RoadNetwork;

pub fn vehicle_drive_system (
    road_network: Res<RoadNetwork>,
    keys: bevy::prelude::Res<bevy::input::Input<bevy::prelude::KeyCode>>,
    mut query: Query<(&mut Transform, &mut Vehicle)>
) {
    for (mut tf, mut vehicle) in query.iter_mut() {

        // if keys.just_pressed(bevy::prelude::KeyCode::G) {
        //     vehicle.current_address = Address {
        //         road: road_network.roads.keys().collect::<Vec<RoadKey>>()[0],
        //         t: 0.0,
        //     };
        // }

        if keys.just_pressed(bevy::prelude::KeyCode::H) {
            println!("attempting pathfinding");

            let mut rng = thread_rng();

            vehicle.route = road_network.calculate_path(&random_address(&mut rng, &road_network).unwrap(), &random_address(&mut rng, &road_network).unwrap()).unwrap();
            
            // vehicle.route = road_network.calculate_path(&vehicle.current_address, &Address {
            //     road: road_network.roads.keys().last().unwrap(),
            //     t: 5.0,
            // }).unwrap();
        }

        if vehicle.route.is_empty() {
            continue;
        }

        // REWORK THIS!!!

        vehicle.current_address.t += 0.025;

        let road = &road_network.roads[vehicle.current_address.road];

        if vehicle.current_address.t.ceil() as usize + 1 > road.nodes.len() {
            vehicle.current_address.t = 0.0;
            if let Some(point) = vehicle.route.pop() {
                vehicle.point = Some(point);
            } else {
                println!("Reached Destination");
            }
        }

        if let Some(point) = &vehicle.point {
            match point {
                Intersection(_) => {
                    std::thread::sleep_ms(500);
                    vehicle.current_address.t = f32::MAX;
                },
                Road(road) => {
                    tf.translation = road_network.roads[*road].calculate_point_at_distance(vehicle.current_address.t).unwrap().0;
                },
            }
        }

        // if let Some((position, rotation)) = road.calculate_point_at_distance(vehicle.current_address.t) {
        //     tf.translation = position;
        //     tf.rotation = rotation;
        // }

        // if vehicle.current_address.road == RoadKey::default() {
        //     for road in road_network.roads.keys() {
        //         vehicle.current_address.road = road;
        //         break;
        //     }
        //     return;
        // }

        // vehicle.current_address.t += 0.025;

        // if let Some(road) = road_network.roads.get(vehicle.current_address.road) {

        //     if vehicle.current_address.t.ceil() as usize + 1 > road.nodes.len() {
        //         vehicle.current_address.t = vehicle.current_address.t - vehicle.current_address.t.ceil();

        //         if road_network.intersections[road.intersection_end].roads.len() == 0 {
        //             vehicle.current_address.road = RoadKey::default();
        //         }

        //         for road in &road_network.intersections[road.intersection_end].roads {
        //             vehicle.current_address.road = road.0;
        //             break;
        //         }
        //     }

        //     if let Some((position, rotation)) = road.calculate_point_at_distance(vehicle.current_address.t) {
        //         tf.translation = position;
        //         tf.rotation = rotation;
        //     }

        // } else {
        //     vehicle.current_address.road = RoadKey::default();
        // }

    }
}

fn random_address(rng: &mut ThreadRng, road_network: &Res<RoadNetwork>) -> Option<Address> {
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