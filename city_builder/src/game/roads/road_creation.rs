use bevy::{utils::hashbrown::HashSet, prelude::{ResMut, Query, Transform, Commands, Assets, Mesh, Color, With, Camera, Without}, pbr::{PbrBundle, StandardMaterial}, math::Vec3};
use crate::game::roads::road_network::INTERSECTION_RADIUS_SQ;
use super::{components::*, road_mesh::generate_road_mesh, road_network::{ROAD_NODE_DISTANCE, ROAD_NODE_DISTANCE_SQ}, road_creation::ValidIntersection::{NotFound, OnIntersection, OnRoad}};

const NEW_INTERSECTION_DISTANCE_SQ: f32 = 3.5 * 3.5;
// static mut DISTANCE_TRAVELED: f32 = 0.0;

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

        {
            // if let Some(road) = road_network.roads.last() {
            //     unsafe {
            //         if let Some((position, rotation)) = road.calculate_point_at_distance(DISTANCE_TRAVELED) {
            //             tf.translation = position;
            //             tf.rotation = rotation;
            //         } else {
            //             DISTANCE_TRAVELED = 0.0;
            //         }
            //         DISTANCE_TRAVELED += 0.05;
            //     }
            // }

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
                tf.rotate(bevy::math::Quat::from_euler(bevy::math::EulerRot::XYZ, 0.0, 0.01, 0.0));
            }

            if keys.pressed(bevy::prelude::KeyCode::Right) {
                tf.rotate(bevy::math::Quat::from_euler(bevy::math::EulerRot::XYZ, 0.0, -0.01, 0.0));
            }

            tf.translation += velocity;

            if let Ok(mut cam_tf) = cam_query.get_single_mut() {
                cam_tf.look_at(tf.translation, Vec3::Y);
            }
        }

        if keys.just_pressed(bevy::prelude::KeyCode::Space) {
            road_creator.toggle_active();
        }

        if road_creator.just_activated {

            road_creator.just_activated = false;

            // Initialize new intersection
            road_creator.start_intersection = road_network.intersections.insert(
                Intersection::new(tf.translation),
            );

        }

        if road_creator.just_deactivated {

            road_creator.just_deactivated = false;

            // New intersection
            let new_intersection_key = road_network.intersections.insert(
                Intersection {
                    position: tf.translation,
                    roads: HashSet::new(),
                }
            );

            // New road
            let new_road_key = road_network.roads.insert(
                Road {
                    nodes: road_creator.current_road_nodes.clone(),
                    intersection_start: road_creator.start_intersection,
                    intersection_end: new_intersection_key,
                    mesh_entity: commands.spawn_bundle(PbrBundle {
                        mesh: meshes.add(generate_road_mesh(&road_creator.current_road_nodes)),
                        material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
                        ..Default::default()
                    }).id()
                }
            );

            // Give intersections reference to new road
            road_network.intersections[new_intersection_key].roads.insert((new_road_key, RoadCap::End));
            road_network.intersections[road_creator.start_intersection].roads.insert((new_road_key, RoadCap::Start));

            // Reset road creator component
            road_creator.current_road_nodes.clear();
            road_creator.start_intersection = IntersectionKey::default();
            road_creator.can_create_intersection = false;

        }

        if !road_creator.active {
            return;
        }

        // Add new node if over certain distance from last node or start intersection
        if let Some(last_node) = road_creator.current_road_nodes.last() {
            if last_node.position.distance_squared(tf.translation) >= ROAD_NODE_DISTANCE_SQ {
                road_creator.current_road_nodes.push(Node::new(tf.translation));
            }
        } else if let Some(intersection) = road_network.intersections.get(road_creator.start_intersection) {
            if intersection.position.distance_squared(tf.translation) >= ROAD_NODE_DISTANCE_SQ {
                road_creator.current_road_nodes.push(Node::new(tf.translation));
            }
        }

        // Check for a valid intersection
        let check_intersection_result = check_for_valid_intersection(
            tf.translation,
            &road_network,
        );

        if !road_creator.can_create_intersection {
            match check_intersection_result {
                OnIntersection(_) | OnRoad(_, _) => return,
                NotFound => {
                    road_creator.can_create_intersection = true;
                    return;
                }
            };
        }

        if let OnIntersection(intersection_key) = check_intersection_result {

            let new_road_key = road_network.roads.insert(
                Road {
                    nodes: road_creator.current_road_nodes.clone(),
                    intersection_start: road_creator.start_intersection,
                    intersection_end: intersection_key,
                    mesh_entity: commands.spawn_bundle(PbrBundle {
                        mesh: meshes.add(generate_road_mesh(&road_creator.current_road_nodes)),
                        material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
                        ..Default::default()
                    }).id()
                }
            );

            road_network.intersections[road_creator.start_intersection].roads.insert((new_road_key, RoadCap::Start));
            road_network.intersections[intersection_key].roads.insert((new_road_key, RoadCap::End));

            road_creator.current_road_nodes.clear();
            road_creator.start_intersection = intersection_key;
            road_creator.can_create_intersection = false;

        }

        else if let OnRoad(road_key, node_index) = check_intersection_result {

            let road_data = road_network.roads[road_key].clone();

            // Create a new intersection at the point
            let new_intersection_key = road_network.intersections.insert(
                Intersection {
                    position: road_data.nodes[node_index].position,
                    roads: HashSet::new(),
                }
            );

            // Create first half road
            let nodes = road_data.nodes[..node_index].to_vec();
            let road_a_key = road_network.roads.insert(
                Road {
                    nodes: nodes.clone(),
                    intersection_start: road_data.intersection_start,
                    intersection_end: new_intersection_key,
                    mesh_entity: commands.spawn_bundle(PbrBundle {
                        mesh: meshes.add(generate_road_mesh(&nodes)),
                        material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
                        ..Default::default()
                    }).id()
                }
            );

            // Create second half road
            let nodes = road_data.nodes[node_index..].to_vec();
            let road_b_key = road_network.roads.insert(
                Road {
                    nodes: nodes.clone(),
                    intersection_start: road_data.intersection_start,
                    intersection_end: new_intersection_key,
                    mesh_entity: commands.spawn_bundle(PbrBundle {
                        mesh: meshes.add(generate_road_mesh(&nodes)),
                        material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
                        ..Default::default()
                    }).id()
                }
            );

            // Create current road
            let current_road_key = road_network.roads.insert(
                Road {
                    nodes: road_creator.current_road_nodes.clone(),
                    intersection_start: road_data.intersection_start,
                    intersection_end: new_intersection_key,
                    mesh_entity: commands.spawn_bundle(PbrBundle {
                        mesh: meshes.add(generate_road_mesh(&road_creator.current_road_nodes)),
                        material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
                        ..Default::default()
                    }).id()
                }
            );

            // Give intersection references to new roads
            road_network.intersections[new_intersection_key].roads.insert((road_a_key, RoadCap::End));
            road_network.intersections[new_intersection_key].roads.insert((road_b_key, RoadCap::Start));
            road_network.intersections[new_intersection_key].roads.insert((current_road_key, RoadCap::End));

            // Remove nodes to close to the new intersection
            road_network.intersections[new_intersection_key].clone().clear_nearby_nodes(&mut road_network);

            // Clear road creator component
            road_creator.current_road_nodes.clear();
            road_creator.start_intersection = new_intersection_key;
            road_creator.can_create_intersection = false;

            // Remove old road
            commands.entity(road_data.mesh_entity).despawn();
            road_network.roads.remove(road_key);

        }

    }
}

fn check_for_valid_intersection(
    position: Vec3,
    road_network: &RoadNetwork,
) -> ValidIntersection {

    for intersection in road_network.intersections.keys() {
        if road_network.intersections[intersection].position.distance_squared(position) <= INTERSECTION_RADIUS_SQ {
            return OnIntersection(intersection);
        }
    }

    for r in road_network.roads.keys() {
        let mut lowest_distance = f32::INFINITY;
        let mut lowest_pair = None;

        let road = &road_network.roads[r];

        let mut n = 0_usize;
        while n < road.nodes.len() {

            let distance_sq = road.nodes[n].position.distance_squared(position);

            // If node is far away then there is not point checking its
            // neighbours so we skip them based on how large the distance is
            if distance_sq > (10.0 * 10.0) * ROAD_NODE_DISTANCE {
                // FREEZE: be aware may cause crash due to subtract below 0
                n += (distance_sq / ROAD_NODE_DISTANCE).sqrt().floor() as usize - 5;
                continue;
            }

            // Since the distance checks overlap we try get the lowest by
            // checking even if we have one within range and only stopping
            // once the distance starts increasing again
            if lowest_distance < distance_sq {
                break;
            }

            // If within range then set lowest variables to it
            if distance_sq <= NEW_INTERSECTION_DISTANCE_SQ {
                lowest_distance = distance_sq;
                lowest_pair = Some((r, n));
            }

            n += 1;
        }

        if let Some(pair) = lowest_pair {
            return OnRoad(pair.0, pair.1);
        }
    }

    NotFound
}

enum ValidIntersection {
    OnIntersection(IntersectionKey),
    OnRoad(RoadKey, usize),
    NotFound,
}

mod tests {
    #![allow(unused_imports)]
    use super::*;

    // #[test]
    // fn test() {
    //
    // }
}