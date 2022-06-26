// use bevy::core::FixedTimestep;
use bevy::prelude::*;

mod building_spawner;
pub mod components;

pub struct BuildingPlugin;
impl Plugin for BuildingPlugin {
    fn build(&self, _app: &mut App) {
        // app.add_system_set(
        //     SystemSet::new()
        //         .with_run_criteria(FixedTimestep::step(1.0))
        //         .with_system(building_spawner::spawn_building)
        // );
    }
}
