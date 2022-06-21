use bevy::prelude::*;
use super::super::roads::components::RoadKey;
use super::super::buildings::components::Address;

#[derive(Component)]
pub struct Vehicle {
    pub current_address: Address,
    pub direction: Direction,
    pub destination: Address,
    pub route: Vec<RoadKey>,
}

pub enum Direction {
    Forward,
    Backward,
}