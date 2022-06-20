use bevy::core::FixedTimestep;
use bevy::prelude::*;

mod buildings;
mod components;
mod building_spawner;

pub struct BuildingPlugin;
impl Plugin for BuildingPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.0))
                .with_system(building_spawner::spawn_building)
        );
    }
}