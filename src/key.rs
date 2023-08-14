use egui::{Align2, Color32};

#[derive(Default, serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct Key {
    pub label: String,
    pub state: bool,
}

impl Key {
    pub fn new(label: String) -> Self {
        Self {
            label,
            state: false,
        }
    }
}

pub const MAX_KEYS: usize = 12;

#[allow(dead_code)]
fn toggle_ui(ui: &mut egui::Ui, on: &mut Key) -> egui::Response {
    let desired_size = ui.spacing().interact_size.y * egui::vec2(5.0, 5.0);
    let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click());
    // if response.clicked() {
    //     *on = !*on.state;
    //     response.mark_changed();
    // }
    response.widget_info(|| {
        egui::WidgetInfo::selected(egui::WidgetType::Checkbox, on.state, &on.label)
    });

    if ui.is_rect_visible(rect) {
        let how_on = ui.ctx().animate_bool(response.id, on.state);
        let visuals = ui.style().interact_selectable(&response, on.state);
        let rect = rect.expand(visuals.expansion);
        let radius = 0.5 * rect.height();
        ui.painter()
            .rect(rect, radius, visuals.bg_fill, visuals.bg_stroke);
        ui.painter().text(
            rect.center(),
            Align2::CENTER_CENTER,
            &on.label,
            egui::FontId {
                size: 10f32,
                family: egui::FontFamily::Monospace,
            },
            Color32::WHITE,
        );
        // let circle_x = egui::lerp((rect.left() + radius)..=(rect.right() - radius), how_on);
        // let center = egui::pos2(circle_x, rect.center().y);
        // ui.painter()
        //     .circle(center, 0.75 * radius, visuals.bg_fill, visuals.fg_stroke);
    }

    response
}

pub fn toggle(on: &mut Key) -> impl egui::Widget + '_ {
    move |ui: &mut egui::Ui| toggle_ui(ui, on)
}
