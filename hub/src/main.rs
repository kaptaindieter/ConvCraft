//! ConvCraft - Universal Media & File Converter Workbench
//! Part of the Craft Application Suite.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod artcraft_agent;
mod artcraft_drive;
mod artcraft_theme;
mod batch_queue;
mod control;
mod converter;

use std::path::PathBuf;
use std::sync::mpsc::{self, Receiver};
use std::time::Duration;

use eframe::egui::{
    self, Align, Color32, CornerRadius, Layout, Rect, RichText, ScrollArea, Stroke, pos2, vec2,
};
use serde_json::{Value, json};

use crate::artcraft_agent::ArtCraftAgent;
use crate::artcraft_drive::ArtCraftDrive;
use crate::artcraft_theme::{
    AURORA_CARD, AURORA_PANEL, AppTheme, GEMINI_CYAN, GEMINI_PURPLE,
    apply_theme, paint_gemini_rainbow_line, render_ai_agent_button,
    render_theme_switcher_dropdown,
};
use crate::batch_queue::{
    BatchQueueState, ConversionItem, ConversionStatus, FileCategory,
};
use crate::control::{ControlRequest, start_control_server};
use crate::converter::{ConversionEvent, ConverterEngine};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum LeftSidebarTab {
    All,
    Images,
    Audio,
    Video,
    Documents,
    Effects,
}

impl LeftSidebarTab {
    pub fn label(&self) -> &'static str {
        match self {
            Self::All => "🏠 All Files",
            Self::Images => "🖼 Images",
            Self::Audio => "🎵 Audio",
            Self::Video => "🎬 Video",
            Self::Documents => "📄 Documents",
            Self::Effects => "🎨 Visual Effects",
        }
    }
}

#[derive(Clone, Copy)]
pub struct ConverterCard {
    pub from: &'static str,
    pub to: &'static str,
    pub cat: LeftSidebarTab,
    pub desc: &'static str,
    pub color: Color32,
    pub exts: &'static [&'static str],
}

pub const CONVERTER_CARDS: &[ConverterCard] = &[
    ConverterCard { from: "JPG", to: "PNG", cat: LeftSidebarTab::Images, desc: "Lossless transparent image", color: Color32::from_rgb(59, 130, 246), exts: &["jpg", "jpeg"] },
    ConverterCard { from: "PNG", to: "WEBP", cat: LeftSidebarTab::Images, desc: "Ultra-fast modern web compression", color: Color32::from_rgb(16, 185, 129), exts: &["png"] },
    ConverterCard { from: "WEBP", to: "JPG", cat: LeftSidebarTab::Images, desc: "Standard high-compatibility image", color: Color32::from_rgb(245, 158, 11), exts: &["webp"] },
    ConverterCard { from: "SVG", to: "PNG", cat: LeftSidebarTab::Images, desc: "Rasterize vector graphics to high-res", color: Color32::from_rgb(236, 72, 153), exts: &["svg"] },
    ConverterCard { from: "MP4", to: "MP3", cat: LeftSidebarTab::Audio, desc: "Extract 320kbps audio track", color: Color32::from_rgb(139, 92, 246), exts: &["mp4", "m4v", "mov", "mkv"] },
    ConverterCard { from: "MOV", to: "MP4", cat: LeftSidebarTab::Video, desc: "Apple QuickTime to universal H.264", color: Color32::from_rgb(99, 102, 241), exts: &["mov"] },
    ConverterCard { from: "WAV", to: "MP3", cat: LeftSidebarTab::Audio, desc: "Compress uncompressed master audio", color: Color32::from_rgb(14, 165, 233), exts: &["wav"] },
    ConverterCard { from: "FLAC", to: "WAV", cat: LeftSidebarTab::Audio, desc: "Lossless decompression for editing", color: Color32::from_rgb(20, 184, 166), exts: &["flac"] },
    ConverterCard { from: "MP4", to: "GIF", cat: LeftSidebarTab::Images, desc: "Convert video clip to animated GIF", color: Color32::from_rgb(249, 115, 22), exts: &["mp4", "mov", "webm"] },
    ConverterCard { from: "DOCX", to: "PDF", cat: LeftSidebarTab::Documents, desc: "Preserve fonts and layout formatting", color: Color32::from_rgb(239, 68, 68), exts: &["docx", "doc"] },
    ConverterCard { from: "PDF", to: "JPG", cat: LeftSidebarTab::Documents, desc: "Extract pages as high-resolution images", color: Color32::from_rgb(217, 70, 239), exts: &["pdf"] },
    ConverterCard { from: "Images", to: "PDF", cat: LeftSidebarTab::Documents, desc: "Combine multiple images into one PDF", color: Color32::from_rgb(168, 85, 247), exts: &["png", "jpg", "jpeg", "webp"] },
    ConverterCard { from: "MKV", to: "MP4", cat: LeftSidebarTab::Video, desc: "Stream-copy to universal video format", color: Color32::from_rgb(59, 130, 246), exts: &["mkv"] },
    ConverterCard { from: "Audio", to: "FLAC", cat: LeftSidebarTab::Audio, desc: "Lossless studio archival quality", color: Color32::from_rgb(16, 185, 129), exts: &["wav", "mp3", "m4a", "aac"] },
    ConverterCard { from: "Video", to: "WEBM", cat: LeftSidebarTab::Video, desc: "Next-gen VP9/AV1 web video format", color: Color32::from_rgb(139, 92, 246), exts: &["mp4", "mov", "mkv"] },
    ConverterCard { from: "DWG", to: "PDF", cat: LeftSidebarTab::Documents, desc: "Vector CAD blueprint to vector PDF", color: Color32::from_rgb(245, 158, 11), exts: &["dwg", "dxf"] },
];

pub struct ConvCraftApp {
    queue: BatchQueueState,
    drive: ArtCraftDrive,
    agent: ArtCraftAgent,
    converter: ConverterEngine,
    theme: AppTheme,
    active_tab: LeftSidebarTab,
    search_query: String,
    status_msg: String,
    batch_target_format: String,
    control_rx: Option<Receiver<ControlRequest>>,
}

impl ConvCraftApp {
    pub fn new(cc: &eframe::CreationContext) -> Self {
        install_symbol_fonts(cc);
        let theme = AppTheme::load_global();
        let drive = ArtCraftDrive::new();
        let queue = BatchQueueState::new();
        let agent = ArtCraftAgent::new();
        let converter = ConverterEngine::new();

        let mut app = Self {
            queue,
            drive,
            agent,
            converter,
            theme,
            active_tab: LeftSidebarTab::All,
            search_query: String::new(),
            status_msg: "ConvCraft ready. Drag & drop files or click Add Files.".to_string(),
            batch_target_format: "webp".to_string(),
            control_rx: None,
        };

        // Parse CLI args: files or --control <port>
        let args: Vec<String> = std::env::args().collect();
        let mut idx = 1;
        let mut initial_paths = Vec::new();
        while idx < args.len() {
            if args[idx] == "--control" && idx + 1 < args.len() {
                if let Ok(port) = args[idx + 1].parse::<u16>() {
                    if let Ok(rx) = start_control_server(port) {
                        app.control_rx = Some(rx);
                        app.status_msg = format!("Control Channel listening on 127.0.0.1:{port}");
                    }
                }
                idx += 2;
                continue;
            }
            let p = PathBuf::from(&args[idx]);
            if p.exists() {
                initial_paths.push(p);
            }
            idx += 1;
        }

        if !initial_paths.is_empty() {
            app.queue.add_paths(initial_paths);
        }

        app
    }

    pub fn start_batch_conversion(&mut self) -> usize {
        let out_dir = self.drive.output_dir();
        let ready_items: Vec<ConversionItem> = self.queue.items.iter()
            .filter(|i| matches!(i.status, ConversionStatus::Pending | ConversionStatus::Error { .. }))
            .cloned()
            .collect();
        let count = ready_items.len();
        if count > 0 {
            for item in &ready_items {
                self.queue.set_item_status(item.id, ConversionStatus::Converting { progress: 0.05, stage: "Queued".to_string() });
            }
            self.converter.start_batch(ready_items, out_dir);
            self.status_msg = format!("Started batch conversion of {count} files...");
        }
        count
    }

    pub fn start_single_conversion(&mut self, item_id: usize) {
        let out_dir = self.drive.output_dir();
        if let Some(item) = self.queue.items.iter().find(|i| i.id == item_id).cloned() {
            self.queue.set_item_status(item.id, ConversionStatus::Converting { progress: 0.05, stage: "Starting...".to_string() });
            self.converter.start_batch(vec![item], out_dir);
        }
    }

    pub fn export_all_to_drive(&self) -> Result<usize, String> {
        let mut count = 0;
        let drive_dir = self.drive.output_dir();
        let _ = std::fs::create_dir_all(&drive_dir);
        for item in &self.queue.items {
            if let ConversionStatus::Done { ref out_path } = item.status {
                if out_path.exists() {
                    let dest = drive_dir.join(out_path.file_name().unwrap_or_default());
                    if dest != *out_path {
                        let _ = std::fs::copy(out_path, dest);
                        count += 1;
                    }
                }
            }
        }
        Ok(count)
    }

    pub fn execute_command(&mut self, command: &str, params: &Value) -> Result<Value, String> {
        match command {
            "execute" => {
                let inner_cmd = params.get("command").and_then(Value::as_str).unwrap_or("");
                let inner_params = params.get("params").cloned().unwrap_or_else(|| params.clone());
                self.execute_command(inner_cmd, &inner_params)
            }
            "conv.addFiles" | "addFiles" => {
                let paths_val = params.get("paths").and_then(Value::as_array);
                let mut added = 0;
                if let Some(arr) = paths_val {
                    let pbs: Vec<PathBuf> = arr.iter().filter_map(|v| v.as_str().map(PathBuf::from)).collect();
                    added = self.queue.add_paths(pbs);
                }
                Ok(json!({"added_files": added, "total_queue": self.queue.items.len()}))
            }
            "conv.clearQueue" | "clearQueue" => {
                self.queue.clear();
                Ok(json!({"status": "queue_cleared"}))
            }
            "conv.removeFile" | "removeFile" => {
                if let Some(id) = params.get("id").and_then(Value::as_u64) {
                    self.queue.remove_item(id as usize);
                    return Ok(json!({"status": "removed", "id": id}));
                }
                if let Some(idx) = params.get("index").and_then(Value::as_u64) {
                    let id_opt = self.queue.items.get(idx as usize).map(|i| i.id);
                    if let Some(id) = id_opt {
                        self.queue.remove_item(id);
                        return Ok(json!({"status": "removed", "id": id}));
                    }
                }
                Err("Item not found in queue".to_string())
            }
            "conv.setTargetFormat" | "setTargetFormat" => {
                let format = params.get("format").and_then(Value::as_str).ok_or("Missing format parameter")?;
                let filter = params.get("filter").and_then(Value::as_str);
                let cat = filter.and_then(FileCategory::from_str);
                let count = self.queue.set_target_format_all(format, cat);
                Ok(json!({"updated_items": count, "target_format": format}))
            }
            "conv.setCompressionAndQuality" | "setCompressionAndQuality" => {
                let quality = params.get("quality").and_then(Value::as_u64).map(|v| v as u32);
                let bitrate = params.get("bitrate").and_then(Value::as_str);
                let crf = params.get("crf").and_then(Value::as_u64).map(|v| v as u32);
                let count = self.queue.update_settings_all(|s| {
                    if let Some(q) = quality { s.quality = q; }
                    if let Some(b) = bitrate { s.audio_bitrate = b.to_string(); }
                    if let Some(c) = crf { s.video_crf = Some(c); }
                });
                Ok(json!({"updated_items": count, "quality": quality, "bitrate": bitrate, "crf": crf}))
            }
            "conv.setResizeAndTransform" | "setResizeAndTransform" => {
                let width = params.get("width").and_then(Value::as_u64).map(|v| v as u32);
                let height = params.get("height").and_then(Value::as_u64).map(|v| v as u32);
                let count = self.queue.update_settings_all(|s| {
                    if let Some(w) = width { s.resize_width = Some(w); }
                    if let Some(h) = height { s.resize_height = Some(h); }
                });
                Ok(json!({"updated_items": count, "width": width, "height": height}))
            }
            "conv.applyVisualEffect" | "applyVisualEffect" => {
                let effect_str = params.get("effect").and_then(Value::as_str).unwrap_or("none");
                let eff_opt = if effect_str == "none" { None } else { Some(effect_str.to_string()) };
                let count = self.queue.update_settings_all(|s| {
                    s.effect = eff_opt.clone();
                });
                Ok(json!({"updated_items": count, "effect": effect_str}))
            }
            "conv.startConversion" | "startConversion" => {
                let started = self.start_batch_conversion();
                Ok(json!({"status": "conversion_started", "jobs_dispatched": started}))
            }
            "conv.exportToDrive" | "exportToDrive" => {
                let exported = self.export_all_to_drive()?;
                Ok(json!({"status": "exported_to_drive", "files_copied": exported, "drive_path": self.drive.output_dir().display().to_string()}))
            }
            "conv.saveAll" | "saveAll" => {
                let p = self.drive.output_dir().join("convcraft_queue_preset.json");
                let data = serde_json::to_string_pretty(&self.queue.items).map_err(|e| e.to_string())?;
                std::fs::write(&p, data).map_err(|e| e.to_string())?;
                Ok(json!({"status": "saved", "path": p.display().to_string()}))
            }
            "conv.inspectQueue" | "inspectQueue" => {
                Ok(json!({
                    "total_items": self.queue.items.len(),
                    "items": self.queue.items.iter().map(|item| json!({
                        "id": item.id,
                        "file_name": item.file_name,
                        "category": item.category.label(),
                        "source_format": item.source_ext,
                        "target_format": item.target_format,
                        "status": match &item.status {
                            ConversionStatus::Pending => "pending",
                            ConversionStatus::Converting { .. } => "converting",
                            ConversionStatus::Done { .. } => "done",
                            ConversionStatus::Error { .. } => "error",
                        }
                    })).collect::<Vec<_>>()
                }))
            }
            _ => Err(format!("Unknown ConvCraft command: {command}")),
        }
    }

    fn render_top_bar(&mut self, ui: &mut egui::Ui) {
        egui::Panel::top("top_header")
            .exact_size(40.0)
            .show(ui, |ui| {
                ui.horizontal_centered(|ui| {
                    ui.add_space(8.0);
                    // Left Zone: Home, Add Files, Undo, Redo (clean text buttons)
                    if ui.button(RichText::new("Home").size(12.0)).clicked() {
                        self.active_tab = LeftSidebarTab::All;
                        self.search_query.clear();
                    }
                    if ui.button(RichText::new("Add Files").size(12.0).strong().color(GEMINI_CYAN)).clicked() {
                        if let Some(files) = rfd::FileDialog::new().pick_files() {
                            self.queue.add_paths(files);
                        }
                    }
                    ui.add_enabled_ui(self.queue.can_undo(), |ui| {
                        if ui.button(RichText::new("Undo").size(12.0)).clicked() {
                            self.queue.undo();
                        }
                    });
                    ui.add_enabled_ui(self.queue.can_redo(), |ui| {
                        if ui.button(RichText::new("Redo").size(12.0)).clicked() {
                            self.queue.redo();
                        }
                    });

                    ui.separator();

                    // Center Zone: Title & Search
                    ui.label(RichText::new("ConvCraft").strong().size(14.0).color(GEMINI_PURPLE));
                    ui.add(
                        egui::TextEdit::singleline(&mut self.search_query)
                            .hint_text("Search files, formats and commands...")
                            .desired_width(240.0),
                    );

                    // Right Zone: Convert All, Auto-Save, Google Drive, AI Agent
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        ui.add_space(8.0);
                        // AI Agent Pill
                        if render_ai_agent_button(ui, self.agent.is_open).clicked() {
                            self.agent.is_open = !self.agent.is_open;
                        }

                        // Drive Status (Clickable to configure Cloud Sync Folder)
                        let modal_id = ui.make_persistent_id("convcraft_cloud_modal_open");
                        let cfg_id = ui.make_persistent_id("convcraft_cloud_modal_cfg");
                        let mut cloud_open = ui.ctx().data_mut(|d| d.get_temp::<bool>(modal_id).unwrap_or(false));
                        let mut cloud_cfg = ui
                            .ctx()
                            .data_mut(|d| d.get_temp::<crate::artcraft_theme::CraftSuiteCloudConfig>(cfg_id))
                            .unwrap_or_else(crate::artcraft_theme::CraftSuiteCloudConfig::load);

                        let drive_color = if self.drive.is_google_drive() {
                            Color32::from_rgb(52, 168, 83)
                        } else {
                            Color32::from_rgb(234, 67, 53)
                        };
                        let drive_resp = ui
                            .button(RichText::new(format!("● {}", self.drive.status_label())).size(11.0).color(drive_color))
                            .on_hover_text("Left-click: Configure Cloud & Sync Folder\nRight-click: Open output folder in Explorer");
                        if drive_resp.clicked() {
                            cloud_cfg = crate::artcraft_theme::CraftSuiteCloudConfig::load();
                            cloud_open = !cloud_open;
                        } else if drive_resp.secondary_clicked() {
                            let out = self.drive.output_dir();
                            let _ = std::process::Command::new("explorer").arg(out).spawn();
                        }
                        if cloud_open {
                            let out_dir = self.drive.output_dir();
                            crate::artcraft_theme::render_cloud_sync_modal(
                                ui.ctx(),
                                &mut cloud_open,
                                &mut cloud_cfg,
                                Some(&out_dir),
                            );
                        }
                        ui.ctx().data_mut(|d| {
                            d.insert_temp(modal_id, cloud_open);
                            d.insert_temp(cfg_id, cloud_cfg);
                        });

                        // Auto-Save
                        ui.label(RichText::new("✓ Auto-Save").size(11.0).color(Color32::from_rgb(52, 168, 83)));

                        // Primary Convert All Button
                        let conv_btn = ui.button(
                            RichText::new("⚡ Convert All")
                                .size(12.0)
                                .color(Color32::WHITE)
                                .strong(),
                        );
                        if conv_btn.clicked() {
                            self.start_batch_conversion();
                        }
                    });
                });

                // Rainbow line at the bottom of the Top Bar
                let rect = Rect::from_min_size(
                    pos2(ui.min_rect().min.x, ui.min_rect().max.y - 2.0),
                    vec2(ui.min_rect().width(), 2.0),
                );
                paint_gemini_rainbow_line(ui.painter(), rect);
            });
    }

    fn render_footer(&mut self, ui: &mut egui::Ui) {
        egui::Panel::bottom("bottom_bar")
            .exact_size(28.0)
            .show(ui, |ui| {
                ui.horizontal_centered(|ui| {
                    ui.add_space(8.0);
                    let total = self.queue.items.len();
                    let done = self.queue.items.iter().filter(|i| matches!(i.status, ConversionStatus::Done { .. })).count();
                    let converting = self.queue.items.iter().filter(|i| matches!(i.status, ConversionStatus::Converting { .. })).count();

                    if converting > 0 {
                        ui.label(RichText::new(format!("⏳ Converting ({converting} active, {done}/{total} done)...")).size(11.0).color(GEMINI_CYAN));
                    } else if total > 0 {
                        ui.label(RichText::new(format!("{total} files in queue ({done} completed) • Output: {}", self.drive.output_dir().display())).size(11.0));
                    } else {
                        ui.label(RichText::new(&self.status_msg).size(11.0));
                    }

                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        ui.add_space(8.0);
                        let mut cur_theme = self.theme;
                        if render_theme_switcher_dropdown(ui, &mut cur_theme) {
                            self.theme = cur_theme;
                        }
                        ui.separator();
                        ui.label(RichText::new(format!("ConvCraft v{}", env!("CARGO_PKG_VERSION"))).size(10.0).weak());
                    });
                });
            });
    }

    fn render_left_sidebar(&mut self, ui: &mut egui::Ui) {
        egui::Panel::left("left_sidebar")
            .exact_size(190.0)
            .show(ui, |ui| {
                ui.add_space(8.0);
                ui.heading(RichText::new("Converters").size(14.0).strong());
                ui.add_space(4.0);

                let tabs = [
                    LeftSidebarTab::All,
                    LeftSidebarTab::Images,
                    LeftSidebarTab::Audio,
                    LeftSidebarTab::Video,
                    LeftSidebarTab::Documents,
                    LeftSidebarTab::Effects,
                ];

                for tab in tabs {
                    let count = match tab {
                        LeftSidebarTab::All => self.queue.items.len(),
                        LeftSidebarTab::Images => self.queue.items.iter().filter(|i| i.category == FileCategory::Images).count(),
                        LeftSidebarTab::Audio => self.queue.items.iter().filter(|i| i.category == FileCategory::Audio).count(),
                        LeftSidebarTab::Video => self.queue.items.iter().filter(|i| i.category == FileCategory::Video).count(),
                        LeftSidebarTab::Documents => self.queue.items.iter().filter(|i| i.category == FileCategory::Documents).count(),
                        LeftSidebarTab::Effects => self.queue.items.iter().filter(|i| i.settings.effect.is_some()).count(),
                    };

                    let is_sel = self.active_tab == tab;
                    let text = format!("{} ({count})", tab.label());
                    let btn = ui.selectable_label(is_sel, text);
                    if btn.clicked() {
                        self.active_tab = tab;
                    }
                }

                ui.add_space(12.0);
                ui.separator();
                ui.add_space(8.0);
                ui.label(RichText::new("Batch Presets").size(11.0).weak().strong());
                ui.add_space(4.0);

                if ui.button(RichText::new("⚡ WebP (Quality 85)").size(11.0)).clicked() {
                    self.queue.set_target_format_all("webp", Some(FileCategory::Images));
                    self.queue.update_settings_all(|s| s.quality = 85);
                    self.status_msg = "Set all images to WebP (Quality 85)".to_string();
                }
                if ui.button(RichText::new("🎵 Extract MP3 (320k)").size(11.0)).clicked() {
                    self.queue.set_target_format_all("mp3", Some(FileCategory::Video));
                    self.queue.update_settings_all(|s| s.audio_bitrate = "320k".to_string());
                    self.status_msg = "Configured MP3 320k audio extraction for all videos".to_string();
                }
                if ui.button(RichText::new("🎬 MP4 (H.264 Fast)").size(11.0)).clicked() {
                    self.queue.set_target_format_all("mp4", Some(FileCategory::Video));
                    self.queue.update_settings_all(|s| s.video_crf = Some(23));
                    self.status_msg = "Configured all videos for MP4 H.264 standard".to_string();
                }
                if ui.button(RichText::new("📄 Images to PDF").size(11.0)).clicked() {
                    self.queue.set_target_format_all("pdf", Some(FileCategory::Images));
                    self.status_msg = "Configured images for PDF compilation".to_string();
                }
                if ui.button(RichText::new("Export to Drive").size(11.0)).clicked() {
                    if let Ok(c) = self.export_all_to_drive() {
                        self.status_msg = format!("Exported {c} files directly to Google Drive");
                    }
                }

                ui.add_space(16.0);
                ui.separator();
                ui.add_space(8.0);
                ui.label(RichText::new("Quick Tools").size(11.0).weak().strong());
                if ui.button(RichText::new("Open Drive Folder").size(11.0)).clicked() {
                    let _ = std::process::Command::new("explorer").arg(&self.drive.output_dir()).spawn();
                }
                if ui.button(RichText::new("Clear Queue").size(11.0)).clicked() {
                    self.queue.clear();
                }
            });
    }

    fn render_central_workbench(&mut self, ui: &mut egui::Ui) {
        egui::CentralPanel::default().show(ui, |ui| {
            let mut to_convert_single: Option<usize> = None;
            let mut to_remove_single: Option<usize> = None;
            let mut to_open_folder: Option<PathBuf> = None;
            let mut card_to_launch: Option<&'static ConverterCard> = None;
            let mut auto_detect_target_fmt: Option<String> = None;

            ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
                ui.spacing_mut().item_spacing = vec2(12.0, 12.0);

                // =============================================================
                // A. HERO DROPZONE BOX (180-200px, max-width 860px, Centered)
                // =============================================================
                let max_hero_w = 860.0f32.min(ui.available_width() - 32.0);
                ui.vertical_centered(|ui| {
                    ui.add_space(4.0);
                    let (hero_rect, _) = ui.allocate_exact_size(vec2(max_hero_w, 188.0), egui::Sense::hover());

                    let hero_bg = Color32::from_rgba_unmultiplied(22, 27, 48, 140);
                    let dashed_stroke = Stroke::new(2.0, Color32::from_rgba_unmultiplied(139, 92, 246, 120));
                    ui.painter().rect(hero_rect, CornerRadius::same(16), hero_bg, dashed_stroke, egui::epaint::StrokeKind::Inside);

                    let mut hero_ui = ui.new_child(egui::UiBuilder::new().max_rect(hero_rect.shrink(14.0)));
                    hero_ui.vertical_centered(|ui| {
                        ui.add_space(2.0);
                        ui.label(RichText::new("🦎").size(32.0));
                        ui.add_space(4.0);

                        // Centered Primary Button + Dropdown
                        ui.horizontal_centered(|ui| {
                            let choose_btn = egui::Button::new(
                                RichText::new("📁 Choose Files").size(14.0).strong().color(Color32::WHITE)
                            )
                            .fill(Color32::from_rgb(124, 58, 237))
                            .stroke(Stroke::new(1.0, Color32::from_rgb(167, 139, 250)))
                            .corner_radius(CornerRadius::same(10))
                            .min_size(vec2(160.0, 38.0));

                            if ui.add(choose_btn).clicked() {
                                if let Some(files) = rfd::FileDialog::new().pick_files() {
                                    self.queue.add_paths(files);
                                }
                            }

                            ui.add_space(4.0);
                            egui::ComboBox::from_id_salt("hero_choose_dropdown")
                                .selected_text("▾")
                                .width(40.0)
                                .show_ui(ui, |ui| {
                                    if ui.button("📁 Choose Folder...").clicked() {
                                        if let Some(folder) = rfd::FileDialog::new().pick_folder() {
                                            self.queue.add_paths(vec![folder]);
                                        }
                                    }
                                    if ui.button("☁️ From Google Drive (G:)").clicked() {
                                        let drive_path = self.drive.output_dir();
                                        if let Some(files) = rfd::FileDialog::new().set_directory(&drive_path).pick_files() {
                                            self.queue.add_paths(files);
                                        }
                                    }
                                });
                        });

                        ui.add_space(8.0);
                        ui.label(RichText::new("or drag & drop files here — Auto-detects format & shows matching converters").size(12.5).color(Color32::from_rgb(148, 163, 184)));

                        // Smart Auto-Detection Bar
                        if let Some(latest) = self.queue.items.last() {
                            ui.add_space(8.0);
                            let ext = latest.source_ext.to_lowercase();
                            let detected_pills: Vec<(&'static str, &'static str)> = match latest.category {
                                FileCategory::Video => vec![
                                    ("Convert to MP4", "mp4"),
                                    ("Extract MP3 Audio", "mp3"),
                                    ("Convert to GIF", "gif"),
                                    ("Convert to WebM", "webm"),
                                ],
                                FileCategory::Audio => vec![
                                    ("Convert to MP3", "mp3"),
                                    ("Convert to WAV", "wav"),
                                    ("Convert to FLAC", "flac"),
                                    ("Convert to AAC", "aac"),
                                ],
                                FileCategory::Images => vec![
                                    ("Convert to WebP", "webp"),
                                    ("Convert to PNG", "png"),
                                    ("Convert to JPG", "jpg"),
                                    ("Convert to PDF", "pdf"),
                                ],
                                FileCategory::Documents => vec![
                                    ("Convert to PDF", "pdf"),
                                    ("Extract JPG Pages", "jpg"),
                                    ("Extract PNG Pages", "png"),
                                    ("Convert to WebP", "webp"),
                                ],
                            };

                            ui.horizontal_centered(|ui| {
                                ui.label(RichText::new(format!("Detected .{ext}:")).size(11.5).strong().color(Color32::from_rgb(192, 132, 252)));
                                for (pill_lbl, pill_fmt) in detected_pills {
                                    let btn = egui::Button::new(RichText::new(pill_lbl).size(11.0).color(Color32::from_rgb(226, 232, 240)))
                                        .fill(Color32::from_rgba_unmultiplied(124, 58, 237, 45))
                                        .stroke(Stroke::new(1.0, Color32::from_rgba_unmultiplied(167, 139, 250, 90)))
                                        .corner_radius(CornerRadius::same(24));
                                    if ui.add(btn).clicked() {
                                        auto_detect_target_fmt = Some(pill_fmt.to_string());
                                    }
                                }
                            });
                        }
                    });
                });

                if let Some(ref target_fmt) = auto_detect_target_fmt {
                    self.batch_target_format = target_fmt.clone();
                    self.queue.set_target_format_all(target_fmt, None);
                }

                // =============================================================
                // B. ACTIVE CONVERSION QUEUE (When items exist in queue)
                // =============================================================
                if !self.queue.items.is_empty() {
                    ui.add_space(8.0);
                    egui::Frame::new()
                        .fill(Color32::from_rgba_unmultiplied(15, 19, 34, 180))
                        .stroke(Stroke::new(1.0, Color32::from_rgba_unmultiplied(255, 255, 255, 18)))
                        .corner_radius(CornerRadius::same(12))
                        .inner_margin(14.0)
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.heading(RichText::new(format!("Conversion Queue ({} files)", self.queue.items.len())).size(15.5).color(Color32::WHITE));

                                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                    let conv_all_btn = egui::Button::new(
                                        RichText::new("⚡ Convert All").size(12.5).strong().color(Color32::WHITE)
                                    )
                                    .fill(Color32::from_rgb(16, 185, 129))
                                    .corner_radius(CornerRadius::same(8));

                                    if ui.add(conv_all_btn).clicked() {
                                        self.start_batch_conversion();
                                    }

                                    if ui.button(RichText::new("Add Files").size(12.0)).clicked() {
                                        if let Some(files) = rfd::FileDialog::new().pick_files() {
                                            self.queue.add_paths(files);
                                        }
                                    }

                                    if ui.button(RichText::new("Clear").size(12.0)).clicked() {
                                        self.queue.clear();
                                    }

                                    ui.label(RichText::new("Set all to:").size(11.5).color(Color32::from_rgb(148, 163, 184)));
                                    egui::ComboBox::from_id_salt("queue_batch_fmt_combo")
                                        .selected_text(&self.batch_target_format)
                                        .show_ui(ui, |ui| {
                                            for fmt in &["webp", "png", "jpg", "mp3", "wav", "mp4", "mkv", "pdf"] {
                                                if ui.selectable_value(&mut self.batch_target_format, fmt.to_string(), *fmt).clicked() {
                                                    let fmt_str = self.batch_target_format.clone();
                                                    self.queue.set_target_format_all(&fmt_str, None);
                                                }
                                            }
                                        });
                                });
                            });

                            ui.add_space(8.0);
                            ui.separator();
                            ui.add_space(8.0);

                            let search = self.search_query.to_lowercase();
                            let tab = self.active_tab;

                            for (idx, item) in self.queue.items.iter_mut().enumerate() {
                                if !search.is_empty() && !item.file_name.to_lowercase().contains(&search) {
                                    continue;
                                }
                                match tab {
                                    LeftSidebarTab::All => {}
                                    LeftSidebarTab::Images if item.category != FileCategory::Images => continue,
                                    LeftSidebarTab::Audio if item.category != FileCategory::Audio => continue,
                                    LeftSidebarTab::Video if item.category != FileCategory::Video => continue,
                                    LeftSidebarTab::Documents if item.category != FileCategory::Documents => continue,
                                    LeftSidebarTab::Effects if item.settings.effect.is_none() => continue,
                                    _ => {}
                                }

                                let item_id = item.id;
                                let bg_color = if idx % 2 == 0 {
                                    Color32::from_rgba_unmultiplied(22, 27, 48, 160)
                                } else {
                                    Color32::from_rgba_unmultiplied(18, 22, 38, 160)
                                };

                                egui::Frame::new()
                                    .fill(bg_color)
                                    .stroke(Stroke::new(1.0, Color32::from_rgba_unmultiplied(255, 255, 255, 12)))
                                    .corner_radius(CornerRadius::same(8))
                                    .inner_margin(10.0)
                                    .show(ui, |ui| {
                                        ui.horizontal(|ui| {
                                            ui.label(RichText::new(item.category.icon()).size(16.0));

                                            ui.vertical(|ui| {
                                                ui.label(RichText::new(&item.file_name).strong().size(12.0).color(Color32::WHITE));
                                                let size_str = item.formatted_size();
                                                ui.label(RichText::new(format!("{size_str} • .{}", item.source_ext)).size(10.5).color(Color32::from_rgb(148, 163, 184)));
                                            });

                                            ui.add_space(8.0);
                                            ui.label(RichText::new("→").size(14.0).color(Color32::from_rgb(100, 116, 139)));
                                            ui.add_space(8.0);

                                            let supported = crate::batch_queue::available_target_formats(&item.category);
                                            egui::ComboBox::from_id_salt(format!("target_fmt_cb_{}", item.id))
                                                .selected_text(format!(".{}", item.target_format))
                                                .show_ui(ui, |ui| {
                                                    for fmt in supported {
                                                        ui.selectable_value(&mut item.target_format, fmt.to_string(), format!(".{fmt}"));
                                                    }
                                                });

                                            match &item.status {
                                                ConversionStatus::Pending => {
                                                    ui.label(RichText::new("Ready").size(11.0).color(Color32::from_rgb(125, 165, 245)));
                                                }
                                                ConversionStatus::Converting { progress, stage } => {
                                                    ui.label(RichText::new(format!("⏳ {:.0}% ({stage})", progress * 100.0)).size(11.0).color(Color32::from_rgb(56, 189, 248)));
                                                }
                                                ConversionStatus::Done { out_path } => {
                                                    ui.label(RichText::new("✓ Done").size(11.0).color(Color32::from_rgb(74, 222, 128)).strong());
                                                    if ui.button(RichText::new("📂 Open").size(11.0)).clicked() {
                                                        if let Some(parent) = out_path.parent() {
                                                            to_open_folder = Some(parent.to_path_buf());
                                                        }
                                                    }
                                                }
                                                ConversionStatus::Error { message } => {
                                                    ui.label(RichText::new("❌ Failed").size(11.0).color(Color32::from_rgb(248, 113, 113))).on_hover_text(message);
                                                }
                                            }

                                            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                                if ui.button(RichText::new("❌").size(11.0)).clicked() {
                                                    to_remove_single = Some(item_id);
                                                }
                                                if matches!(item.status, ConversionStatus::Pending | ConversionStatus::Error { .. }) {
                                                    if ui.button(RichText::new("▶ Convert").size(11.0).strong().color(Color32::from_rgb(167, 139, 250))).clicked() {
                                                        to_convert_single = Some(item_id);
                                                    }
                                                }
                                            });
                                        });
                                    });
                                ui.add_space(4.0);
                            }
                        });
                }

                // =============================================================
                // C. FILTERABLE CONVERTER CARDS GRID (CutCraft Bento Style)
                // =============================================================
                ui.add_space(14.0);
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Supported Converters").strong().size(15.0).color(Color32::WHITE));

                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        ui.add(egui::TextEdit::singleline(&mut self.search_query).hint_text("Filter converters (e.g. JPG to PNG, MP4 to MP3)...").desired_width(280.0));
                    });
                });

                // Filter Pills
                ui.horizontal_wrapped(|ui| {
                    ui.spacing_mut().item_spacing = vec2(6.0, 6.0);
                    let tabs = [
                        (LeftSidebarTab::All, "All Converters"),
                        (LeftSidebarTab::Images, "🖼️ Images"),
                        (LeftSidebarTab::Audio, "🎵 Audio & Music"),
                        (LeftSidebarTab::Video, "🎬 Video"),
                        (LeftSidebarTab::Documents, "📄 Documents & PDF"),
                        (LeftSidebarTab::Effects, "✨ Visual Effects"),
                    ];
                    for (tab_val, tab_label) in tabs {
                        let is_sel = self.active_tab == tab_val;
                        let pill_btn = egui::Button::new(
                            RichText::new(tab_label).size(11.5).color(if is_sel { Color32::WHITE } else { Color32::from_rgb(148, 163, 184) })
                        )
                        .fill(if is_sel { Color32::from_rgb(124, 58, 237) } else { Color32::from_rgba_unmultiplied(255, 255, 255, 10) })
                        .stroke(Stroke::new(1.0, if is_sel { Color32::from_rgb(167, 139, 250) } else { Color32::from_rgba_unmultiplied(255, 255, 255, 20) }))
                        .corner_radius(CornerRadius::same(24));

                        if ui.add(pill_btn).clicked() {
                            self.active_tab = tab_val;
                        }
                    }
                });

                ui.add_space(8.0);

                let query = self.search_query.to_lowercase();
                let filtered_cards: Vec<&'static ConverterCard> = CONVERTER_CARDS.iter().filter(|c| {
                    if self.active_tab != LeftSidebarTab::All && self.active_tab != c.cat {
                        return false;
                    }
                    if !query.is_empty() {
                        let text_match = format!("{} to {} {} {}", c.from, c.to, c.desc, c.cat.label()).to_lowercase();
                        if !text_match.contains(&query) {
                            return false;
                        }
                    }
                    true
                }).collect();

                let avail_w = ui.available_width() - 8.0;
                let min_card_w = 230.0f32;
                let num_cols = ((avail_w / (min_card_w + 12.0)).floor() as usize).max(1);
                let actual_card_w = ((avail_w - (num_cols as f32 - 1.0) * 12.0) / num_cols as f32).max(min_card_w);

                for row_chunk in filtered_cards.chunks(num_cols) {
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 12.0;
                        for card in row_chunk {
                            let (card_rect, card_resp) = ui.allocate_exact_size(vec2(actual_card_w, 74.0), egui::Sense::click());
                            let is_hovered = card_resp.hovered();

                            let bg = if is_hovered {
                                Color32::from_rgba_unmultiplied(168, 85, 247, 26)
                            } else {
                                Color32::from_rgba_unmultiplied(255, 255, 255, 8)
                            };
                            let border = if is_hovered {
                                Stroke::new(1.0, Color32::from_rgb(168, 85, 247))
                            } else {
                                Stroke::new(1.0, Color32::from_rgba_unmultiplied(255, 255, 255, 18))
                            };

                            ui.painter().rect(card_rect, CornerRadius::same(12), bg, border, egui::epaint::StrokeKind::Inside);

                            let inner = card_rect.shrink(12.0);
                            let mut card_ui = ui.new_child(egui::UiBuilder::new().max_rect(inner));
                            card_ui.vertical(|ui| {
                                ui.horizontal(|ui| {
                                    let badge_bg = card.color.linear_multiply(0.2);
                                    let badge_rect = Rect::from_min_size(ui.cursor().min, vec2(100.0, 20.0));
                                    ui.painter().rect(badge_rect, CornerRadius::same(6), badge_bg, Stroke::new(1.0, card.color.linear_multiply(0.6)), egui::epaint::StrokeKind::Inside);

                                    ui.add_space(4.0);
                                    ui.label(RichText::new(format!("{} → {}", card.from, card.to)).size(12.0).strong().color(Color32::WHITE));
                                });

                                ui.add_space(4.0);
                                ui.label(RichText::new(card.desc).size(11.5).color(Color32::from_rgb(148, 163, 184)));
                            });

                            if card_resp.clicked() {
                                card_to_launch = Some(*card);
                            }
                        }
                    });
                    ui.add_space(6.0);
                }

                ui.add_space(20.0);
            });

            if let Some(card) = card_to_launch {
                let mut picker = rfd::FileDialog::new();
                if !card.exts.is_empty() {
                    picker = picker.add_filter(format!("{} Files", card.from), card.exts);
                }
                if let Some(files) = picker.pick_files() {
                    let count = self.queue.add_paths(files);
                    let target_fmt = card.to.to_lowercase();
                    self.batch_target_format = target_fmt.clone();
                    for item in self.queue.items.iter_mut().rev().take(count) {
                        item.target_format = target_fmt.clone();
                    }
                    self.status_msg = format!("Added {count} files set to .{}", target_fmt);
                }
            }

            if let Some(id) = to_convert_single {
                self.start_single_conversion(id);
            }
            if let Some(id) = to_remove_single {
                self.queue.remove_item(id);
            }
            if let Some(folder) = to_open_folder {
                let _ = std::process::Command::new("explorer").arg(&folder).spawn();
            }
        });
    }

    fn handle_external_control(&mut self) {
        if let Some(ref rx) = self.control_rx {
            let mut requests = Vec::new();
            while let Ok(req) = rx.try_recv() {
                requests.push(req);
            }
            for req in requests {
                let res = self.execute_command(&req.method, &req.params);
                let val = match res {
                    Ok(v) => json!({"ok": true, "result": v}),
                    Err(e) => json!({"ok": false, "error": e}),
                };
                let _ = req.reply.send(val);
            }
        }
    }

    fn handle_ai_tool_requests(&mut self) {
        self.agent.process_events();
        let requests = self.agent.take_tool_requests();
        for req in requests {
            let res = self.execute_command(&req.tool_name, &req.params);
            let val = match res {
                Ok(v) => json!({"ok": true, "result": v}),
                Err(e) => json!({"ok": false, "error": e}),
            };
            let _ = req.reply_sender.send(Ok(val));
        }
    }

    fn poll_conversion_worker(&mut self, ctx: &egui::Context) {
        let events = self.converter.poll_events();
        let mut got_update = false;
        for ev in events {
            got_update = true;
            match ev {
                ConversionEvent::Progress { item_id, progress, stage } => {
                    self.queue.set_item_status(item_id, ConversionStatus::Converting { progress, stage });
                }
                ConversionEvent::Done { item_id, out_path } => {
                    self.status_msg = format!("Done: {}", out_path.file_name().and_then(|n| n.to_str()).unwrap_or("file"));
                    self.queue.set_item_status(item_id, ConversionStatus::Done { out_path });
                }
                ConversionEvent::Error { item_id, message } => {
                    self.status_msg = format!("Conversion failed: {message}");
                    self.queue.set_item_status(item_id, ConversionStatus::Error { message });
                }
            }
        }
        let is_converting = self.queue.items.iter().any(|i| matches!(i.status, ConversionStatus::Converting { .. }));
        if got_update || is_converting {
            ctx.request_repaint_after(Duration::from_millis(50));
        }
    }

    fn handle_drag_and_drop(&mut self, ctx: &egui::Context) {
        let dropped = ctx.input(|i| i.raw.dropped_files.clone());
        if !dropped.is_empty() {
            let pbs: Vec<PathBuf> = dropped.into_iter().map(|f| f.path().to_path_buf()).collect();
            let count = self.queue.add_paths(pbs);
            if count > 0 {
                self.status_msg = format!("Added {count} files via Drag & Drop");
            }
        }
    }
}

impl eframe::App for ConvCraftApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // Theme styling
        apply_theme(ui.ctx(), self.theme);

        // Background loops
        self.handle_external_control();
        self.handle_ai_tool_requests();
        self.poll_conversion_worker(ui.ctx());
        self.handle_drag_and_drop(ui.ctx());

        // Top Bar & Footer
        self.render_top_bar(ui);
        self.render_footer(ui);

        // Left Tool Sidebar
        self.render_left_sidebar(ui);

        // Right AI Agent Sidebar
        if self.agent.is_open {
            let context_str = format!(
                "ConvCraft Queue Summary:\nTotal Files: {}\nReady: {}\nConverting: {}\nDone: {}\nOutput Dir: {}\n\nFiles:\n{}",
                self.queue.items.len(),
                self.queue.items.iter().filter(|i| matches!(i.status, ConversionStatus::Pending)).count(),
                self.queue.items.iter().filter(|i| matches!(i.status, ConversionStatus::Converting { .. })).count(),
                self.queue.items.iter().filter(|i| matches!(i.status, ConversionStatus::Done { .. })).count(),
                self.drive.output_dir().display(),
                self.queue.items.iter().take(20).map(|i| format!("- {} (.{}) -> .{} [{:?}]", i.file_name, i.source_ext, i.target_format, i.category)).collect::<Vec<_>>().join("\n")
            );
            let drive_path = self.drive.output_dir().display().to_string();
            self.agent.render_sidebar(ui, &context_str, &drive_path);
        }

        // Central Conversion Workbench
        self.render_central_workbench(ui);
    }
}

/// Registers the platform symbol/emoji fonts as fallbacks for both text families, so UI icons
/// (🦎 ⚡ ✓ → ▾ …) render on every operating system instead of tofu boxes (▯).
fn install_symbol_fonts(cc: &eframe::CreationContext) {
    let mut fonts = egui::FontDefinitions::default();
    let add_fallback = |fonts: &mut egui::FontDefinitions, name: &str, path: &str| {
        if let Ok(bytes) = std::fs::read(path) {
            fonts
                .font_data
                .insert(name.to_owned(), std::sync::Arc::new(egui::FontData::from_owned(bytes)));
            for family in [egui::FontFamily::Proportional, egui::FontFamily::Monospace] {
                fonts.families.entry(family).or_default().push(name.to_owned());
            }
        }
    };
    #[cfg(target_os = "windows")]
    {
        add_fallback(&mut fonts, "SegoeUIEmoji", r"C:\Windows\Fonts\seguiemj.ttf");
        add_fallback(&mut fonts, "SegoeUISymbol", r"C:\Windows\Fonts\seguisym.ttf");
    }
    #[cfg(target_os = "macos")]
    {
        add_fallback(&mut fonts, "AppleColorEmoji", "/System/Library/Fonts/Apple Color Emoji.ttf");
    }
    #[cfg(target_os = "linux")]
    {
        for path in [
            "/usr/share/fonts/truetype/noto/NotoColorEmoji.ttf",
            "/usr/share/fonts/NotoColorEmoji.ttf",
        ] {
            add_fallback(&mut fonts, "NotoColorEmoji", path);
        }
    }
    cc.egui_ctx.set_fonts(fonts);
}

fn main() -> eframe::Result {
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|a| a == "--mcp" || a == "mcp") {
        let (tx, _rx) = mpsc::channel();
        let _server = std::thread::spawn(move || {
            crate::control::run_stdio_mcp_server(tx);
        });
    }

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 800.0])
            .with_min_inner_size([900.0, 600.0])
            .with_title("ConvCraft - Universal Media & File Converter")
            .with_drag_and_drop(true)
            .with_icon(egui::IconData {
                rgba: include_bytes!("../assets/app_icon.rgba").to_vec(),
                width: 256,
                height: 256,
            }),
        ..Default::default()
    };

    eframe::run_native(
        "ConvCraft",
        options,
        Box::new(|cc| Ok(Box::new(ConvCraftApp::new(cc)))),
    )
}
