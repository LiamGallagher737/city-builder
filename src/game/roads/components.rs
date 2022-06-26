use bevy::{
    prelude::{Component, Entity, Quat, Vec3},
    utils::hashbrown::HashMap,
};
use slotmap::{new_key_type, SlotMap};

#[derive(Debug)]
pub struct RoadNetwork {
    pub roads: SlotMap<RoadKey, Road>,
    pub intersections: SlotMap<IntersectionKey, Intersection>,
}

new_key_type! {
    pub struct RoadKey;
    pub struct IntersectionKey;
}

#[derive(Debug, Clone)]
pub struct Road {
    pub nodes: Vec<Node>,
    pub intersection_start: IntersectionKey,
    pub intersection_end: IntersectionKey,
    pub speed: u8,
    pub mesh_entity: Option<Entity>,
}

impl Road {
    pub fn calculate_point_at_distance(&self, distance: f32) -> Option<(Vec3, Quat)> {
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

        Some((position, rotation))
    }
    #[inline(always)]
    pub fn get_other_intersection(&self, cap: &RoadCap) -> IntersectionKey {
        match cap {
            RoadCap::Start => self.intersection_end,
            RoadCap::End => self.intersection_start,
        }
    }
    #[inline(always)]
    pub fn has_intersection(&self, intersection: &IntersectionKey) -> bool {
        if *intersection == self.intersection_start || *intersection == self.intersection_end {
            return true;
        }
        false
    }
}

#[derive(Debug, Clone)]
pub struct Node {
    pub position: Vec3,
}

impl Node {
    #[inline(always)]
    pub fn new(position: Vec3) -> Self {
        Self { position }
    }
}

#[derive(Debug, Clone)]
pub struct Intersection {
    pub position: Vec3,
    pub connections: HashMap<RoadKey, RoadCap>,
    pub mesh_entity: Option<Entity>,
}

impl Intersection {
    pub fn new(position: Vec3) -> Self {
        Self {
            position,
            connections: HashMap::new(),
            mesh_entity: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RoadCap {
    Start,
    End,
}

#[derive(Component, Default)]
pub struct RoadCreator {
    pub active: bool,
    pub just_activated: bool,
    pub just_deactivated: bool,
    pub current_road_nodes: Vec<Node>,
    pub start_intersection: IntersectionKey,
    pub can_create_intersection: bool,
}

impl RoadCreator {
    pub fn toggle_active(&mut self) {
        if self.active {
            self.deactivate();
        } else {
            self.activate();
        }
    }
    pub fn activate(&mut self) {
        self.active = true;
        self.just_activated = true;
    }
    pub fn deactivate(&mut self) {
        self.active = false;
        self.just_deactivated = true;
    }
}
