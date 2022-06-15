use bevy::prelude::{App, Assets, Color, Commands, Mesh, PbrBundle, Plugin, ResMut, shape, StandardMaterial, Transform};
use crate::game::roads::components::RoadKey;
use crate::game::vehicles::components::{Direction, Vehicle};

mod components;
mod vehicle_driving;

pub struct VehiclesPlugin;
impl Plugin for VehiclesPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_vehicles);
        app.add_system(vehicle_driving::vehicle_drive_system);
    }
}

fn spawn_vehicles (
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..Default::default()
    })
    .insert(Vehicle {
        road: RoadKey::default(),
        t: 0.0,
        direction: Direction::Forward,
    });
}