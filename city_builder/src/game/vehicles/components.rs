use bevy::prelude::*;
use super::super::roads::road_pathfinding::RoutePoint;
use super::super::buildings::components::Address;

#[derive(Component)]
pub struct Vehicle {
    pub current_address: Address,
    pub direction: Direction,
    pub destination: Address,
    pub route: Vec<RoutePoint>,
    pub point: Option<RoutePoint>,
}

pub enum Direction {
    Forward,
    Backward,
}