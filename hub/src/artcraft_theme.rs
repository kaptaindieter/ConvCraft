//! Craft Suite Global Shared Theme & Header/Footer Action Components for ConvCraft
//!
//! Synchronizes Aurora, Dark, and Light themes plus uniform Top-Right Action Bar
//! and Bottom-Right Theme Switcher across Craft Suite apps.

use eframe::egui::{self, Color32, CornerRadius, FontId, Pos2, Rect, Stroke, Vec2, pos2, vec2};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum AppTheme {
    #[default]
    Aurora,
    Dark,
    Light,
}

impl AppTheme {
    pub const ALL: [AppTheme; 3] = [AppTheme::Aurora, AppTheme::Dark, AppTheme::Light];

    pub fn code(&self) -> &'static str {
        match self {
            AppTheme::Aurora => "aurora",
            AppTheme::Dark => "dark",
            AppTheme::Light => "light",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            AppTheme::Aurora => "✨ Aurora",
            AppTheme::Dark => "🌙 Dark",
            AppTheme::Light => "☀️ Light",
        }
    }

    pub fn from_code(s: &str) -> Option<Self> {
        match s.trim().to_lowercase().as_str() {
            "aurora" | "prism" => Some(AppTheme::Aurora),
            "dark" => Some(AppTheme::Dark),
            "light" => Some(AppTheme::Light),
            _ => None,
        }
    }

    pub fn load_global() -> Self {
        for p in shared_theme_paths() {
            if p.exists() {
                if let Ok(data) = std::fs::read_to_string(&p) {
                    if let Ok(cfg) = serde_json::from_str::<ThemeConfig>(&data) {
                        return cfg.theme;
                    }
                }
            }
        }
        AppTheme::Aurora
    }

    pub fn save_global(theme: AppTheme) {
        let cfg = ThemeConfig { theme };
        if let Ok(json) = serde_json::to_string_pretty(&cfg) {
            for p in shared_theme_paths() {
                if let Some(parent) = p.parent() {
                    let _ = std::fs::create_dir_all(parent);
                }
                let _ = std::fs::write(&p, &json);
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
struct ThemeConfig {
    theme: AppTheme,
}

pub fn shared_theme_paths() -> Vec<std::path::PathBuf> {
    let mut paths = Vec::new();
    if let Some(prof) = std::env::var_os("USERPROFILE").or_else(|| std::env::var_os("HOME")) {
        let base = std::path::PathBuf::from(prof);
        let craft_dir = base.join(".craft-suite");
        let _ = std::fs::create_dir_all(&craft_dir);
        paths.push(craft_dir.join("artcraft_theme.json"));

        let dir = base.join(".artcraft");
        let _ = std::fs::create_dir_all(&dir);
        paths.push(dir.join("artcraft_theme.json"));
    }
    paths
}

// Spectacular Gemini Rainbow Aurora Color Scheme Constants
pub const GEMINI_CYAN: Color32 = Color32::from_rgb(56, 189, 248);     // #38bdf8
pub const GEMINI_BLUE: Color32 = Color32::from_rgb(66, 133, 244);     // #4285f4
pub const GEMINI_PURPLE: Color32 = Color32::from_rgb(168, 85, 247);   // #a855f7
pub const GEMINI_VIOLET: Color32 = Color32::from_rgb(192, 132, 252);  // #c084fc
pub const GEMINI_MAGENTA: Color32 = Color32::from_rgb(236, 72, 153);  // #ec4899
pub const GEMINI_AMBER: Color32 = Color32::from_rgb(251, 146, 60);    // #fb923c

pub const AURORA_BG: Color32 = Color32::from_rgb(9, 11, 20);         // #090b14 Deep cosmic midnight-indigo         // Deep cosmic nebula space
pub const AURORA_PANEL: Color32 = Color32::from_rgb(15, 19, 34);      // #0f1322 Dark nebula-navy panel      // Twilight indigo panel
pub const AURORA_CARD: Color32 = Color32::from_rgb(22, 27, 48);       // #161b30 Iridescent indigo-slate card       // Dark iridescent card fill
pub const AURORA_BORDER: Color32 = Color32::from_rgba_premultiplied(31, 20, 54, 56); // rgba(139, 92, 246, 0.22) Specular violet     // Subtle cosmic border
pub const AURORA_TEXT: Color32 = Color32::from_rgb(241, 243, 249);    // #f1f3f9 Crisp starlight white    // Crisp starlight white
pub const AURORA_TEXT_MUTED: Color32 = Color32::from_rgb(148, 163, 184);// #94a3b8 Muted stardust slate// Muted stardust blue
pub const AURORA_ACCENT: Color32 = Color32::from_rgb(139, 92, 246);   // Violet-indigo accent

/// Renders a horizontal Gemini iridescent rainbow gradient line [Cyan -> Blue -> Purple -> Magenta -> Amber]
pub fn paint_gemini_rainbow_line(painter: &egui::Painter, rect: egui::Rect) {
    let w = rect.width();
    let segments = [
        (0.0, 0.20, GEMINI_CYAN, GEMINI_BLUE),
        (0.20, 0.45, GEMINI_BLUE, GEMINI_PURPLE),
        (0.45, 0.70, GEMINI_PURPLE, GEMINI_MAGENTA),
        (0.70, 1.0, GEMINI_MAGENTA, GEMINI_AMBER),
    ];
    for (f_start, f_end, c1, c2) in segments {
        let seg_rect = egui::Rect::from_min_max(
            pos2(rect.min.x + w * f_start, rect.min.y),
            pos2(rect.min.x + w * f_end, rect.max.y),
        );
        let mut mesh = egui::Mesh::default();
        mesh.colored_vertex(seg_rect.left_top(), c1);
        mesh.colored_vertex(seg_rect.right_top(), c2);
        mesh.colored_vertex(seg_rect.right_bottom(), c2);
        mesh.colored_vertex(seg_rect.left_bottom(), c1);
        mesh.add_triangle(0, 1, 2);
        mesh.add_triangle(0, 2, 3);
        painter.add(egui::Shape::mesh(mesh));
    }
}

/// Applies theme visuals to egui context
pub fn apply_theme(ctx: &egui::Context, theme: AppTheme) {
    let mut visuals = match theme {
        AppTheme::Aurora => {
            let mut v = egui::Visuals::dark();
            v.panel_fill = AURORA_BG;
            v.window_fill = AURORA_PANEL;
            v.faint_bg_color = AURORA_CARD;
            v.extreme_bg_color = Color32::from_rgb(9, 11, 20);
            v.widgets.noninteractive.bg_fill = AURORA_PANEL;
            v.widgets.noninteractive.bg_stroke = Stroke::new(1.0, AURORA_BORDER);
            v.widgets.noninteractive.fg_stroke = Stroke::new(1.0, AURORA_TEXT);
            v.widgets.inactive.bg_fill = AURORA_CARD;
            v.widgets.inactive.bg_stroke = Stroke::new(1.0, AURORA_BORDER);
            v.widgets.inactive.fg_stroke = Stroke::new(1.0, AURORA_TEXT);
            v.widgets.hovered.bg_fill = Color32::from_rgb(30, 36, 64);
            v.widgets.hovered.bg_stroke = Stroke::new(1.0, GEMINI_PURPLE);
            v.widgets.hovered.fg_stroke = Stroke::new(1.0, Color32::WHITE);
            v.widgets.active.bg_fill = Color32::from_rgb(40, 48, 86);
            v.widgets.active.bg_stroke = Stroke::new(1.2, GEMINI_VIOLET);
            v.selection.bg_fill = Color32::from_rgba_unmultiplied(168, 85, 247, 70);
            v.selection.stroke = Stroke::new(1.0, GEMINI_VIOLET);
            v
        }
        AppTheme::Dark => {
            let mut v = egui::Visuals::dark();
            v.panel_fill = Color32::from_rgb(18, 18, 20);
            v.window_fill = Color32::from_rgb(24, 24, 27);
            v.faint_bg_color = Color32::from_rgb(32, 32, 36);
            v.extreme_bg_color = Color32::from_rgb(18, 18, 20);
            v.widgets.noninteractive.bg_fill = Color32::from_rgb(24, 24, 27);
            v.widgets.noninteractive.bg_stroke = Stroke::new(1.0, Color32::from_rgb(39, 39, 42));
            v.widgets.noninteractive.fg_stroke = Stroke::new(1.0, Color32::from_rgb(237, 237, 237));
            v.widgets.inactive.bg_fill = Color32::from_rgb(32, 32, 36);
            v.widgets.inactive.bg_stroke = Stroke::new(1.0, Color32::from_rgb(39, 39, 42));
            v.widgets.inactive.fg_stroke = Stroke::new(1.0, Color32::from_rgb(237, 237, 237));
            v.widgets.hovered.bg_fill = Color32::from_rgb(39, 39, 42);
            v.widgets.hovered.bg_stroke = Stroke::new(1.0, Color32::from_rgb(63, 63, 70));
            v.widgets.hovered.fg_stroke = Stroke::new(1.0, Color32::WHITE);
            v.widgets.active.bg_fill = Color32::from_rgb(50, 50, 56);
            v.widgets.active.bg_stroke = Stroke::new(1.0, Color32::from_rgb(75, 75, 82));
            v
        }
        AppTheme::Light => egui::Visuals::light(),
    };
    visuals.window_corner_radius = CornerRadius::same(8);
    ctx.set_visuals(visuals);
}

/// Universal uniform theme switcher dropdown for 28px Footers
pub fn render_theme_switcher_dropdown(ui: &mut egui::Ui, current_theme: &mut AppTheme) -> bool {
    let mut changed = false;
    let button_text = format!("{} ▾", current_theme.label());

    egui::ComboBox::from_id_salt("convcraft_universal_footer_theme_dropdown")
        .selected_text(egui::RichText::new(&button_text).size(11.5))
        .width(115.0)
        .show_ui(ui, |ui| {
            for theme in &[AppTheme::Aurora, AppTheme::Dark, AppTheme::Light] {
                let is_selected = *current_theme == *theme;
                if ui.selectable_label(is_selected, egui::RichText::new(theme.label()).size(12.0)).clicked() {
                    if *current_theme != *theme {
                        *current_theme = *theme;
                        AppTheme::save_global(*theme);
                        changed = true;
                    }
                }
            }
        });
    changed
}

/// Universal uniform AI Agent Pill Button ("✨ AI Agent", height 28px, border-radius: 9999px)
pub fn render_ai_agent_button(ui: &mut egui::Ui, is_open: bool) -> egui::Response {
    let desired_size = vec2(106.0, 28.0);
    let (rect, resp) = ui.allocate_exact_size(desired_size, egui::Sense::click());
    let hovered = resp.hovered();
    let is_dark = ui.visuals().dark_mode;
    let painter = ui.painter();
    let radius = 14.0;

    if is_open {
        let bg_fill = if hovered {
            Color32::from_rgb(109, 72, 235)
        } else {
            Color32::from_rgb(94, 60, 218)
        };
        painter.rect_filled(rect, radius, bg_fill);
        painter.rect_stroke(
            rect,
            radius,
            Stroke::new(1.2, Color32::from_rgb(185, 145, 255)),
            egui::StrokeKind::Inside,
        );
    } else {
        let (bg_fill, stroke_col) = if is_dark {
            let fill = if hovered {
                Color32::from_rgba_unmultiplied(255, 255, 255, 26)
            } else {
                Color32::from_rgba_unmultiplied(255, 255, 255, 14)
            };
            let stroke = if hovered {
                Color32::from_rgba_unmultiplied(168, 85, 247, 140)
            } else {
                Color32::from_rgba_unmultiplied(255, 255, 255, 36)
            };
            (fill, stroke)
        } else {
            let fill = if hovered {
                Color32::from_rgba_unmultiplied(15, 23, 42, 22)
            } else {
                Color32::from_rgba_unmultiplied(15, 23, 42, 10)
            };
            let stroke = if hovered {
                Color32::from_rgb(124, 58, 237)
            } else {
                Color32::from_rgba_unmultiplied(15, 23, 42, 42)
            };
            (fill, stroke)
        };
        painter.rect_filled(rect, radius, bg_fill);
        painter.rect_stroke(rect, radius, Stroke::new(1.0, stroke_col), egui::StrokeKind::Inside);
    }

    let text_col = if is_open {
        Color32::WHITE
    } else if is_dark {
        Color32::from_rgb(230, 232, 245)
    } else {
        Color32::from_rgb(30, 41, 59)
    };

    painter.text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        "✨ AI Agent",
        FontId::proportional(12.5),
        text_col,
    );

    resp.on_hover_text("Toggle ✨ AI Agent Sidebar")
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CraftSuiteCloudConfig {
    pub provider: String,
    pub sync_path: String,
    pub auto_save: bool,
}

impl Default for CraftSuiteCloudConfig {
    fn default() -> Self {
        let (provider, sync_path) = if std::path::Path::new(r"G:").exists() {
            ("Google Drive".to_string(), r"G:".to_string())
        } else if let Some(user_profile) = std::env::var_os("USERPROFILE") {
            let docs = std::path::PathBuf::from(user_profile).join("Documents");
            ("Local Folder".to_string(), docs.to_string_lossy().to_string())
        } else {
            ("Local Folder".to_string(), r"C:\CraftSuite_Sync".to_string())
        };
        Self {
            provider,
            sync_path,
            auto_save: true,
        }
    }
}

impl CraftSuiteCloudConfig {
    pub fn config_file() -> Option<std::path::PathBuf> {
        if let Some(user_profile) = std::env::var_os("USERPROFILE").or_else(|| std::env::var_os("HOME")) {
            Some(std::path::PathBuf::from(user_profile).join(".craft-suite").join("cloud-config.json"))
        } else {
            None
        }
    }

    pub fn load() -> Self {
        if let Some(p) = Self::config_file() {
            if p.exists() {
                if let Ok(d) = std::fs::read_to_string(&p) {
                    if let Ok(cfg) = serde_json::from_str::<CraftSuiteCloudConfig>(&d) {
                        return cfg;
                    }
                }
            }
        }
        Self::default()
    }

    pub fn save(&self) {
        if let Some(p) = Self::config_file() {
            if let Some(parent) = p.parent() {
                let _ = std::fs::create_dir_all(parent);
            }
            if let Ok(d) = serde_json::to_string_pretty(self) {
                let _ = std::fs::write(&p, d);
            }
        }
    }
}

pub fn render_cloud_sync_modal(
    ctx: &egui::Context,
    open: &mut bool,
    config: &mut CraftSuiteCloudConfig,
    proj_dir: Option<&std::path::Path>,
) {
    if !*open {
        return;
    }

    let mut should_close = false;
    let mut should_save = false;

    egui::Window::new("☁️ Cloud & Auto-Save Sync")
        .open(open)
        .collapsible(false)
        .resizable(false)
        .default_width(440.0)
        .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
        .show(ctx, |ui| {
            ui.add_space(4.0);
            ui.label(egui::RichText::new("Configure your cloud drive provider or local synchronization folder.").size(12.0).color(Color32::from_rgb(160, 175, 205)));
            ui.add_space(10.0);

            ui.label(egui::RichText::new("Cloud Provider:").strong().size(12.5));
            egui::ComboBox::from_id_salt("craft_cloud_provider_dropdown")
                .selected_text(&config.provider)
                .width(ui.available_width())
                .show_ui(ui, |ui| {
                    for p in &["Google Drive", "OneDrive", "Nextcloud", "Dropbox", "Local Folder"] {
                        ui.selectable_value(&mut config.provider, p.to_string(), *p);
                    }
                });

            ui.add_space(10.0);
            ui.label(egui::RichText::new("Sync Path / Drive Root:").strong().size(12.5));
            ui.horizontal(|ui| {
                ui.add(egui::TextEdit::singleline(&mut config.sync_path).desired_width(ui.available_width() - 130.0));
                if ui.button("📂 Choose Folder").clicked() {
                    if let Some(folder) = rfd::FileDialog::new().pick_folder() {
                        config.sync_path = folder.to_string_lossy().to_string();
                    }
                }
            });

            ui.add_space(12.0);
            ui.checkbox(&mut config.auto_save, "Enable automatic background project saving");

            if let Some(pd) = proj_dir {
                ui.add_space(8.0);
                ui.separator();
                ui.add_space(6.0);
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Active Directory:").size(11.5).color(Color32::GRAY));
                    ui.label(egui::RichText::new(pd.to_string_lossy()).size(11.0).monospace());
                });
                if ui.button("📁 Open in Windows Explorer").clicked() {
                    let _ = std::fs::create_dir_all(pd);
                    let _ = std::process::Command::new("explorer").arg(pd).spawn();
                }
            }

            ui.add_space(14.0);
            ui.separator();
            ui.add_space(8.0);

            ui.horizontal(|ui| {
                if ui.button(egui::RichText::new("Save & Apply").strong().color(Color32::WHITE)).clicked() {
                    should_save = true;
                    should_close = true;
                }
                if ui.button("Cancel").clicked() {
                    should_close = true;
                }
            });
        });

    if should_save {
        config.save();
    }
    if should_close {
        *open = false;
    }
}
