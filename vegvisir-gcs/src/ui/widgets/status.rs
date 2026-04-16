use egui::{self, Color32, Response, Sense, Ui, Vec2, Widget};

pub const GREEN: Color32 = Color32::from_rgb(130, 184, 114);
pub const RED: Color32 = Color32::from_rgb(184, 114, 114);
pub const YELLOW: Color32 = Color32::from_rgb(184, 179, 114);

pub struct StatusDot {
    color: Color32,
    size: f32,
}

impl StatusDot {
    pub fn new(color: Color32) -> Self {
        Self {
            color,
            size: 8.0,
        }
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }
}

impl Widget for StatusDot {
    fn ui(self, ui: &mut Ui) -> Response {
        let desired_size = Vec2::splat(self.size);
        let (rect, response) = ui.allocate_exact_size(desired_size, Sense::hover());

        if ui.is_rect_visible(rect) {
            let painter = ui.painter();
            let center = rect.center();
            let radius = self.size * 0.5;

            painter.circle_filled(center, radius, self.color);
        }

        response
    }
}