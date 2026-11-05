//! Google Drive detection and file export management for ConvCraft.

use std::path::{Path, PathBuf};

#[derive(Clone, Debug)]
pub struct ArtCraftDrive {
    pub app_name: &'static str,
    pub detected_drive_root: PathBuf,
}

impl ArtCraftDrive {
    pub fn new() -> Self {
        let detected = Self::detect_google_drive();
        Self {
            app_name: "ConvCraft",
            detected_drive_root: detected,
        }
    }

    pub fn user_configured_sync_root() -> Option<(String, PathBuf)> {
        let home = std::env::var_os("USERPROFILE").or_else(|| std::env::var_os("HOME"))?;
        let cfg_path = PathBuf::from(home).join(".craft-suite").join("cloud-config.json");
        let raw = std::fs::read_to_string(cfg_path).ok()?;
        let val: serde_json::Value = serde_json::from_str(&raw).ok()?;
        let sync_path = val.get("sync_path")?.as_str()?.trim();
        let provider = val
            .get("provider")
            .and_then(|p| p.as_str())
            .unwrap_or("Cloud Sync")
            .trim()
            .to_string();
        if sync_path.is_empty() {
            None
        } else {
            let root = PathBuf::from(sync_path).join("Art Craft").join("ConvCraft");
            Some((provider, root))
        }
    }

    pub fn detect_google_drive() -> PathBuf {
        if let Some((_, custom)) = Self::user_configured_sync_root() {
            let _ = std::fs::create_dir_all(&custom);
            return custom;
        }

        let candidates = [
            r"G:\My Drive\Art Craft\ConvCraft",
            r"G:\Meine Ablage\Art Craft\ConvCraft",
            r"G:\Art Craft\ConvCraft",
            r"G:\My Drive\ConvCraft",
            r"G:\ConvCraft",
        ];

        for c in candidates {
            let p = PathBuf::from(c);
            if p.exists() || Path::new(r"G:\").exists() {
                let _ = std::fs::create_dir_all(&p);
                return p;
            }
        }

        // Fallback to Documents/Art Craft/ConvCraft
        if let Some(user_profile) = std::env::var_os("USERPROFILE").or_else(|| std::env::var_os("HOME")) {
            let p = PathBuf::from(user_profile).join("Documents").join("Art Craft").join("ConvCraft");
            let _ = std::fs::create_dir_all(&p);
            return p;
        }

        let fallback = PathBuf::from("ConvCraft_Output");
        let _ = std::fs::create_dir_all(&fallback);
        fallback
    }

    pub fn is_google_drive(&self) -> bool {
        Self::user_configured_sync_root().is_some()
            || self.detected_drive_root.starts_with("G:")
            || Path::new(r"G:\").exists()
            || self.output_dir().exists()
    }

    pub fn status_label(&self) -> String {
        if let Some((provider, _)) = Self::user_configured_sync_root() {
            format!("{provider}: Active")
        } else if self.detected_drive_root.starts_with("G:") || Path::new(r"G:\").exists() {
            "Google Drive: Active".to_string()
        } else {
            "Local Sync: Active".to_string()
        }
    }

    pub fn output_dir(&self) -> PathBuf {
        if let Some((_, custom)) = Self::user_configured_sync_root() {
            let _ = std::fs::create_dir_all(&custom);
            return custom;
        }
        let _ = std::fs::create_dir_all(&self.detected_drive_root);
        self.detected_drive_root.clone()
    }
}
