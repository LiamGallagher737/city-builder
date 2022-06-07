use bevy::{utils::hashbrown::HashSet, prelude::{ResMut, Query, Transform, Commands, Assets, Mesh, Color, With, Camera, Without}, pbr::{PbrBundle, StandardMaterial}, math::Vec3};
use super::{components::*, road_mesh};

const NEW_NODE_DISTANCE_SQ: f32 = 1.0 * 1.0;
static mut counter: f32 = 0.0;

pub fn road_creation_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut road_network: ResMut<RoadNetwork>,
    keys: bevy::prelude::Res<bevy::input::Input<bevy::prelude::KeyCode>>,
    mut query: Query<(&mut RoadCreator, &mut Transform)>,
    mut cam_query: Query<&mut Transform, (With<Camera>, Without<RoadCreator>)>
) {
    if let Ok((mut road_creator, mut tf)) = query.get_single_mut() {

        if let Some(road) = road_network.roads.last() {

            

            unsafe {
                counter += 0.05;
            }
        }

        let mut velocity = Vec3::default();

        if keys.pressed(bevy::prelude::KeyCode::W) {
            velocity += tf.forward() * 0.05;
        }

        if keys.pressed(bevy::prelude::KeyCode::S) {
            velocity -= tf.forward() * 0.05;
        }

        if keys.pressed(bevy::prelude::KeyCode::A) {
            velocity -= tf.right() * 0.05;
        }

        if keys.pressed(bevy::prelude::KeyCode::D) {
            velocity += tf.right() * 0.05;
        }

        if keys.pressed(bevy::prelude::KeyCode::Left) {
            tf.rotate(bevy::math::Quat::from_euler(bevy::math::EulerRot::XYZ, 0.0, 0.05, 0.0));
        }

        if keys.pressed(bevy::prelude::KeyCode::Right) {
            tf.rotate(bevy::math::Quat::from_euler(bevy::math::EulerRot::XYZ, 0.0, -0.05, 0.0));
        }

        tf.translation += velocity;

        if let Ok(mut cam_tf) = cam_query.get_single_mut() {
            cam_tf.look_at(tf.translation, Vec3::Y);
        }

        // Start of actual code

        if road_creator.current_road_nodes.is_none() {
            road_creator.current_road_nodes = Some(vec![Node::new(tf.translation, tf.rotation)]);
        }

        if road_creator.start_intersection.is_none() {
            road_network.intersections.push(Intersection {
                position: tf.translation,
                roads: HashSet::new(),
            });
            road_creator.start_intersection = Some((road_network.intersections.len() - 1) as u16);
        }

        if let Some(last_node) = road_creator.last_node() {
            if last_node.position.distance_squared(tf.translation) >= NEW_NODE_DISTANCE_SQ {
                road_creator.current_road_nodes.as_mut().unwrap().push(Node::new(tf.translation, tf.rotation));
            }
        }

        if !keys.just_pressed(bevy::prelude::KeyCode::Space) {
            return;
        }

        road_network.intersections.push(Intersection {
            position: tf.translation,
            roads: HashSet::new(),
        });

        let mut road = Road {
            nodes: road_creator.current_road_nodes.clone().unwrap(),
            length: f32::NAN,
            intersection_start: road_creator.start_intersection.unwrap(),
            intersection_end: (road_network.intersections.len() - 1) as u16,
            mesh_entity: commands.spawn_bundle(PbrBundle {
                mesh: meshes.add(road_mesh::generate_road_mesh(&road_creator.current_road_nodes.as_ref().unwrap())),
                material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
                ..Default::default()
            }).id()
        };
        road.calculate_distances_and_length(); 

        road_network.roads.push(road);

        road_creator.current_road_nodes = None;
        road_creator.start_intersection = Some((road_network.intersections.len() - 1) as u16);

    }
}
