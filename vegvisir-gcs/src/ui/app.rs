use egui_tracing::EventCollector;
use tracing_core::Level;
use tracing_subscriber::filter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use crate::ui;
use crate::ui::dialogs::DialogKind;
use crate::ui::{dialogs, widgets};
use crate::ui::dialogs::connect_vehicle::ConnectVehicleState;
use crate::ui::widgets::status::StatusDot;

/// Structure handling application state
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct App {
    /// [`egui_tracing`] event collector
    #[serde(skip)]
    event_collector: EventCollector,

    #[serde(skip)]
    dialog: Option<DialogKind>
}

impl Default for App {
    fn default() -> Self {
        let event_collector = EventCollector::default();
        let filter = filter::Targets::new()
            .with_target("vegvisir_gcs", Level::INFO);

        tracing_subscriber::registry()
            .with(filter)
            .with(event_collector.clone())
            .init();

        Self {
            event_collector,
            dialog: None
        }
    }
}

impl App {
    /// Create an instance of [`App`]
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        }
    }
}

impl eframe::App for App {
    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let should_close = if let Some(dialog) = self.dialog.as_mut() {
            match dialog {
                DialogKind::ConnectVehicle { state } => {
                    dialogs::connect_vehicle::show(ctx, state)
                }
            }
        } else {
            false
        };

        if should_close {
            self.dialog = None;
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::Panel::top("top_panel").show_inside(ui, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                if ui.button("Connect vehicle").clicked() {
                    self.dialog = Some(DialogKind::ConnectVehicle { state: ConnectVehicleState::default() });
                }
            });
        });

        egui::Panel::right("right_panel").show_inside(ui, |ui| {
            ui.add(egui_tracing::Logs::new(self.event_collector.clone()));
        });

        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.add(StatusDot::new(ui::widgets::status::RED));
                    ui.label("Not connected");
                });
                ui.separator();
            });
        });
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}
