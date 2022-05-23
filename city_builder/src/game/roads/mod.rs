use bevy::prelude::*;

mod road_network;

pub struct RoadsPlugin;
impl Plugin for RoadsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(road_network::road_network_startup_system);
    }
}