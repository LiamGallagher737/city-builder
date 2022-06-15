use bevy::prelude::*;
use super::super::roads::components::RoadKey;

#[derive(Component)]
pub struct Building {
    pub road: RoadKey,
    pub t: f32,
}