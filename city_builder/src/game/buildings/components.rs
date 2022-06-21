use bevy::prelude::*;
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

#[derive(Debug)]
pub struct Dwelling {

}

#[derive(Debug)]
pub struct Shop {

}