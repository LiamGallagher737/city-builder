use bevy::{utils::hashbrown::HashSet, prelude::{Vec3, Quat, Component, Entity}};

#[derive(Debug)]
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

#[derive(Debug, Clone)]
pub struct Road {
    pub nodes: Vec<Node>,
    pub intersection_start: usize,
    pub intersection_end: usize,
    pub mesh_entity: Entity,
}

impl Road {
    pub fn _calculate_point_at_distance(self: &Self, distance: f32) -> Option<(Vec3, Quat)> {
        let node_index = distance.floor() as usize;

        if self.nodes.len() <= node_index + 1 {
            return None;
        }

        let node_position = self.nodes[node_index].position;
        let other_node_position = self.nodes[node_index + 1].position;

        let t = distance - node_index as f32;
        let position = node_position + (other_node_position - node_position) * t;

        // Stolen from Transform::look_at method
        let forward = node_position - other_node_position;
        let right = Vec3::Y.cross(forward).normalize();
        let up = forward.cross(right);
        let rotation = Quat::from_mat3(&bevy::math::Mat3::from_cols(right, up, forward));

        Some((
            position,
            rotation,
        ))
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

#[derive(Debug)]
pub struct Intersection {
    pub position: Vec3,
    pub roads: HashSet<(usize, RoadCap)>
}

impl Intersection {
    pub fn new(position: Vec3) -> Self {
        Self {
            position,
            roads: HashSet::new(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum RoadCap {
    Start,
    End,
}

#[derive(Component)]
pub struct RoadCreator {
    pub active: bool,
    pub current_road_nodes: Vec<Node>,
    pub start_intersection: usize,
    pub can_create_intersection: bool,
}

impl Default for RoadCreator {
    fn default() -> Self {
        Self {
            active: true, // Set this to false later
            current_road_nodes: Vec::new(),
            start_intersection: 0,
            can_create_intersection: false,
        }
    }
}