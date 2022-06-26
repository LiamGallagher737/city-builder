use bevy::utils::hashbrown::HashMap;
use priority_queue::PriorityQueue;
use std::cmp::Ordering;

use super::super::buildings::components::Address;
use super::components::{IntersectionKey, Road, RoadNetwork};

impl RoadNetwork {
    pub fn calculate_path(
        &self,
        start: &Address,
        destination: &Address,
    ) -> Option<Vec<IntersectionKey>> {
        // Using the A* algorithm
        // https://youtu.be/ySN5Wnu88nE

        let mut explored_points = HashMap::<IntersectionKey, AStarPoint>::new();
        let mut queue = PriorityQueue::<IntersectionKey, AStarPoint>::new();

        let destination_position = self.get_address_position(destination).unwrap();
        let h_cost = |key| destination_position.distance_squared(self.intersections[key].position);

        let start_road = &self.roads[start.road];

        queue.push(
            start_road.intersection_start,
            AStarPoint {
                previous_intersection: None,
                g: start.t,
                h: h_cost(start_road.intersection_start),
            },
        );

        queue.push(
            start_road.intersection_end,
            AStarPoint {
                previous_intersection: None,
                g: start_road.get_length() - start.t,
                h: h_cost(start_road.intersection_end),
            },
        );

        let mut destination_intersection = None;
        while let Some((key, data)) = queue.pop() {
            explored_points.insert(key, data.clone());
            let intersection = &self.intersections[key];

            if self.roads[destination.road].has_intersection(&key) {
                destination_intersection = Some(key);
                break;
            }

            for (road_key, road_cap) in &intersection.connections {
                print!("{esc}c", esc = 27 as char);
                println!(
                    "\n\n\n Queue: {:#?} \n\n Explored: {:#?} \n\n\n",
                    queue, explored_points
                );

                if self.roads.get(*road_key).is_none() {
                    // Invalid keys are appearing, onced fixed this wont be needed
                    continue;
                }

                let road = &self.roads[*road_key];
                let other_intersection = road.get_other_intersection(road_cap);

                if explored_points.contains_key(&other_intersection) {
                    continue;
                }

                let new_point = AStarPoint {
                    previous_intersection: Some(key),
                    g: road.get_length(),
                    h: h_cost(other_intersection),
                };

                // Replace the clone with something else
                if let Some((k, data)) = queue.clone().get(&other_intersection) {
                    if data.f_cost() > new_point.f_cost() {
                        queue.remove(k);
                    } else {
                        continue;
                    }
                }

                queue.push(other_intersection, new_point);
            }
        }

        if let Some(key) = destination_intersection {
            let mut route = vec![key];

            let mut next_intersection = key;
            while let Some(intersection) = explored_points[&next_intersection].previous_intersection
            {
                route.push(intersection);
                next_intersection = intersection;
            }

            println!("Route: {:#?}", route);

            return Some(route);
        }

        None
    }
}

impl Road {
    pub fn get_length(&self) -> f32 {
        self.nodes.len() as f32 * 2.0
    }
}

#[derive(Debug, Clone, PartialEq)]
struct AStarPoint {
    // The intersection we came from
    previous_intersection: Option<IntersectionKey>,
    // Cost of this path
    g: f32,
    // Square euclidean distance to end
    h: f32,
}

impl Eq for AStarPoint {}

impl AStarPoint {
    fn f_cost(&self) -> f32 {
        (self.g * self.g) + self.h
    }
}

impl PartialOrd for AStarPoint {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for AStarPoint {
    fn cmp(&self, other: &Self) -> Ordering {
        // PriorityQueue sorts by largest so here I return the opposite
        // to make sure we get the lowest f cost and not the highest
        if self.f_cost() > other.f_cost() {
            Ordering::Less
        } else if self.f_cost() < other.f_cost() {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}
