// use bevy::prelude::*;
// use rand::prelude::*;
// use super::components::*;
// use super::super::roads::components::{RoadNetwork, RoadKey};

// pub fn spawn_building(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
//     road_network: Res<RoadNetwork>,
// ) {
//     let mut rng = thread_rng();

//     if let Some(address) = random_address(&mut rng, &road_network) {
//         commands.spawn_bundle(PbrBundle {
//             mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
//             material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
//             transform: Transform::from_translation(road_network.get_address_position(&address).unwrap()),
//             ..Default::default()
//         }).insert(Building {
//             data: BuildingData::Dwelling(Dwelling{}),
//             address,
//         });
//     }
// }

// fn random_address(rng: &mut ThreadRng, road_network: &Res<RoadNetwork>) -> Option<Address> {
//     let road_keys: Vec<RoadKey> = road_network.roads.keys().collect();

//     if road_keys.is_empty() {
//         return None;
//     }

//     let road = road_keys[rng.gen_range(0..road_keys.len())];
//     let t = rng.gen_range(0.0..road_network.roads[road].nodes.len() as f32);

//     Some(Address {
//         road,
//         t,
//     })
// }
