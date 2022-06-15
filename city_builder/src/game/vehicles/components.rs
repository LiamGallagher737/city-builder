use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use super::super::roads::components::RoadKey;

#[derive(Component)]
pub struct Vehicle {
    pub road: RoadKey,
    pub t: f32,
    pub direction: Direction,
}

#[derive(Inspectable)]
pub enum Direction {
    Forward,
    Backward,
}