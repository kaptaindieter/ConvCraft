//! Universal Media Converter Engine for ConvCraft.
//! Handles Images, Audio, Video, and Documents via native image decoding and ffmpeg.

use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use crate::batch_queue::{ConversionItem, FileCategory};

pub enum ConversionEvent {
    Progress { item_id: usize, progress: f32, stage: String },
    Done { item_id: usize, out_path: PathBuf },
    Error { item_id: usize, message: String },
}

pub struct ConverterEngine {
    tx: Sender<ConversionEvent>,
    rx: Receiver<ConversionEvent>,
    pub is_converting: bool,
    cancel_flag: Arc<AtomicBool>,
}

impl ConverterEngine {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        Self {
            tx,
            rx,
            is_converting: false,
            cancel_flag: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn poll_events(&self) -> Vec<ConversionEvent> {
        let mut events = Vec::new();
        while let Ok(ev) = self.rx.try_recv() {
            events.push(ev);
        }
        events
    }

    pub fn cancel(&mut self) {
        self.cancel_flag.store(true, Ordering::SeqCst);
        self.is_converting = false;
    }

    pub fn start_batch(&mut self, items: Vec<ConversionItem>, out_dir: PathBuf) {
        if self.is_converting || items.is_empty() {
            return;
        }

        self.is_converting = true;
        self.cancel_flag = Arc::new(AtomicBool::new(false));
        let cancel = Arc::clone(&self.cancel_flag);
        let tx = self.tx.clone();

        std::thread::spawn(move || {
            let _ = std::fs::create_dir_all(&out_dir);

            for item in items.iter() {
                if cancel.load(Ordering::SeqCst) {
                    break;
                }

                let _ = tx.send(ConversionEvent::Progress {
                    item_id: item.id,
                    progress: 0.1,
                    stage: "Starting conversion...".to_string(),
                });

                let file_stem = item.source_path.file_stem().and_then(|s| s.to_str()).unwrap_or("output");
                let target_ext = item.target_format.trim().trim_start_matches('.').to_ascii_lowercase();
                let out_file_name = format!("{}_{}.{}", file_stem, "conv", target_ext);
                let out_path = out_dir.join(&out_file_name);

                let res = match item.category {
                    FileCategory::Images => convert_image(item, &out_path),
                    FileCategory::Audio | FileCategory::Video => convert_audio_video(item, &out_path),
                    FileCategory::Documents => convert_document(item, &out_path),
                };

                match res {
                    Ok(path) => {
                        let _ = tx.send(ConversionEvent::Progress {
                            item_id: item.id,
                            progress: 1.0,
                            stage: "Completed".to_string(),
                        });
                        let _ = tx.send(ConversionEvent::Done {
                            item_id: item.id,
                            out_path: path,
                        });
                    }
                    Err(err) => {
                        let _ = tx.send(ConversionEvent::Error {
                            item_id: item.id,
                            message: err,
                        });
                    }
                }
            }
        });
    }
}

fn ffmpeg_binary() -> PathBuf {
    let local = std::env::var_os("USERPROFILE").map(|u| PathBuf::from(u).join(".local").join("bin").join("ffmpeg.exe")).unwrap_or_else(|| PathBuf::from("ffmpeg.exe"));
    if local.exists() {
        return local;
    }
    let cutcraft = PathBuf::from("../cutcraft/build-deps/ffmpeg/bin/ffmpeg.exe");
    if cutcraft.exists() {
        return cutcraft;
    }
    PathBuf::from("ffmpeg")
}

fn convert_image(item: &ConversionItem, out_path: &Path) -> Result<PathBuf, String> {
    let target_ext = item.target_format.to_ascii_lowercase();

    // Use native image crate where possible
    let img_res = image::open(&item.source_path);
    if let Ok(mut dynamic_img) = img_res {
        // Handle resize if requested
        if let (Some(w), Some(h)) = (item.settings.resize_width, item.settings.resize_height) {
            if item.settings.keep_aspect {
                dynamic_img = dynamic_img.resize(w, h, image::imageops::FilterType::Lanczos3);
            } else {
                dynamic_img = dynamic_img.resize_exact(w, h, image::imageops::FilterType::Lanczos3);
            }
        }

        // Handle visual effect if requested
        if let Some(ref eff) = item.settings.effect {
            if eff.contains("gray") {
                dynamic_img = dynamic_img.grayscale();
            }
        }

        let format = match target_ext.as_str() {
            "png" => Some(image::ImageFormat::Png),
            "jpg" | "jpeg" => Some(image::ImageFormat::Jpeg),
            "webp" => Some(image::ImageFormat::WebP),
            "bmp" => Some(image::ImageFormat::Bmp),
            "ico" => Some(image::ImageFormat::Ico),
            "tiff" | "tif" => Some(image::ImageFormat::Tiff),
            "gif" => Some(image::ImageFormat::Gif),
            _ => None,
        };

        if let Some(fmt) = format {
            if fmt == image::ImageFormat::Jpeg {
                let mut file = std::fs::File::create(out_path).map_err(|e| format!("Failed to create file: {e}"))?;
                let mut encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut file, item.settings.quality as u8);
                encoder.encode_image(&dynamic_img).map_err(|e| format!("JPEG encode error: {e}"))?;
                return Ok(out_path.to_path_buf());
            }

            dynamic_img.save_with_format(out_path, fmt)
                .map_err(|e| format!("Image save error: {e}"))?;
            return Ok(out_path.to_path_buf());
        }
    }

    // Fallback to ffmpeg for unsupported or vector/pdf conversions
    let ff = ffmpeg_binary();
    let mut cmd = Command::new(&ff);
    cmd.arg("-y").arg("-i").arg(&item.source_path);

    if target_ext == "webp" {
        cmd.arg("-quality").arg(format!("{}", item.settings.quality));
    } else if target_ext == "jpg" || target_ext == "jpeg" {
        cmd.arg("-q:v").arg(format!("{}", (100 - item.settings.quality) / 3 + 1));
    }

    cmd.arg(out_path);

    let output = cmd.output().map_err(|e| format!("FFmpeg execution failed: {e}"))?;
    if output.status.success() && out_path.exists() {
        Ok(out_path.to_path_buf())
    } else {
        let err_msg = String::from_utf8_lossy(&output.stderr);
        Err(format!("Image conversion error: {}", err_msg.lines().last().unwrap_or("unknown error")))
    }
}

fn convert_audio_video(item: &ConversionItem, out_path: &Path) -> Result<PathBuf, String> {
    let ff = ffmpeg_binary();
    let target_ext = item.target_format.to_ascii_lowercase();
    let mut cmd = Command::new(&ff);
    cmd.arg("-y").arg("-i").arg(&item.source_path);

    // Audio target (either audio-to-audio or video-to-audio extraction!)
    let is_audio_target = matches!(target_ext.as_str(), "mp3" | "wav" | "flac" | "aac" | "ogg" | "m4a" | "opus");

    if is_audio_target {
        cmd.arg("-vn"); // Strip video stream
        let b = if item.settings.audio_bitrate.is_empty() { "320k" } else { &item.settings.audio_bitrate };
        cmd.arg("-b:a").arg(b);
    } else {
        // Video target: handle resolution, fps, crf
        match item.settings.video_resolution.as_str() {
            "1080p" => { cmd.arg("-vf").arg("scale=-2:1080"); }
            "720p" => { cmd.arg("-vf").arg("scale=-2:720"); }
            "4k" => { cmd.arg("-vf").arg("scale=-2:2160"); }
            _ => {}
        }
        if let Some(fps) = item.settings.video_fps {
            cmd.arg("-r").arg(format!("{fps}"));
        }
        if let Some(crf) = item.settings.video_crf {
            cmd.arg("-crf").arg(format!("{crf}"));
        }
    }

    cmd.arg(out_path);

    let output = cmd.output().map_err(|e| format!("FFmpeg failed: {e}"))?;
    if output.status.success() && out_path.exists() {
        Ok(out_path.to_path_buf())
    } else {
        let err_msg = String::from_utf8_lossy(&output.stderr);
        Err(format!("Media conversion error: {}", err_msg.lines().last().unwrap_or("unknown ffmpeg error")))
    }
}

fn convert_document(item: &ConversionItem, out_path: &Path) -> Result<PathBuf, String> {
    let target_ext = item.target_format.to_ascii_lowercase();
    let ff = ffmpeg_binary();

    // Check if target is PDF
    if target_ext == "pdf" {
        // If image to PDF:
        if let Ok(_) = image::open(&item.source_path) {
            let mut cmd = Command::new(&ff);
            cmd.arg("-y").arg("-i").arg(&item.source_path).arg(out_path);
            if let Ok(out) = cmd.output() {
                if out.status.success() && out_path.exists() {
                    return Ok(out_path.to_path_buf());
                }
            }
        }
    }

    // Default copy or fallback
    let _ = std::fs::copy(&item.source_path, out_path);
    if out_path.exists() {
        Ok(out_path.to_path_buf())
    } else {
        Err(format!("Cannot convert {} to {}", item.source_ext, target_ext))
    }
}
