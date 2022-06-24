use bevy::utils::hashbrown::HashMap;

use super::components::{RoadNetwork, Road, IntersectionKey, RoadKey};
use super::super::buildings::components::Address;

impl RoadNetwork {
    pub fn calculate_path(self: &Self, start: &Address, destination: &Address) -> Option<Vec<RoutePoint>> {

        // dijkstra's algorithm
        // https://youtu.be/EFg3u_E6eHU

        // usually in dijkstra the lowest value is better but 
        // in this implementaation im using higher values as better

        let mut explored_intersections = HashMap::<IntersectionKey, DijkstraVertexData>::new();
        let mut unexplored_intersections = HashMap::<IntersectionKey, DijkstraVertexData>::new();

        let start_road = &self.roads[start.road];
        unexplored_intersections.insert(start_road.intersection_start,  DijkstraVertexData::default());
        unexplored_intersections.insert(start_road.intersection_end, DijkstraVertexData::default());

        while let Some((key, vertex)) = pop_best_value(&unexplored_intersections) {
            unexplored_intersections.remove(&key);
            explored_intersections.insert(key, vertex.clone());

            if self.roads[destination.road].connects_to_intersection(&key) {
                // We found the route
                let mut route = vec![];

                let mut last_key = key;
                loop {
                    if let Some(last) = explored_intersections[&last_key].last {
                        route.push(RoutePoint::Intersection(last.0));
                        route.push(RoutePoint::Road(last.1));
                        last_key = last.0;
                    } else {
                        break;
                    }
                }

                return Some(route);
            }

            for (road_key, cap) in &self.intersections[key].roads {
                if let Some(road) = self.roads.get(*road_key) {
                    let other_intersection = road.get_other_intersection(&cap);
                    
                    if explored_intersections.contains_key(&other_intersection) {
                        continue;
                    }
    
                    let value = vertex.value - road.calculate_dijkstra_value();
    
                    if let Some(v) = unexplored_intersections.get_mut(&other_intersection) {
                        if v.value < value {
                            v.value = value;
                        }
                        continue;
                    }
    
                    unexplored_intersections.insert(other_intersection, DijkstraVertexData::new(Some((key, *road_key)), value));
                } else {
                    println!("{:#?}", *road_key);
                }
            }
        }

        None
    }
}

fn pop_best_value(unexplored_intersections: &HashMap::<IntersectionKey, DijkstraVertexData>) -> Option<(IntersectionKey, DijkstraVertexData)> {
    if unexplored_intersections.is_empty() {
        return None;
    }

    let mut best = (IntersectionKey::default(), DijkstraVertexData::default());
    for entry in unexplored_intersections {
        if entry.1.value >= best.1.value {
            best = (*entry.0, entry.1.clone());
        }
    }

    Some(best)
}

pub enum RoutePoint {
    Intersection(IntersectionKey),
    Road(RoadKey),
}

impl Road {
    pub fn calculate_dijkstra_value(self: &Self) -> usize {
        (self.speed as usize * 100) / self.nodes.len()
    }
}

#[derive(Clone, Debug)]
struct DijkstraVertexData {
    last: Option<(IntersectionKey, RoadKey)>,
    value: usize,
}

impl Default for DijkstraVertexData {
    fn default() -> Self {
        Self {
            last: None,
            value: usize::MAX,
        }
    }
}

impl DijkstraVertexData {
    fn new(last: Option<(IntersectionKey, RoadKey)>, value: usize) -> Self {
        Self {
            last,
            value,
        }
    }
}