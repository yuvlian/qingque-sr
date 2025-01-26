#![windows_subsystem = "windows"] // to hide console

use eframe::egui;
use std::fs;
use std::path::Path;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(egui::vec2(400.0, 304.0))
            .with_resizable(true)
            .with_decorations(true),
        centered: true,
        ..Default::default()
    };
    eframe::run_native(
        "made by yulian with <3",
        options,
        Box::new(|_cc| Ok(Box::new(ConfigManager::default()))),
    )
}

struct ConfigManager {
    files: Vec<String>,
    selected_file: Option<String>,
    logs: String,
}

impl Default for ConfigManager {
    fn default() -> Self {
        Self {
            files: get_files_in_srtools(),
            selected_file: None,
            logs: String::new(),
        }
    }
}

impl eframe::App for ConfigManager {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("qq config.json manager");

            ui.label("Select a config:");
            egui::ComboBox::from_label("(files are in _cfg/srtools/)")
                .selected_text(
                    self.selected_file
                        .clone()
                        .unwrap_or_else(|| "None".to_string()),
                )
                .show_ui(ui, |ui| {
                    for file in &self.files {
                        if ui.selectable_label(false, file).clicked() {
                            self.selected_file = Some(file.clone());
                        }
                    }
                });

            ui.separator();

            if ui.button("Refresh Files").clicked() {
                self.files = get_files_in_srtools();
                self.logs.push_str("> files refreshed.\n");
            }

            if ui.button("Load Config").clicked() {
                if let Some(selected_file) = &self.selected_file {
                    if load_file_into_config(selected_file) {
                        self.logs
                            .push_str(&format!("> loaded config: {}\n", selected_file));
                    } else {
                        self.logs
                            .push_str(&format!("> failed to load config: {}\n", selected_file));
                    }
                } else {
                    self.logs.push_str("> no config selected!\n");
                }
            }

            if ui.button("Clear Logs").clicked() {
                self.logs.clear();
            }

            ui.separator();

            egui::ScrollArea::vertical().show(ui, |ui| {
                egui::TextEdit::multiline(&mut self.logs)
                    .font(egui::TextStyle::Monospace)
                    .code_editor()
                    .desired_rows(10)
                    .lock_focus(true)
                    .show(ui);
            });
        });
    }
}

fn get_files_in_srtools() -> Vec<String> {
    let path = Path::new("./_cfg/srtools");
    if let Ok(entries) = fs::read_dir(path) {
        entries
            .filter_map(|entry| {
                entry.ok().and_then(|e| {
                    e.path()
                        .file_name()
                        .and_then(|name| name.to_str().map(|s| s.to_string()))
                })
            })
            .collect()
    } else {
        vec![]
    }
}

fn load_file_into_config(selected_file: &str) -> bool {
    let srtools_path = Path::new("./_cfg/srtools").join(selected_file);
    let config_path = Path::new("./_cfg/config.json");

    if let Ok(contents) = fs::read_to_string(srtools_path) {
        if fs::write(config_path, contents).is_ok() {
            return true;
        }
    }
    false
}
