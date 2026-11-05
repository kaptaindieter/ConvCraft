//! Batch Queue Data Models and State for ConvCraft.

use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum FileCategory {
    Images,
    Audio,
    Video,
    Documents,
}

impl FileCategory {
    pub fn label(&self) -> &'static str {
        match self {
            FileCategory::Images => "🖼️ Images & Graphics",
            FileCategory::Audio => "🎵 Audio & Music",
            FileCategory::Video => "🎬 Video & Animation",
            FileCategory::Documents => "📄 Documents & Effects",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            FileCategory::Images => "🖼️",
            FileCategory::Audio => "🎵",
            FileCategory::Video => "🎬",
            FileCategory::Documents => "📄",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "image" | "images" | "img" => Some(Self::Images),
            "audio" | "sound" | "music" => Some(Self::Audio),
            "video" | "movie" | "vid" => Some(Self::Video),
            "doc" | "docs" | "document" | "documents" => Some(Self::Documents),
            _ => None,
        }
    }

    pub fn from_ext(ext: &str) -> Self {
        let e = ext.to_ascii_lowercase();
        match e.as_str() {
            "jpg" | "jpeg" | "png" | "webp" | "avif" | "svg" | "ico" | "gif" | "tiff" | "tif" | "bmp" | "tga" | "psd" => {
                FileCategory::Images
            }
            "mp3" | "wav" | "flac" | "aac" | "ogg" | "m4a" | "opus" | "wma" | "aiff" => {
                FileCategory::Audio
            }
            "mp4" | "mov" | "mkv" | "webm" | "avi" | "flv" | "wmv" | "m4v" => {
                FileCategory::Video
            }
            _ => FileCategory::Documents,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConversionSettings {
    pub quality: u32,
    pub audio_bitrate: String,
    pub video_resolution: String,
    pub video_fps: Option<u32>,
    pub video_crf: Option<u32>,
    pub resize_width: Option<u32>,
    pub resize_height: Option<u32>,
    pub keep_aspect: bool,
    pub effect: Option<String>,
}

impl Default for ConversionSettings {
    fn default() -> Self {
        Self {
            quality: 90,
            audio_bitrate: "320k".to_string(),
            video_resolution: "original".to_string(),
            video_fps: None,
            video_crf: Some(23),
            resize_width: None,
            resize_height: None,
            keep_aspect: true,
            effect: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ConversionStatus {
    Pending,
    Converting { progress: f32, stage: String },
    Done { out_path: PathBuf },
    Error { message: String },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConversionItem {
    pub id: usize,
    pub source_path: PathBuf,
    pub file_name: String,
    pub file_size: u64,
    pub category: FileCategory,
    pub source_ext: String,
    pub target_format: String,
    pub settings: ConversionSettings,
    pub status: ConversionStatus,
    pub selected: bool,
}

impl ConversionItem {
    pub fn new(id: usize, path: PathBuf) -> Self {
        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("file").to_string();
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("").to_ascii_lowercase();
        let cat = FileCategory::from_ext(&ext);
        let size = std::fs::metadata(&path).map(|m| m.len()).unwrap_or(0);
        let target = default_target_for(&cat, &ext);

        Self {
            id,
            source_path: path,
            file_name: name,
            file_size: size,
            category: cat,
            source_ext: ext,
            target_format: target,
            settings: ConversionSettings::default(),
            status: ConversionStatus::Pending,
            selected: false,
        }
    }

    pub fn formatted_size(&self) -> String {
        let b = self.file_size as f64;
        if b < 1024.0 {
            format!("{b:.0} B")
        } else if b < 1024.0 * 1024.0 {
            format!("{:.1} KB", b / 1024.0)
        } else if b < 1024.0 * 1024.0 * 1024.0 {
            format!("{:.1} MB", b / (1024.0 * 1024.0))
        } else {
            format!("{:.2} GB", b / (1024.0 * 1024.0 * 1024.0))
        }
    }
}

pub fn default_target_for(cat: &FileCategory, ext: &str) -> String {
    match cat {
        FileCategory::Images => {
            if ext == "webp" { "png".to_string() } else { "webp".to_string() }
        }
        FileCategory::Audio => {
            if ext == "mp3" { "wav".to_string() } else { "mp3".to_string() }
        }
        FileCategory::Video => {
            if ext == "mp4" { "webm".to_string() } else { "mp4".to_string() }
        }
        FileCategory::Documents => "pdf".to_string(),
    }
}

pub fn available_target_formats(cat: &FileCategory) -> &'static [&'static str] {
    match cat {
        FileCategory::Images => &["webp", "png", "jpg", "avif", "gif", "ico", "bmp", "tif", "pdf"],
        FileCategory::Audio => &["mp3", "wav", "flac", "aac", "ogg", "m4a", "opus"],
        FileCategory::Video => &["mp4", "mov", "webm", "mkv", "avi", "gif", "mp3", "wav"],
        FileCategory::Documents => &["pdf", "txt", "png", "jpg"],
    }
}

#[derive(Default)]
pub struct BatchQueueState {
    pub items: Vec<ConversionItem>,
    pub next_id: usize,
    pub active_category: Option<FileCategory>,
    pub search_query: String,
    pub undo_history: Vec<Vec<ConversionItem>>,
    pub redo_history: Vec<Vec<ConversionItem>>,
}

impl BatchQueueState {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            next_id: 1,
            active_category: None,
            search_query: String::new(),
            undo_history: Vec::new(),
            redo_history: Vec::new(),
        }
    }

    pub fn snapshot(&mut self) {
        if self.undo_history.len() > 30 {
            self.undo_history.remove(0);
        }
        self.undo_history.push(self.items.clone());
        self.redo_history.clear();
    }

    pub fn undo(&mut self) -> bool {
        if let Some(prev) = self.undo_history.pop() {
            self.redo_history.push(self.items.clone());
            self.items = prev;
            true
        } else {
            false
        }
    }

    pub fn redo(&mut self) -> bool {
        if let Some(next) = self.redo_history.pop() {
            self.undo_history.push(self.items.clone());
            self.items = next;
            true
        } else {
            false
        }
    }

    pub fn add_paths(&mut self, paths: Vec<PathBuf>) -> usize {
        let mut added = 0;
        self.snapshot();
        for p in paths {
            if p.is_file() {
                let id = self.next_id;
                self.next_id += 1;
                self.items.push(ConversionItem::new(id, p));
                added += 1;
            } else if p.is_dir() {
                for entry in walkdir::WalkDir::new(&p).max_depth(3).into_iter().filter_map(|e| e.ok()) {
                    if entry.file_type().is_file() {
                        let id = self.next_id;
                        self.next_id += 1;
                        self.items.push(ConversionItem::new(id, entry.into_path()));
                        added += 1;
                    }
                }
            }
        }
        added
    }

    pub fn clear(&mut self) {
        self.snapshot();
        self.items.clear();
    }

    pub fn remove_item(&mut self, id: usize) {
        self.snapshot();
        self.items.retain(|i| i.id != id);
    }

    pub fn set_all_target_format(&mut self, format: &str) {
        self.snapshot();
        for item in &mut self.items {
            item.target_format = format.to_string();
        }
    }

    pub fn can_undo(&self) -> bool {
        !self.undo_history.is_empty()
    }

    pub fn can_redo(&self) -> bool {
        !self.redo_history.is_empty()
    }

    pub fn add_path(&mut self, p: &Path) -> usize {
        self.add_paths(vec![p.to_path_buf()])
    }

    pub fn set_target_format_all(&mut self, format: &str, category: Option<FileCategory>) -> usize {
        self.snapshot();
        let mut count = 0;
        for item in &mut self.items {
            if category.is_none() || category == Some(item.category) {
                item.target_format = format.to_string();
                count += 1;
            }
        }
        count
    }

    pub fn update_settings_all<F: Fn(&mut ConversionSettings)>(&mut self, f: F) -> usize {
        self.snapshot();
        let count = self.items.len();
        for item in &mut self.items {
            f(&mut item.settings);
        }
        count
    }

    pub fn set_item_status(&mut self, id: usize, status: ConversionStatus) {
        if let Some(item) = self.items.iter_mut().find(|i| i.id == id) {
            item.status = status;
        }
    }
}
