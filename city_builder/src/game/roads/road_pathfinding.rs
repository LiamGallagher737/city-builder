use super::components::{RoadNetwork, RoadKey, Road};
use super::super::buildings::components::Address;

impl RoadNetwork {
    pub fn calculate_path(self: &Self, start: Address, end: Address) -> Option<Vec<RoadKey>> {
        // dijkstra's algorithm

        // https://youtu.be/EFg3u_E6eHU

        None
    }
}

impl Road {
    pub fn calculate_dijkstra_value(self: &Self) -> u32 {
        // cant b negative
        5
    }
}