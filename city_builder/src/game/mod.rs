use bevy::prelude::*;

pub mod buildings;
pub mod roads;
pub mod vehicles;

pub struct GamePlugins;

impl PluginGroup for GamePlugins {
    fn build(&mut self, group: &mut bevy::app::PluginGroupBuilder) {
        group.add(roads::RoadsPlugin);
        group.add(vehicles::VehiclesPlugin);
        group.add(buildings::BuildingPlugin);
    }
}
