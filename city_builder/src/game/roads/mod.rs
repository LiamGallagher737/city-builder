use bevy::prelude::*;

mod road_network;
mod road_creation;
mod road_mesh;
pub mod components;
pub mod road_pathfinding;

pub struct RoadsPlugin;
impl Plugin for RoadsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(road_network::road_network_startup_system);

        app.add_system_to_stage(
            CoreStage::PreUpdate,
            road_creation::road_creation_system
        );
    }
}