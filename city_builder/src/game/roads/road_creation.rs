use bevy::prelude::*;

use super::components::*;

pub fn road_creation_system(
    mut road_network: ResMut<RoadNetwork>,
    mut query: Query<(&mut RoadCreator, &Transform)>
) {
    
}