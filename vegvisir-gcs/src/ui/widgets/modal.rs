use egui::{self, Id, Ui};

pub struct DialogOutput {
    pub close: bool,
}

/// Modal dialog wrapper.
///
/// - `id_source` must be stable across frames.
/// - `title` is optional.
/// - `body` can render anything.
/// - returns whether the dialog should be closed this frame.
pub fn modal<R: Copy>(
    ctx: &egui::Context,
    id_source: impl std::hash::Hash,
    title: &str,
    body: impl FnOnce(&mut Ui) -> R,
) -> (R, DialogOutput) {
    let modal = egui::Modal::new(Id::new(id_source));

    let response = modal.show(ctx, |ui| {
        ui.set_min_width(360.0);

        ui.vertical(|ui| {
            ui.heading(title);
            ui.separator();
            ui.add_space(8.0);

            let inner = body(ui);

            ui.add_space(12.0);
            let close = false;

            (inner, close)
        })
            .inner
    });

    let (inner, clicked_close) = response.inner;
    let close = clicked_close || response.should_close();

    (inner, DialogOutput { close })
}