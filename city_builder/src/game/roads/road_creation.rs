use std::sync::{Arc, RwLock};

use bevy::{utils::hashbrown::HashSet, prelude::{ResMut, Query, Transform, Vec3}};

use super::components::*;

const NEW_NODE_DISTANCE_SQ: f32 = 10.0 * 10.0;
const JOIN_INTERSECTION_DISTANCE_SQ: f32 = 2.0 * 2.0;

pub fn road_creation_system(
    mut _road_network: ResMut<RoadNetwork>,
    mut query: Query<(&mut RoadCreator, &mut Transform)>
) {
    if let Ok((mut road_creator, mut tf)) = query.get_single_mut() {

        tf.translation.x += 0.05;

        if road_creator.current_road_nodes.is_none() {

            road_creator.start_intersection = Some(Arc::new(RwLock::new(Intersection {
                position: tf.translation,
                roads: HashSet::<(Arc<RwLock<Road>>, RoadCap)>::new(),
            })));

            road_creator.current_road_nodes = Some(vec![Node::new(tf.translation, tf.rotation)]);

            return;
        }

        if road_creator.current_road_nodes.as_ref().unwrap().last().unwrap().position.distance_squared(tf.translation) >= NEW_NODE_DISTANCE_SQ {
            road_creator.current_road_nodes.as_mut().unwrap().push(Node::new(tf.translation, tf.rotation));

            // println!("{:#?} \n\n\n", road_creator.current_road_nodes);
        }

        // TODO: check for intersection
    }
}