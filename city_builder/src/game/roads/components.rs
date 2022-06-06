use std::f32::consts::PI;
use bevy::{utils::hashbrown::HashSet, prelude::{Vec3, Quat, Component, Entity}};

use crate::lib::bezier::evaluate_cubic_3d;

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
    pub length: f32,
    pub intersection_start: u16,
    pub intersection_end: u16,
    pub mesh_entity: Entity,
}

impl Road {
    pub fn calculate_distances_and_length(self: &mut Self) {
        const RESULTION: u8 = 20;
        let mut total_length = 0.0;

        for n in 0..self.nodes.len() - 1 {
            let node = &self.nodes[n];
            let next_node = &self.nodes[n + 1];

            let mut length = 0.0;
            let mut previous_point = node.position;
            for i in 1..=RESULTION {
                let t = 1.0 / RESULTION as f32 * i as f32;
                let point = evaluate_cubic_3d(node.position, node.control_b, next_node.control_a, next_node.position, t);
                length += previous_point.distance(point);
                previous_point = point;
            }

            self.nodes[n].next_node_distance = length;
            total_length += length;
        }

        self.length = total_length;
    }
}

#[derive(Debug, Clone)]
pub struct Node {
    pub position: Vec3,
    pub control_a: Vec3,
    pub control_b: Vec3,
    pub next_node_distance: f32,
}

impl Node {
    // Creates a new node as chosen position and generates 2 control nodes based on rotation
    pub fn new(position: Vec3, rotation: Quat) -> Self {
        let sinp = 2.0 * (rotation.w * rotation.y - rotation.z * rotation.x);
        let euler_y: f32;
        if sinp.abs() >= 1.0 {
            euler_y = PI / 2.0 * sinp.sin();
        } else {
            euler_y = sinp.asin();
        }
        let forward = Vec3::new(euler_y.cos(), 0.0, euler_y.sin());
        
        Self {
            position: position,
            control_a: position + forward * 1.0,
            control_b: position - forward * 1.0,
            next_node_distance: f32::NAN,
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
            active: true,
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