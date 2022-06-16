use bevy::core::FixedTimestep;
use bevy::prelude::*;

mod components;
mod building_spawner;

pub struct BuildingPlugin;
impl Plugin for BuildingPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(building_spawner::spawn_building);
    }
}