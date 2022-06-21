use bevy::prelude::*;
use crate::game::roads::components::RoadNetwork;

use super::super::roads::components::RoadKey;

#[derive(Debug)]
pub enum BuildingData {
    Dwelling(Dwelling),
    Shop(Shop),
}

#[derive(Component, Debug)]
pub struct Building {
    pub data: BuildingData,
    pub address: Address,
}

#[derive(Debug)]
pub struct Address {
    pub road: RoadKey,
    pub t: f32,
}

impl RoadNetwork {
    pub fn get_address_position(self: &Self, address: &Address) -> Option<Vec3> {
        if let Some(road) = self.roads.get(address.road) {
            if let Some((position, _)) = road.calculate_point_at_distance(address.t) {
                return Some(position);
            }
        }
        
        None
    }
}

#[derive(Debug)]
pub struct Dwelling {

}

#[derive(Debug)]
pub struct Shop {

}