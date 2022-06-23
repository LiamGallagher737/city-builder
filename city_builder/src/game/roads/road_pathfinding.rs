use bevy::utils::hashbrown::HashMap;

use super::components::{RoadNetwork, RoadKey, Road, IntersectionKey};
use super::super::buildings::components::Address;

impl RoadNetwork {
    pub fn calculate_path(self: &Self, start: Address, end: Address) -> Option<Vec<RoadKey>> {

        // dijkstra's algorithm
        // https://youtu.be/EFg3u_E6eHU

        // usually in dijkstra the lowest value is better but 
        // in this implementaation im using higher values as better

        let mut explored_intersections = HashMap::<IntersectionKey, (IntersectionKey, usize)>::new();
        let mut unexplored_intersections = HashMap::<IntersectionKey, usize>::new();

        let start_road = &self.roads[start.road];
        unexplored_intersections.insert(start_road.intersection_start,  usize::MAX);
        unexplored_intersections.insert(start_road.intersection_end, usize::MAX);

        let end_road = &self.roads[end.road];
        let is_destination = |key| {
            key == end_road.intersection_start || key == end_road.intersection_end
        };

        let pop_best_value = {

            if unexplored_intersections.is_empty() {
                return None;
            }

            let mut best_key = IntersectionKey::default();
            let mut best_value = usize::MAX;
            for (k, v) in &unexplored_intersections {
                if v >= &best_value {
                    best_key = *k;
                    best_value = *v;
                }
            }
            unexplored_intersections.remove(&best_key);
            Some((best_key, best_value))
        };

        while let Some((key, value)) = pop_best_value {
            if is_destination(key) {
                println!("Found Destination");
                break;
            }

            for (road, _) in &self.intersections[key].roads {
                let road = &self.roads[*road];
                let intersection = road.get_other_intersection(&key);

                if explored_intersections.contains_key(&intersection) {
                    continue;
                }

                let new_value = value + road.calculate_dijkstra_value();

                if let Some(value) = unexplored_intersections.get_mut(&intersection) {
                    if *value < new_value {
                        *value = new_value;
                        continue;
                    }
                }

                unexplored_intersections.insert(intersection, new_value);
            }
        }

        None
    }
}

impl Road {
    pub fn calculate_dijkstra_value(self: &Self) -> usize {
        self.speed as usize / self.nodes.len()
    }
}