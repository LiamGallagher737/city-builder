use bevy::prelude::*;

use self::road_creation::road_creation_system;

mod road_network;
mod road_creation;
mod road_mesh;
mod components;

pub struct RoadsPlugin;
impl Plugin for RoadsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(road_network::road_network_startup_system);

        app.add_system_to_stage(
            CoreStage::PreUpdate,
            road_creation_system
        );
    }
}