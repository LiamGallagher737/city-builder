use bevy::prelude::*;

use crate::game::buildings::components::Address;

use super::components::*;
use super::super::roads::components::RoadNetwork;

static mut counter: usize = 0;

pub fn vehicle_drive_system (
    road_network: Res<RoadNetwork>,
    mut query: Query<(&mut Transform, &mut Vehicle)>
) {
    for (mut tf, mut vehicle) in query.iter_mut() {

        unsafe {
            counter += 1;

            if counter > 1000 {
                vehicle.current_address = Address {
                    road: road_network.roads.keys().last().unwrap(),
                    t: 0.0,
                };
            }

            if counter > 5000 {
                println!("attempting pathfinding");
                vehicle.route = road_network.calculate_path(&vehicle.current_address, &Address {
                    road: road_network.roads.keys().last().unwrap(),
                    t: 5.0,
                }).unwrap();
            }

        }

        if vehicle.route.is_empty() {
            continue;
        }

        vehicle.current_address.t += 0.025;

        let road = &road_network.roads[vehicle.current_address.road];

        if vehicle.current_address.t.ceil() as usize + 1 > road.nodes.len() {
            vehicle.current_address.t = 0.0;
            if let Some(intersection) = vehicle.route.pop() {
                // vehicle.current_address.road = intersection.common_road(vehicle., road_network.as_ref());
                tf.translation = road_network.intersections[intersection].position;
            } else {
                println!("Reached Destination");
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