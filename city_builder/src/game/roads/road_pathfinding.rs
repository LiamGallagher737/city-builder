use bevy::utils::hashbrown::HashMap;

use super::components::{RoadNetwork, Road, IntersectionKey, RoadKey};
use super::super::buildings::components::Address;

impl RoadNetwork {
    // pub fn calculate_path(self: &Self, start: &Address, end: &Address) -> Option<Vec<IntersectionKey>> {

    //     // dijkstra's algorithm
    //     // https://youtu.be/EFg3u_E6eHU

    //     // usually in dijkstra the lowest value is better but 
    //     // in this implementaation im using higher values as better

    //     println!("pathfind start");

    //     let mut explored_intersections = HashMap::<IntersectionKey, DijkstraVertexData>::new();
    //     let mut unexplored_intersections = HashMap::<IntersectionKey, DijkstraVertexData>::new();

        // let start_road = &self.roads[start.road];
        // unexplored_intersections.insert(start_road.intersection_start,  DijkstraVertexData::default());
        // unexplored_intersections.insert(start_road.intersection_end, DijkstraVertexData::default());

    //     let end_road = &self.roads[end.road];
    //     let is_destination = |key: &IntersectionKey| {
    //         *key == end_road.intersection_start || *key == end_road.intersection_end
    //     };

    //     let pop_best_value = {

    //         if unexplored_intersections.is_empty() {
    //             return None;
    //         }

    //         let mut best_key = IntersectionKey::default();
    //         let mut best_value = DijkstraVertexData::default();
    //         for (k, v) in &unexplored_intersections {
    //             if v.value >= best_value.value {
    //                 best_key = *k;
    //                 best_value = v.clone();
    //             }
    //         }
    //         unexplored_intersections.remove(&best_key);
    //         Some((best_key, best_value))
    //     };

    //     let mut destination_key = None;
    //     while let Some((k, v)) = &pop_best_value {

    //         explored_intersections.insert(*k, v.clone());

    //         if is_destination(k) {
    //             println!("Found Destination");
    //             destination_key = Some(k);
    //             break;
    //         }

    //         for (road, _) in &self.intersections[*k].roads {
    //             let road = &self.roads[*road];
    //             let intersection = road.get_other_intersection(&k);

    //             if explored_intersections.contains_key(&intersection) {
    //                 continue;
    //             }

    //             let new_value = v.value + road.calculate_dijkstra_value();

    //             if let Some(vertex) = unexplored_intersections.get_mut(&intersection) {
    //                 if vertex.value < new_value {
    //                     vertex.value = new_value;
    //                     vertex.previous_intersection = *k;
    //                     continue;
    //                 }
    //             }

    //             unexplored_intersections.insert(intersection, DijkstraVertexData::new(*k, new_value));
    //         }
    //     }

    //     if let Some(key) = destination_key {

    //         println!("{:#?}", explored_intersections);

    //         let mut path = vec![];
    //         let mut last_intersection = *key;
    //         loop {
    //             let intersection = explored_intersections[&last_intersection].previous_intersection;
    //             path.push(intersection);

    //             if start_road.connects_to_intersection(&intersection) {
    //                 break;
    //             }

    //             last_intersection = intersection;
    //         }

    //         return Some(path);

    //     } else {
    //         return None;
    //     }

    // }

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

        let pop_best_value = {
            if unexplored_intersections.is_empty() {
                return None
            }

            let mut best = (IntersectionKey::default(), DijkstraVertexData::default());
            for entry in &unexplored_intersections {
                if entry.1.value >= best.1.value {
                    best = (*entry.0, entry.1.clone());
                }
            }

            unexplored_intersections.remove(&best.0);
            Some(best)
        };

        while let Some((key, vertex)) = &pop_best_value {
            explored_intersections.insert(*key, vertex.clone());

            if self.roads[destination.road].connects_to_intersection(&key) {
                // We found the route
                let mut route = vec![];

                let mut last_key = key;
                loop {
                    if let Some(last) = explored_intersections[&last_key].last {
                        route.push(RoutePoint::Intersection(last.0));
                        route.push(RoutePoint::Road(last.1));
                    } else {
                        break;
                    }
                }

                return Some(route);
            }

            for (road_key, cap) in &self.intersections[*key].roads {
                // CRASH: invalid key
                let road = &self.roads[*road_key];
                let other_intersection = road.get_other_intersection(&cap);
                
                if explored_intersections.contains_key(&other_intersection) {
                    continue;
                }

                let value = vertex.value + road.calculate_dijkstra_value();

                if let Some(v) = unexplored_intersections.get_mut(&other_intersection) {
                    if v.value < value {
                        v.value = value;
                        continue;
                    }
                }

                unexplored_intersections.insert(other_intersection, DijkstraVertexData::new(Some((*key, *road_key)), value));
            }
        }

        None
    }
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