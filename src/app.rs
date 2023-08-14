use std::collections::HashMap;

use egui::{Align2, Color32};

use crate::key::{toggle, Key, MAX_KEYS};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,

    #[serde(skip)]
    keys: [Key; MAX_KEYS],
    #[serde(skip)]
    current_key: String,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "tinykeys".to_owned(),
            current_key: "".into(),
            keys: [
                Key::new("▽".into()),
                Key::new("◀".into()),
                Key::new("◆".into()),
                Key::new("▶".into()),
                Key::new("⋐".into()),
                Key::new("⋑".into()),
                Key::new("⋑".into()),
                Key::new("⋐".into()),
                Key::new("◀".into()),
                Key::new("◆".into()),
                Key::new("▶".into()),
                Key::new("△".into()),
            ],
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut layout = HashMap::new();
        //
        layout.insert(egui::Key::Space, ("SPC".to_string(), vec![7]));
        layout.insert(egui::Key::Enter, ("RET".to_string(), vec![6]));
        layout.insert(egui::Key::Backspace, ("BSPC".to_string(), vec![4]));
        layout.insert(egui::Key::Tab, ("TAB".to_string(), vec![5]));
        layout.insert(egui::Key::Escape, ("ESC".to_string(), vec![5, 6]));
        // No mod layer
        layout.insert(egui::Key::Num1, ("1".to_string(), vec![0, 1, 11]));
        layout.insert(egui::Key::Num2, ("2".to_string(), vec![0, 1, 2, 11]));
        layout.insert(egui::Key::Num3, ("3".to_string(), vec![0, 2, 11]));
        layout.insert(egui::Key::Num4, ("4".to_string(), vec![0, 2, 3, 11]));
        layout.insert(egui::Key::Num5, ("5".to_string(), vec![0, 3, 11]));
        layout.insert(egui::Key::Num6, ("6".to_string(), vec![0, 7, 11]));
        layout.insert(egui::Key::Num7, ("7".to_string(), vec![0, 7, 8, 11]));
        layout.insert(egui::Key::Num8, ("8".to_string(), vec![0, 8, 11]));
        layout.insert(egui::Key::Num9, ("9".to_string(), vec![0, 8, 9, 11]));
        layout.insert(egui::Key::Num0, ("0".to_string(), vec![0, 9, 11]));
        //
        layout.insert(egui::Key::Q, ("Q".to_string(), vec![11, 1]));
        layout.insert(egui::Key::W, ("W".to_string(), vec![11, 1, 2]));
        layout.insert(egui::Key::E, ("E".to_string(), vec![11, 2]));
        layout.insert(egui::Key::R, ("R".to_string(), vec![11, 2, 3]));
        layout.insert(egui::Key::T, ("T".to_string(), vec![11, 3]));
        //
        layout.insert(egui::Key::A, ("A".to_string(), vec![1]));
        layout.insert(egui::Key::S, ("S".to_string(), vec![1, 2]));
        layout.insert(egui::Key::D, ("D".to_string(), vec![2]));
        layout.insert(egui::Key::F, ("F".to_string(), vec![2, 3]));
        layout.insert(egui::Key::G, ("G".to_string(), vec![3]));
        layout.insert(egui::Key::H, ("H".to_string(), vec![7]));
        layout.insert(egui::Key::J, ("J".to_string(), vec![7, 8]));
        layout.insert(egui::Key::K, ("K".to_string(), vec![8]));
        layout.insert(egui::Key::L, ("L".to_string(), vec![8, 9]));
        //
        layout.insert(egui::Key::Z, ("Z".to_string(), vec![0, 1]));
        layout.insert(egui::Key::X, ("X".to_string(), vec![0, 1, 2]));
        layout.insert(egui::Key::C, ("C".to_string(), vec![0, 2]));
        layout.insert(egui::Key::V, ("V".to_string(), vec![0, 2, 3]));
        layout.insert(egui::Key::B, ("B".to_string(), vec![0, 3]));

        //TODO: reset all states to false
        egui::CentralPanel::default().show(ctx, |ui| {
            for (event, (key, ixs)) in layout {
                if ui.input(|i| i.key_pressed(event)) {
                    for ix in &ixs {
                        self.keys[*ix].state = true;
                    }
                    self.current_key = key;
                }
                if ui.input(|i| i.key_released(event)) {
                    for ix in &ixs {
                        self.keys[*ix].state = false;
                    }
                    self.current_key = "".to_string();
                }
            }

            ui.heading("eframe template");
            let desired_size = ui.available_size() * egui::vec2(0.1, 0.2);
            let (rect, mut _response) = ui.allocate_exact_size(desired_size, egui::Sense::click());
            ui.painter().text(
                rect.center(),
                Align2::CENTER_CENTER,
                &self.current_key,
                egui::FontId {
                    size: 30f32,
                    family: egui::FontFamily::Monospace,
                },
                Color32::WHITE,
            );
            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                for i in 0..MAX_KEYS {
                    ui.add(toggle(&mut self.keys[i]));
                    if i == 5 {
                        ui.separator();
                    }
                }
            });
            egui::warn_if_debug_build(ui);
        });
    }
}
