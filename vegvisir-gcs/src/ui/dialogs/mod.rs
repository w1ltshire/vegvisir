use crate::ui::dialogs::connect_vehicle::ConnectVehicleState;

pub mod connect_vehicle;

pub enum DialogKind {
    ConnectVehicle { state: ConnectVehicleState },
}
