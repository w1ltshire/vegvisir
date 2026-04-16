use crate::ui::widgets;

#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct ConnectVehicleState {
    connection_type: ConnectionType,
    ip: IpConnectionState,
    serial: SerialConnectionState,
}

#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct IpConnectionState {
    host: String,
    port: u16,
    protocol: Protocol,
    server_or_client: ServerOrClient
}

#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct SerialConnectionState {
    port_name: String,
    baud_rate: u32,
}

#[derive(Default, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
enum ConnectionType {
    #[default]
    IPConnection,
    SerialConnection,
}

#[derive(Default, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
enum Protocol {
    #[default]
    TCP,
    UDP,
}

#[derive(Default, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
enum ServerOrClient {
    #[default]
    Server,
    Client,
}

impl ConnectionType {
    fn name(&self) -> &'static str {
        match self {
            Self::IPConnection => "IP",
            Self::SerialConnection => "Serial",
        }
    }
}

pub fn show(ctx: &egui::Context, state: &mut ConnectVehicleState) -> bool {
    let (_, out) = widgets::modal::modal(ctx, "connect_vehicle_dialog", "Connect vehicle", |ui| {
        ui.horizontal(|ui| {
            ui.selectable_value(
                &mut state.connection_type,
                ConnectionType::IPConnection,
                "IP",
            );
            ui.selectable_value(
                &mut state.connection_type,
                ConnectionType::SerialConnection,
                "Serial",
            );
        });

        ui.separator();
        ui.add_space(8.0);

        match state.connection_type {
            ConnectionType::IPConnection => {
                ui.horizontal(|ui| {
                    ui.label("Protocol");
                    ui.radio_value(&mut state.ip.protocol, Protocol::TCP, "TCP");
                    ui.radio_value(&mut state.ip.protocol, Protocol::UDP, "UDP");
                });

                ui.add_space(2.0);

                ui.horizontal(|ui| {
                    ui.label("Act as");
                    ui.radio_value(&mut state.ip.server_or_client, ServerOrClient::Server, "Server");
                    ui.radio_value(&mut state.ip.server_or_client, ServerOrClient::Client, "Client");
                });

                ui.add_space(2.0);

                ui.horizontal(|ui| {
                    ui.label("Host");
                    ui.text_edit_singleline(&mut state.ip.host);
                });

                ui.add_space(2.0);

                ui.horizontal(|ui| {
                    ui.label("Port");
                    ui.add(egui::DragValue::new(&mut state.ip.port).range(0..=65535));
                });
            }

            ConnectionType::SerialConnection => {
                ui.horizontal(|ui| {
                    ui.label("Port");
                    ui.text_edit_singleline(&mut state.serial.port_name);
                });

                ui.add_space(2.0);

                ui.horizontal(|ui| {
                    ui.label("Baudrate");
                    ui.add(egui::DragValue::new(&mut state.serial.baud_rate));
                });
            }
        }
    });

    out.close
}