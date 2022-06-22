use bevy::utils::hashbrown::{HashMap, HashSet};

use super::components::{RoadNetwork, RoadKey, Road, Intersection, IntersectionKey};
use super::super::buildings::components::Address;

impl RoadNetwork {
    pub fn calculate_path(self: &Self, start: Address, end: Address) -> Option<Vec<RoadKey>> {
        // dijkstra's algorithm
        // https://youtu.be/EFg3u_E6eHU


        let mut lowest_unexplored_values = HashMap::<IntersectionKey, usize>::new();

        let road = &self.roads[start.road];
        lowest_unexplored_values.insert(road.intersection_start,  0);
        lowest_unexplored_values.insert(road.intersection_end, 0);

        let mut explored_intersections = HashMap::<IntersectionKey, usize>::new();

        None
    }
}

impl Road {
    pub fn calculate_dijkstra_value(self: &Self) -> usize {
        self.nodes.len()
    }
}