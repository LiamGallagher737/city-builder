use std::ops::Index;

use bevy::{utils::hashbrown::HashSet, prelude::{Vec3, Component, Entity}};

pub struct RoadNetwork {
    pub roads: Vec<Road>,
    pub intersections: Vec<Intersection>,
}

impl RoadNetwork {
    pub fn new() -> Self {
        Self {
            roads: Vec::new(),
            intersections: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct Road {
    pub nodes: Vec<Node>,
    pub intersection_start: u16,
    pub intersection_end: u16,
    pub mesh_entity: Entity,
}

impl Road {
    pub fn calculate_point_at_distance(self: &Self, distance: f32) -> Option<Vec3> {
        let node_index = distance.floor() as usize;

        if self.nodes.len() <= node_index + 1 {
            return None;
        }

        let t = distance - node_index as f32;

        let node_position = self.nodes[node_index].position;
        Some(node_position + (self.nodes[node_index + 1].position - node_position) * t)
    }
}

#[derive(Debug, Clone)]
pub struct Node {
    pub position: Vec3,
}

impl Node {
    // Creates a new node as chosen position and generates 2 control nodes based on rotation
    pub fn new(position: Vec3) -> Self {      
        Self {
            position: position,
        }
    }
}

pub struct Intersection {
    pub position: Vec3,
    pub roads: HashSet<(u16, RoadCap)>
}

#[derive(PartialEq, Eq, Hash)]
pub enum RoadCap {
    _Start,
    _End,
}

#[derive(Component)]
pub struct RoadCreator {
    pub active: bool,
    pub current_road_nodes: Option<Vec<Node>>,
    pub start_intersection: Option<u16>,
}

impl Default for RoadCreator {
    fn default() -> Self {
        Self {
            active: true, // Set this to false later
            current_road_nodes: None,
            start_intersection: None,
        }
    }
}

impl RoadCreator {
    pub fn last_node(self: &Self) -> Option<Node>{
        if let Some(nodes) = self.current_road_nodes.as_ref() {
            if let Some(last_node) = nodes.last() {
                return Some(last_node.clone());
            }
        }
        None
    }
}