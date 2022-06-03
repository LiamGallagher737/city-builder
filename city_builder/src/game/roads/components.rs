use std::{sync::{RwLock, Arc}, f32::consts::PI};
use bevy::{utils::hashbrown::HashSet, prelude::{Vec3, Quat, Component, Entity}};

pub struct RoadNetwork {
    pub roads: HashSet<Arc<RwLock<Road>>>,
}

impl RoadNetwork {
    pub fn new() -> Self {
        Self {
            roads: HashSet::<Arc<RwLock<Road>>>::new(),
        }
    }
}

pub struct Road {
    pub nodes: Vec<Node>,
    pub intersection_start: Option<Arc<RwLock<Intersection>>>,
    pub intersection_end: Option<Arc<RwLock<Intersection>>>,
    pub entity: Entity,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub position: Vec3,
    pub control_a: Vec3,
    pub control_b: Vec3,
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
        }
    }
}

pub struct Intersection {
    pub position: Vec3,
    pub roads: HashSet<(Arc<RwLock<Road>>, RoadCap)>
}

#[derive(PartialEq, Eq, Hash)]
pub enum RoadCap {
    _Start,
    _End,
}

#[derive(Component)]
pub struct RoadCreator {
    pub current_road_nodes: Option<Vec<Node>>,
    pub start_intersection: Option<Arc<RwLock<Intersection>>>,
}

impl Default for RoadCreator {
    fn default() -> Self {
        Self {
            current_road_nodes: None,
            start_intersection: None,
        }
    }
}