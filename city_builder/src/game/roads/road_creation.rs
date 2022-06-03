use std::sync::{Arc, RwLock};
use bevy::{utils::hashbrown::HashSet, prelude::{ResMut, Query, Transform, Commands, Assets, Mesh, Color}, pbr::{PbrBundle, StandardMaterial}};
use super::{components::*, road_mesh};

const NEW_NODE_DISTANCE_SQ: f32 = 10.0 * 10.0;

pub fn road_creation_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut road_network: ResMut<RoadNetwork>,
    keys: bevy::prelude::Res<bevy::input::Input<bevy::prelude::KeyCode>>,
    mut query: Query<(&mut RoadCreator, &mut Transform)>,
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

            println!("\n\n\n{:#?}\n\n\n", road_creator.current_road_nodes);
        }

        // TODO: check for intersection
        if !keys.just_pressed(bevy::prelude::KeyCode::Space) {
            return;
        }

        let intersection = Arc::new(RwLock::new(Intersection {
            position: tf.translation,
            roads: HashSet::new(),
        }));

        let road = Arc::new(RwLock::new(Road {
            nodes: road_creator.current_road_nodes.unwrap(),
            intersection_start: road_creator.start_intersection.clone(),
            intersection_end: Some(intersection.clone()),
            entity: commands.spawn_bundle(PbrBundle {
                mesh: meshes.add(road_mesh::generate_road_mesh(&road_creator.current_road_nodes.unwrap())),
                material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
                ..Default::default()
            }).id()
        }));

        *intersection.write().unwrap().roads.insert(road);
    }
}