use bevy::{utils::hashbrown::HashSet, prelude::{Vec3, Quat, Component, Entity}};
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
    #[inline(always)]
    pub fn new(position: Vec3) -> Self {
        Self {
            position,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Intersection {
    pub position: Vec3,
    pub roads: HashSet<(RoadKey, RoadCap)>,
    pub mesh_entity: Option<Entity>,
}

impl Intersection {
    pub fn new(position: Vec3) -> Self {
        Self {
            position,
            roads: HashSet::new(),
            mesh_entity: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RoadCap {
    Start,
    End,
}

#[derive(Component)]
pub struct RoadCreator {
    pub active: bool,
    pub just_activated: bool,
    pub just_deactivated: bool,
    pub current_road_nodes: Vec<Node>,
    pub start_intersection: IntersectionKey,
    pub can_create_intersection: bool,
}

impl Default for RoadCreator {
    fn default() -> Self {
        Self {
            active: false,
            just_activated: false,
            just_deactivated: false,
            current_road_nodes: Vec::new(),
            start_intersection: IntersectionKey::default(),
            can_create_intersection: false,
        }
    }
}

impl RoadCreator {
    pub fn toggle_active(self: &mut Self) {
        if self.active {
            self.deactivate();
        } else {
            self.activate();
        }
    }
    pub fn activate(self: &mut Self) {
        self.active = true;
        self.just_activated = true;
    }
    pub fn deactivate(self: &mut Self) {
        self.active = false;
        self.just_deactivated = true;
    }
}