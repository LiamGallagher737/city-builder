use bevy::prelude::*;

mod roads;
mod vehicles;
mod buildings;

pub struct GamePlugins;

impl PluginGroup for GamePlugins {
    fn build(&mut self, group: &mut bevy::app::PluginGroupBuilder) {
        group.add(roads::RoadsPlugin);
        group.add(vehicles::VehiclesPlugin);
    }
}