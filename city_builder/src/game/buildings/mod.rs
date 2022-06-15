use bevy::core::FixedTimestep;
use bevy::prelude::*;

mod components;
mod building_spawner;

pub struct BuildingPlugin;
impl Plugin for BuildingPlugin {
    fn build(&self, app: &mut App) {
        // app.add_system_to_stage(
        //     SystemSet::new().with_run_criteria(FixedTimestep::step(5.0)),
        //     building_spawner::spawn_building
        // );
    }
}