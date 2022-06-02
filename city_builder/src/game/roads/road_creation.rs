use std::sync::{Arc, RwLock};

use bevy::{prelude::*, utils::hashbrown::HashSet};

use super::components::*;

pub fn road_creation_system(
    mut road_network: ResMut<RoadNetwork>,
    mut query: Query<(&mut RoadCreator, &Transform)>
) {
    if let Ok((mut road_creator, tf)) = query.get_single_mut() {
        if road_creator.current_road.is_none() {
            road_creator.current_road = Some(Arc::new(RwLock::new(Road {
                nodes: vec![],
                intersection_start: Some(Arc::new(RwLock::new(Intersection {
                    position: tf.translation,
                    roads: HashSet::<(Arc<RwLock<Road>>, RoadCap)>::new()
                }))),
                intersection_end: None,
            })));
        }
    }
}