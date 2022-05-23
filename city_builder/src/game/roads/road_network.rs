use bevy::{prelude::*, utils::hashbrown::HashMap};

pub struct RoadNetwork {
    // roads: HashMap< >
    num: i8
}

impl RoadNetwork {
    fn new() -> Self {
        Self {num:1}
    }
}

pub fn road_network_startup_system(mut commands: Commands) {
    commands.insert_resource(RoadNetwork::new());
}