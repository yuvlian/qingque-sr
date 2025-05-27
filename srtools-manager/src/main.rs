#![windows_subsystem = "windows"] // to hide console

use eframe::egui;
use std::fs;
use std::path::Path;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(egui::vec2(800.0, 600.0))
            .with_resizable(true)
            .with_decorations(true),
        centered: true,
        ..Default::default()
    };
    eframe::run_native(
        "SRTools Manager",
        options,
        Box::new(|_cc| Ok(Box::new(ConfigManager::default()))),
    )
}

struct ConfigManager {
    files: Vec<String>,
    selected_file: Option<String>,
    search_text: String,
    logs: String,
    show_config_list: bool,
}

impl Default for ConfigManager {
    fn default() -> Self {
        Self {
            files: get_files_in_srtools(),
            selected_file: None,
            search_text: String::new(),
            logs: String::new(),
            show_config_list: false,
        }
    }
}

impl eframe::App for ConfigManager {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("SRTools Manager");
            ui.label("- An app to help managing config.json from SRTools.");
            ui.label("- You can put your config.json(s) in [ _configs_/srtools/ ] and name the file something else.");
            if ui.button("Open [ ./_configs/srtools/ ]").clicked() {
                let _ = open::that("./_configs_/srtools/");
            }

            ui.separator();

            ui.label("Select a config:");
            ui.horizontal(|ui| {
                if ui.button(if self.show_config_list { "Hide configs" } else { "Show configs" }).clicked() {
                    self.show_config_list = !self.show_config_list;
                    if self.show_config_list {
                        self.files = get_files_in_srtools();
                    }
                }

                let search_response = ui.add(
                    egui::TextEdit::singleline(&mut self.search_text)
                        .hint_text("search for config")
                        .id_source("search_box"),
                );

                if search_response.gained_focus() {
                    self.files = get_files_in_srtools();
                    self.show_config_list = true;
                }

                if search_response.clicked() {
                    self.show_config_list = true;
                }
            });

            if self.show_config_list {
                ui.push_id("config_scroll", |ui| {
                    let filtered_files: Vec<&String> = self
                        .files
                        .iter()
                        .filter(|f| f.to_lowercase().contains(&self.search_text.to_lowercase()))
                        .collect();

                    egui::ScrollArea::vertical()
                        .max_height(100.0)
                        .show(ui, |ui| {
                            for file in filtered_files {
                                if ui
                                    .selectable_label(self.selected_file.as_ref() == Some(file), file)
                                    .clicked()
                                {
                                    self.selected_file = Some(file.clone());
                                    if load_file_into_config(file) {
                                        self.logs
                                            .push_str(&format!("> loaded config: {}\n", file));
                                    } else {
                                        self.logs
                                            .push_str(&format!("> failed to load config: {}\n", file));
                                    }

                                    self.show_config_list = false;
                                }
                            }
                        });
                });
            }

            ui.separator();

            if ui.button("Clear Logs").clicked() {
                self.logs.clear();
            }

            ui.separator();

            ui.push_id("logs_scroll", |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    egui::TextEdit::multiline(&mut self.logs)
                        .code_editor()
                        .desired_rows(10)
                        .show(ui);
                });
            });
        });
    }
}

fn get_files_in_srtools() -> Vec<String> {
    let path = Path::new("./_configs_/srtools");
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
        if fs::create_dir_all(path).is_ok() {
            Vec::new()
        } else {
            Vec::new()
        }
    }
}

fn load_file_into_config(selected_file: &str) -> bool {
    let srtools_path = Path::new("./_configs_/srtools").join(selected_file);
    let config_path = Path::new("./_configs_/config.json");

    if let Some(parent) = config_path.parent() {
        let _ = fs::create_dir_all(parent);
    }

    if let Ok(contents) = fs::read_to_string(srtools_path) {
        fs::write(config_path, contents).is_ok()
    } else {
        false
    }
}
