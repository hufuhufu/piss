#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct App {
    title: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            title: "PISS".to_owned(),
        }
    }
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        Default::default()
    }
}

impl eframe::App for App {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
                let _ = ui.button("About");
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Priconne Inventory Scanning System");
            egui::warn_if_debug_build(ui);

            let scan_file_button = ui.button("Scan file(s)")
                .on_hover_text("Scan inventory from screenshot image files.");
            if scan_file_button.clicked() {
                // TODO: Show file dialog
            }

            let scan_clipboard_button = ui.button("From clipboard")
                .on_hover_text("Scan inventory from clipboard.\nTip: You can use Win+Shift+S to take a screenshot and automatically put it in your clipboard.");
            if scan_clipboard_button.clicked() {
                // TODO: Get image from the clipboard
            }
        });
    }
}
