use bevy::prelude::*;

mod roads;

pub struct GamePlugins;

impl PluginGroup for GamePlugins {
    fn build(&mut self, group: &mut bevy::app::PluginGroupBuilder) {
        group.add(roads::RoadsPlugin);
    }
}