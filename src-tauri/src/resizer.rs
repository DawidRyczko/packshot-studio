use image::{
    codecs::jpeg::JpegEncoder, imageops, ColorType, DynamicImage, ImageBuffer, Rgba, RgbaImage,
};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tauri::ipc::Channel;

#[derive(Debug, Clone, Deserialize)]
pub struct ResizeOptions {
    pub canvas_width: u32,
    pub canvas_height: u32,
    pub bg_color_hex: String, // e.g. "#FFFFFF" or "transparent"
    pub padding_percent: f32, // e.g. 5.0 for 5% margin on each side
    pub output_format: String, // "jpg", "png", "webp"
    pub quality: u8,          // 1 - 100 (for JPG/WEBP)
    pub output_dir: String,
    pub file_prefix: Option<String>,
    pub file_suffix: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProcessResult {
    pub input_path: String,
    pub output_path: String,
    pub success: bool,
    pub error: Option<String>,
    pub original_width: u32,
    pub original_height: u32,
    pub output_width: u32,
    pub output_height: u32,
    pub output_size_bytes: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProcessProgress {
    pub completed: usize,
    pub total: usize,
    pub current_file: String,
}

fn parse_hex_color(hex: &str) -> Rgba<u8> {
    let clean = hex.trim().trim_start_matches('#');
    if clean.eq_ignore_ascii_case("transparent") {
        return Rgba([0, 0, 0, 0]);
    }

    match clean.len() {
        6 => {
            let r = u8::from_str_radix(&clean[0..2], 16).unwrap_or(255);
            let g = u8::from_str_radix(&clean[2..4], 16).unwrap_or(255);
            let b = u8::from_str_radix(&clean[4..6], 16).unwrap_or(255);
            Rgba([r, g, b, 255])
        }
        8 => {
            let r = u8::from_str_radix(&clean[0..2], 16).unwrap_or(255);
            let g = u8::from_str_radix(&clean[2..4], 16).unwrap_or(255);
            let b = u8::from_str_radix(&clean[4..6], 16).unwrap_or(255);
            let a = u8::from_str_radix(&clean[6..8], 16).unwrap_or(255);
            Rgba([r, g, b, a])
        }
        _ => Rgba([255, 255, 255, 255]), // default white
    }
}

pub fn process_single_image(input_path_str: &str, options: &ResizeOptions) -> ProcessResult {
    let input_path = Path::new(input_path_str);
    let file_stem = input_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("image");

    let ext = match options.output_format.to_lowercase().as_str() {
        "png" => "png",
        "webp" => "webp",
        _ => "jpg",
    };

    let prefix = options.file_prefix.as_deref().unwrap_or("");
    let suffix = options.file_suffix.as_deref().unwrap_or("");
    let new_filename = format!("{}{}{}.{}", prefix, file_stem, suffix, ext);
    let output_path = PathBuf::from(&options.output_dir).join(&new_filename);
    let output_path_str = output_path.to_string_lossy().to_string();

    let mut result = ProcessResult {
        input_path: input_path_str.to_string(),
        output_path: output_path_str.clone(),
        success: false,
        error: None,
        original_width: 0,
        original_height: 0,
        output_width: options.canvas_width,
        output_height: options.canvas_height,
        output_size_bytes: 0,
    };

    let decoded = match image::open(input_path) {
        Ok(img) => img,
        Err(e) => {
            result.error = Some(format!("Failed to open image: {}", e));
            return result;
        }
    };

    let orig_w = decoded.width();
    let orig_h = decoded.height();
    result.original_width = orig_w;
    result.original_height = orig_h;

    let src_rgba = decoded.to_rgba8();

    // Calculate maximum bounding box allowing for padding
    let pad_factor = (options.padding_percent / 100.0).clamp(0.0, 0.45);
    let avail_w = (options.canvas_width as f32 * (1.0 - pad_factor * 2.0)).max(1.0);
    let avail_h = (options.canvas_height as f32 * (1.0 - pad_factor * 2.0)).max(1.0);

    // Calculate proportional fit
    let scale_ratio = (avail_w / orig_w as f32).min(avail_h / orig_h as f32);
    let target_w = ((orig_w as f32 * scale_ratio).round() as u32).max(1);
    let target_h = ((orig_h as f32 * scale_ratio).round() as u32).max(1);

    // High quality resize using Lanczos3
    let resized = imageops::resize(&src_rgba, target_w, target_h, imageops::FilterType::Lanczos3);

    // Prepare canvas with background color
    let mut bg_color = parse_hex_color(&options.bg_color_hex);
    if ext == "jpg" && bg_color[3] == 0 {
        // JPG doesn't support alpha transparency, fallback to solid white
        bg_color = Rgba([255, 255, 255, 255]);
    }

    let mut canvas: RgbaImage =
        ImageBuffer::from_pixel(options.canvas_width, options.canvas_height, bg_color);

    // Center resized image onto the canvas
    let offset_x = ((options.canvas_width - target_w) / 2) as i64;
    let offset_y = ((options.canvas_height - target_h) / 2) as i64;
    imageops::overlay(&mut canvas, &resized, offset_x, offset_y);

    // Ensure output directory exists
    if let Some(parent) = output_path.parent() {
        if let Err(e) = fs::create_dir_all(parent) {
            result.error = Some(format!("Failed to create output directory: {}", e));
            return result;
        }
    }

    // Save image based on requested format
    let save_status: Result<(), String> = match ext {
        "jpg" => {
            // Flatten to RGB for JPEG
            let rgb_img = DynamicImage::ImageRgba8(canvas).to_rgb8();
            let file = match fs::File::create(&output_path) {
                Ok(f) => f,
                Err(e) => {
                    return {
                        result.error = Some(format!("Failed to create output file: {}", e));
                        result
                    };
                }
            };
            let mut encoder = JpegEncoder::new_with_quality(file, options.quality.clamp(1, 100));
            encoder
                .encode(
                    &rgb_img,
                    options.canvas_width,
                    options.canvas_height,
                    ColorType::Rgb8.into(),
                )
                .map_err(|e| format!("JPEG encode error: {}", e))
        }
        "png" => {
            let dyn_img = DynamicImage::ImageRgba8(canvas);
            dyn_img
                .save(&output_path)
                .map_err(|e| format!("PNG save error: {}", e))
        }
        "webp" => {
            let dyn_img = DynamicImage::ImageRgba8(canvas);
            dyn_img
                .save(&output_path)
                .map_err(|e| format!("WEBP save error: {}", e))
        }
        _ => Err("Unsupported format".to_string()),
    };

    match save_status {
        Ok(_) => {
            result.success = true;
            if let Ok(meta) = fs::metadata(&output_path) {
                result.output_size_bytes = meta.len();
            }
        }
        Err(e) => {
            result.error = Some(e);
        }
    }

    result
}

pub fn process_batch_images(
    paths: Vec<String>,
    options: ResizeOptions,
    on_progress: Channel<ProcessProgress>,
) -> Vec<ProcessResult> {
    let total = paths.len();
    let counter = Arc::new(AtomicUsize::new(0));

    // Ensure output folder exists before starting batch
    let _ = fs::create_dir_all(&options.output_dir);

    paths
        .par_iter()
        .map(|path| {
            let res = process_single_image(path, &options);
            let done = counter.fetch_add(1, Ordering::SeqCst) + 1;
            let file_name = Path::new(path)
                .file_name()
                .and_then(|s| s.to_str())
                .unwrap_or(path)
                .to_string();
            let _ = on_progress.send(ProcessProgress {
                completed: done,
                total,
                current_file: file_name,
            });
            res
        })
        .collect()
}
