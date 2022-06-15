use bevy::prelude::*;
use super::components::*;
use super::super::roads::components::{RoadNetwork, RoadKey};

pub fn vehicle_drive_system (
    road_network: Res<RoadNetwork>,
    mut query: Query<(&mut Transform, &mut Vehicle)>
) {
    for (mut tf, mut vehicle) in query.iter_mut() {

        if vehicle.road == RoadKey::default() {
            for road in road_network.roads.keys() {
                vehicle.road = road;
                break;
            }
            return;
        }

        vehicle.t += 0.025;

        if let Some(road) = road_network.roads.get(vehicle.road) {

            if vehicle.t.ceil() as usize + 1 > road.nodes.len() {
                vehicle.t = vehicle.t - vehicle.t.ceil();

                if road_network.intersections[road.intersection_end].roads.len() == 0 {
                    vehicle.road = RoadKey::default();
                }

                for road in &road_network.intersections[road.intersection_end].roads {
                    vehicle.road = road.0;
                    break;
                }
            }

            if let Some((position, rotation)) = road.calculate_point_at_distance(vehicle.t) {
                tf.translation = position;
                tf.rotation = rotation;
            }

        } else {
            vehicle.road = RoadKey::default();
        }

    }
}