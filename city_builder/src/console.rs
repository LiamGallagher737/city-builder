use bevy_console::ConsoleCommand;

/// Spawns a vehicle at chosen point - road: usize, t: f64
#[derive(ConsoleCommand)]
#[console_command(name = "spawn_vehicle")]
pub struct SpawnVehicleCommand {
    road: usize,
    t: f64,
}

pub fn spawn_vehicle_command(mut log: ConsoleCommand<SpawnVehicleCommand>) {
    if let Some(SpawnVehicleCommand { road, t }) = log.take() {
        println!("Road: {0}, t: {1}", road, t);
    }
}
