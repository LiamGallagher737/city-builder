use super::super::buildings::components::Address;
use super::super::roads::components::IntersectionKey;
use bevy::prelude::*;

#[derive(Component)]
pub struct Vehicle {
    pub current_address: Address,
    pub direction: Direction,
    pub destination: Address,
    pub route: Vec<IntersectionKey>,
}

pub enum Direction {
    Forward,
    // Backward,
}
