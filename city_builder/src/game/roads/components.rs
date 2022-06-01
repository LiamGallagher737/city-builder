use std::sync::{RwLock, Arc};
use bevy::{utils::hashbrown::HashSet, prelude::{Vec3, Component}};

pub struct RoadNetwork {
    pub roads: HashSet<Road>,
}

impl RoadNetwork {
    pub fn new() -> Self {
        Self {
            roads: HashSet::<Road>::new(),
        }
    }
}

pub struct Road {
    pub nodes: Vec<Node>,
    pub intersection_start: Option<Arc<RwLock<Intersection>>>,
    pub intersection_end: Option<Arc<RwLock<Intersection>>>,
}

impl Road {
    pub fn new() -> Self {
        Self { 
            nodes: Vec::<Node>::new(), 
            intersection_start: None, 
            intersection_end: None,
        }
    }
}

pub struct Node {
    pub position: Vec3,
    pub control_a: Vec3,
    pub control_b: Vec3,
}

pub struct Intersection {
    pub position: Vec3,
    pub roads: HashSet<(Arc<RwLock<Road>>, RoadCap)>
}

impl Intersection {
    pub fn new() -> Self {
        Self {
            position: Vec3::default(),
            roads: HashSet::<(Arc<RwLock<Road>>, RoadCap)>::new(),
        }
    }
}

pub enum RoadCap {
    Start,
    End,
}

#[derive(Component)]
pub struct RoadCreator {

}