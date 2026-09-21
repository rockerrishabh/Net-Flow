//! Realtime bandwidth sparkline rendering.
//!
//! Net-Flow draws its sparkline directly into an in-memory RGBA buffer with
//! supersampling and analytic edge anti-aliasing. This produces a mirrored,
//! gradient-filled dual-stream chart (download on top, upload on bottom) on a
//! transparent background without external plotting dependencies or bulky canvas runtimes.

use crate::backend::HistorySample;
use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;
use image::{ImageEncoder, codecs::png::PngEncoder};
use serde::{Deserialize, Serialize};

#[cfg(windows)]
use windows::Win32::System::Registry::{
    HKEY, HKEY_CURRENT_USER, KEY_READ, REG_DWORD, RegCloseKey, RegOpenKeyExW, RegQueryValueExW,
};
#[cfg(windows)]
use windows::core::w;

/// Straight 8-bit RGBA color representation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rgba(pub u8, pub u8, pub u8, pub u8);

/// Download curve color for Dark theme (electric cyan).
pub const DOWNLOAD_COLOR: Rgba = Rgba(56, 217, 240, 255);
/// Upload curve color for Dark theme (warm amber).
pub const UPLOAD_COLOR: Rgba = Rgba(255, 176, 32, 255);
/// Minimum vertical scale floor in bytes/sec (50 KB/s).
/// Prevents small background network noise (e.g. 500 B/s) from stretching across the full chart height.
pub const MIN_CHART_SCALE_BPS: f64 = 50_000.0;
/// Subdued center dividing line separating download and upload regions in Dark theme.
const AXIS_COLOR: Rgba = Rgba(150, 160, 176, 56);

/// Maximum opacity for the gradient fill directly adjacent to the curve stroke.
const FILL_ALPHA_NEAR: f32 = 120.0;
/// Fade-out opacity for the gradient fill near the baseline axis.
const FILL_ALPHA_FAR: f32 = 14.0;
/// Small vertical offset in pixels to keep zero-traffic rails visually distinct
/// from each other and the center axis line.
const IDLE_OFFSET_PX: f64 = 1.25;

/// Error conditions when probing Windows application theme from the registry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeDetectionError {
    OpenKeyFailed(i32),
    QueryValueFailed(i32),
    InvalidType,
    UnsupportedPlatform,
}

/// Fallible query for Windows application light theme.
/// Returns Ok(true) if AppsUseLightTheme is set to 1.
/// Returns Ok(false) if set to 0.
/// Returns Err(ThemeDetectionError) if key or value is absent or fails to open.
pub fn query_windows_light_theme() -> Result<bool, ThemeDetectionError> {
    #[cfg(windows)]
    unsafe {
        let mut hkey = HKEY::default();
        let subkey = w!("Software\\Microsoft\\Windows\\CurrentVersion\\Themes\\Personalize");
        let status = RegOpenKeyExW(HKEY_CURRENT_USER, subkey, Some(0), KEY_READ, &mut hkey);
        if status.is_err() {
            return Err(ThemeDetectionError::OpenKeyFailed(status.0 as i32));
        }

        let value_name = w!("AppsUseLightTheme");
        let mut val_type = Default::default();
        let mut data: u32 = 0;
        let mut data_len = core::mem::size_of::<u32>() as u32;

        let query_status = RegQueryValueExW(
            hkey,
            value_name,
            None,
            Some(&mut val_type),
            Some(&mut data as *mut u32 as *mut u8),
            Some(&mut data_len),
        );
        let _ = RegCloseKey(hkey);

        if query_status.is_err() {
            return Err(ThemeDetectionError::QueryValueFailed(query_status.0 as i32));
        }

        if val_type != REG_DWORD {
            return Err(ThemeDetectionError::InvalidType);
        }

        Ok(data == 1)
    }

    #[cfg(not(windows))]
    Err(ThemeDetectionError::UnsupportedPlatform)
}

/// Safe helper detecting if Windows is currently using light theme for apps.
/// Gracefully falls back to `false` (Dark theme default) if the registry key is
/// absent or uninitialized.
pub fn detect_windows_light_theme() -> bool {
    query_windows_light_theme().unwrap_or(false)
}

/// User-selectable theme mode configuration.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum ThemeMode {
    /// Automatically match the host Windows theme via registry detection.
    #[default]
    Auto,
    /// Force high-contrast electric Dark theme.
    Dark,
    /// Force high-contrast crisp Light theme.
    Light,
}

impl ThemeMode {
    /// Resolves user theme policy to a concrete rendering theme.
    pub fn resolve(self) -> ResolvedTheme {
        match self {
            Self::Auto => {
                if detect_windows_light_theme() {
                    ResolvedTheme::Light
                } else {
                    ResolvedTheme::Dark
                }
            }
            Self::Dark => ResolvedTheme::Dark,
            Self::Light => ResolvedTheme::Light,
        }
    }

    pub fn to_str_value(self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::Dark => "dark",
            Self::Light => "light",
        }
    }

    pub fn from_str_value(val: &str) -> Self {
        match val {
            "dark" => Self::Dark,
            "light" => Self::Light,
            _ => Self::Auto,
        }
    }
}

/// Concrete resolved theme used by the rasterizer and cache keys.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum ResolvedTheme {
    #[default]
    Dark,
    Light,
}

/// Color palette tailored for dark vs light card backgrounds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Palette {
    pub download: Rgba,
    pub upload: Rgba,
    pub axis: Rgba,
}

impl Palette {
    pub fn for_theme(theme: ResolvedTheme) -> Self {
        match theme {
            ResolvedTheme::Dark => Self {
                download: DOWNLOAD_COLOR,
                upload: UPLOAD_COLOR,
                axis: AXIS_COLOR,
            },
            ResolvedTheme::Light => Self {
                download: Rgba(0, 140, 180, 255), // #008CB4 deep cyan with >4.5:1 contrast on white
                upload: Rgba(215, 95, 0, 255), // #D75F00 deep amber with >4.5:1 contrast on white
                axis: Rgba(110, 125, 145, 90), // refined medium-dark axis line
            },
        }
    }
}

/// Chart graph presentation style.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum GraphStyle {
    /// Mirrored Catmull-Rom spline with gradient area fill fading to baseline.
    #[default]
    Area,
    /// Clean, minimalist spline stroke with glowing pulse dot and no area fill.
    Line,
    /// Discrete vertical bandwidth bars per sample with rounded heads from baseline.
    Bar,
}

impl GraphStyle {
    pub fn to_str_value(self) -> &'static str {
        match self {
            Self::Area => "area",
            Self::Line => "line",
            Self::Bar => "bar",
        }
    }

    pub fn from_str_value(val: &str) -> Self {
        match val {
            "line" => Self::Line,
            "bar" => Self::Bar,
            _ => Self::Area,
        }
    }
}

/// Telemetry stream direction selector.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Track {
    Download,
    Upload,
}

impl Track {
    pub fn color(self) -> Rgba {
        match self {
            Track::Download => DOWNLOAD_COLOR,
            Track::Upload => UPLOAD_COLOR,
        }
    }

    pub fn color_with_palette(self, palette: &Palette) -> Rgba {
        match self {
            Track::Download => palette.download,
            Track::Upload => palette.upload,
        }
    }
}

/// Strongly typed widget size preset for chart rendering.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChartSize {
    Small,
    Medium,
    Large,
}

impl ChartSize {
    pub fn from_str_name(name: &str) -> Self {
        match name {
            "Small" => Self::Small,
            "Large" => Self::Large,
            _ => Self::Medium,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Small => "Small",
            Self::Medium => "Medium",
            Self::Large => "Large",
        }
    }
}

/// Cache key covering every rendering input that affects the idle chart output.
/// Invariant: For every two rendering states represented by the same cache key,
/// the idle PNG output must be bit-for-bit identical.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IdleChartCacheKey {
    pub size: ChartSize,
    pub chart_window: u32,
    pub width: u32,
    pub height: u32,
    pub supersample: u32,
    pub resolved_theme: ResolvedTheme,
    pub graph_style: GraphStyle,
}

/// Target pixel dimensions and sampling resolution for a widget size.
pub struct ChartDimensions {
    pub width: u32,
    pub height: u32,
    pub sample_count: usize,
    /// Supersampling multiplier for antialiased rasterization.
    pub supersample: u32,
}

impl ChartDimensions {
    pub fn for_chart_size(size: ChartSize, chart_window: u32) -> Self {
        let sample_count = crate::history_samples_for_secs(crate::clamp_chart_window(chart_window));
        match size {
            ChartSize::Small => ChartDimensions {
                width: 400,
                height: 44,
                sample_count,
                supersample: 3,
            },
            ChartSize::Large => ChartDimensions {
                width: 800,
                height: 150,
                sample_count,
                supersample: 2,
            },
            ChartSize::Medium => ChartDimensions {
                width: 600,
                height: 64,
                sample_count,
                supersample: 3,
            },
        }
    }

    pub fn for_size(size: &str, chart_window: u32) -> Self {
        Self::for_chart_size(ChartSize::from_str_name(size), chart_window)
    }
}

/// A simple RGBA canvas with source-over compositing.
struct Canvas {
    width: u32,
    height: u32,
    pixels: Vec<u8>,
}

impl Canvas {
    fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            pixels: vec![0u8; (width * height * 4) as usize],
        }
    }

    fn blend(&mut self, x: u32, y: u32, color: Rgba, alpha_scale: f32) {
        if x >= self.width || y >= self.height {
            return;
        }
        let a = (color.3 as f32 * alpha_scale.clamp(0.0, 1.0)) / 255.0;
        if a <= 0.0 {
            return;
        }
        let idx = ((y * self.width + x) * 4) as usize;
        let dst_alpha_byte = self.pixels[idx + 3];

        if dst_alpha_byte == 0 {
            // Fast path: destination is completely transparent (virgin canvas).
            // Under source-over compositing, transparent destination contributes nothing:
            // out_a = a, val = src.
            self.pixels[idx] = color.0;
            self.pixels[idx + 1] = color.1;
            self.pixels[idx + 2] = color.2;
            self.pixels[idx + 3] = (a * 255.0).round().clamp(0.0, 255.0) as u8;
            return;
        }

        let dst_a = dst_alpha_byte as f32 / 255.0;
        let out_a = a + dst_a * (1.0 - a);
        if out_a <= 0.0 {
            return;
        }
        let inv_out_a = 1.0 / out_a;
        let dst_weight = dst_a * (1.0 - a);
        let src_weight = a;

        let r = (color.0 as f32 * src_weight + self.pixels[idx] as f32 * dst_weight) * inv_out_a;
        let g =
            (color.1 as f32 * src_weight + self.pixels[idx + 1] as f32 * dst_weight) * inv_out_a;
        let b =
            (color.2 as f32 * src_weight + self.pixels[idx + 2] as f32 * dst_weight) * inv_out_a;

        self.pixels[idx] = r.round().clamp(0.0, 255.0) as u8;
        self.pixels[idx + 1] = g.round().clamp(0.0, 255.0) as u8;
        self.pixels[idx + 2] = b.round().clamp(0.0, 255.0) as u8;
        self.pixels[idx + 3] = (out_a * 255.0).round().clamp(0.0, 255.0) as u8;
    }

    /// Fills vertical range `top..bottom` with fractional pixel coverage for anti-aliasing.
    fn fill_span(&mut self, x: u32, top: f64, bottom: f64, color: Rgba, alpha_scale: f32) {
        if top.is_nan() || bottom.is_nan() || bottom <= top {
            return;
        }
        let first = top.floor().max(0.0) as i64;
        let last = (bottom.ceil() as i64 - 1).min(self.height as i64 - 1);
        for y in first..=last {
            if y < 0 {
                continue;
            }
            let coverage = (bottom.min((y + 1) as f64) - top.max(y as f64)).clamp(0.0, 1.0);
            if coverage > 0.0 {
                self.blend(x, y as u32, color, alpha_scale * coverage as f32);
            }
        }
    }

    fn fill_row(&mut self, y: i64, color: Rgba) {
        if y < 0 || y >= self.height as i64 {
            return;
        }
        for x in 0..self.width {
            self.blend(x, y as u32, color, 1.0);
        }
    }

    /// Downsamples supersampled buffer using box filter averaging in premultiplied alpha space.
    fn downsample(&self, factor: u32) -> (u32, u32, Vec<u8>) {
        if factor <= 1 {
            return (self.width, self.height, self.pixels.clone());
        }
        let out_w = self.width / factor;
        let out_h = self.height / factor;
        let mut out = vec![0u8; (out_w * out_h * 4) as usize];
        let samples = (factor * factor) as f32;

        for oy in 0..out_h {
            for ox in 0..out_w {
                let (mut r, mut g, mut b, mut a) = (0.0f32, 0.0f32, 0.0f32, 0.0f32);
                for dy in 0..factor {
                    let row_start = ((oy * factor + dy) * self.width + ox * factor) as usize * 4;
                    for dx in 0..factor {
                        let idx = row_start + dx as usize * 4;
                        let pa_byte = self.pixels[idx + 3];
                        if pa_byte == 0 {
                            continue;
                        }
                        let pa = pa_byte as f32 / 255.0;
                        r += self.pixels[idx] as f32 * pa;
                        g += self.pixels[idx + 1] as f32 * pa;
                        b += self.pixels[idx + 2] as f32 * pa;
                        a += pa;
                    }
                }
                let o = ((oy * out_w + ox) * 4) as usize;
                if a > 0.0 {
                    out[o] = (r / a).round().clamp(0.0, 255.0) as u8;
                    out[o + 1] = (g / a).round().clamp(0.0, 255.0) as u8;
                    out[o + 2] = (b / a).round().clamp(0.0, 255.0) as u8;
                    out[o + 3] = ((a / samples) * 255.0).round().clamp(0.0, 255.0) as u8;
                }
            }
        }
        (out_w, out_h, out)
    }
}

fn encode_png(width: u32, height: u32, rgba: &[u8]) -> Result<Vec<u8>, String> {
    let mut png_bytes = Vec::new();
    PngEncoder::new(&mut png_bytes)
        .write_image(rgba, width, height, image::ExtendedColorType::Rgba8)
        .map_err(|e| format!("PNG encode error: {:?}", e))?;
    Ok(png_bytes)
}

/// Takes the latest `sample_count` points, front-padding with zeroes when the
/// widget first starts up so the curve scrolls in smoothly from the right.
fn windowed(values: &[f64], sample_count: usize) -> Vec<f64> {
    let total = sample_count.max(2);
    let mut out = vec![0.0f64; total];
    let take = values.len().min(total);
    let src = &values[values.len() - take..];
    let offset = total - take;
    out[offset..].copy_from_slice(src);
    out
}

/// Evaluates a Catmull-Rom spline at fractional position `t` for smooth column interpolation.
fn catmull_at(values: &[f64], t: f64) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    if values.len() == 1 {
        return values[0];
    }
    let max_i = values.len() - 1;
    let i = t.floor().max(0.0) as usize;
    let i = i.min(max_i.saturating_sub(1));
    let f = (t - i as f64).clamp(0.0, 1.0);

    let p0 = values[i.saturating_sub(1)];
    let p1 = values[i];
    let p2 = values[(i + 1).min(max_i)];
    let p3 = values[(i + 2).min(max_i)];

    let f2 = f * f;
    let f3 = f2 * f;
    let v = 0.5
        * ((2.0 * p1)
            + (-p0 + p2) * f
            + (2.0 * p0 - 5.0 * p1 + 4.0 * p2 - p3) * f2
            + (-p0 + 3.0 * p1 - 3.0 * p2 + p3) * f3);
    v.max(0.0)
}

/// Renders a single direction track with Catmull-Rom spline, gradient area fill, and pulse indicator.
#[allow(clippy::too_many_arguments)]
fn draw_area_track(
    canvas: &mut Canvas,
    values: &[f64],
    scale: f64,
    baseline_y: f64,
    span: f64,
    direction: f64,
    color: Rgba,
    stroke: f64,
    idle_offset: f64,
) {
    let width = canvas.width;
    if width == 0 || values.len() < 2 || span <= 0.0 {
        return;
    }
    let last_index = (values.len() - 1) as f64;

    let x_factor = if width > 1 {
        last_index / (width - 1) as f64
    } else {
        0.0
    };
    let reachable = (span - idle_offset).max(0.0);
    let inv_scale = if scale > 0.0 { 1.0 / scale } else { 0.0 };

    for x in 0..width {
        let t = x as f64 * x_factor;
        let v = catmull_at(values, t);
        let norm = (v * inv_scale).clamp(0.0, 1.0);
        let y = baseline_y + direction * (idle_offset + norm * reachable);

        // Gradient area fill: brightest against the stroke, fading out towards
        // the baseline so the two tracks never fight for attention.
        let (top, bottom) = if direction < 0.0 {
            (y, baseline_y)
        } else {
            (baseline_y, y)
        };
        let reach = (y - baseline_y).abs().max(1e-6);
        let inv_reach = 1.0 / reach;
        let first = top.floor().max(0.0) as i64;
        let last_row = (bottom.ceil() as i64 - 1).min(canvas.height as i64 - 1);

        if first <= last_row && first >= 0 {
            if first == last_row {
                // Single boundary row
                let coverage =
                    (bottom.min((first + 1) as f64) - top.max(first as f64)).clamp(0.0, 1.0);
                if coverage > 0.0 {
                    let nearness = (((first as f64 + 0.5) - baseline_y).abs() * inv_reach).min(1.0);
                    let alpha = FILL_ALPHA_FAR
                        + (FILL_ALPHA_NEAR - FILL_ALPHA_FAR) * (nearness * nearness) as f32;
                    canvas.blend(x, first as u32, color, (alpha / 255.0) * coverage as f32);
                }
            } else {
                // Top boundary row
                let first_cov = ((first + 1) as f64 - top).clamp(0.0, 1.0);
                if first_cov > 0.0 {
                    let nearness = (((first as f64 + 0.5) - baseline_y).abs() * inv_reach).min(1.0);
                    let alpha = FILL_ALPHA_FAR
                        + (FILL_ALPHA_NEAR - FILL_ALPHA_FAR) * (nearness * nearness) as f32;
                    canvas.blend(x, first as u32, color, (alpha / 255.0) * first_cov as f32);
                }

                // Interior rows: coverage == 1.0 unconditionally
                for py in (first + 1)..last_row {
                    let nearness = (((py as f64 + 0.5) - baseline_y).abs() * inv_reach).min(1.0);
                    let alpha = FILL_ALPHA_FAR
                        + (FILL_ALPHA_NEAR - FILL_ALPHA_FAR) * (nearness * nearness) as f32;
                    canvas.blend(x, py as u32, color, alpha / 255.0);
                }

                // Bottom boundary row
                let last_cov = (bottom - last_row as f64).clamp(0.0, 1.0);
                if last_cov > 0.0 {
                    let nearness =
                        (((last_row as f64 + 0.5) - baseline_y).abs() * inv_reach).min(1.0);
                    let alpha = FILL_ALPHA_FAR
                        + (FILL_ALPHA_NEAR - FILL_ALPHA_FAR) * (nearness * nearness) as f32;
                    canvas.blend(x, last_row as u32, color, (alpha / 255.0) * last_cov as f32);
                }
            }
        }

        // Stroke, centred on the curve.
        let half = stroke / 2.0;
        canvas.fill_span(x, y - half, y + half, color, 1.0);
    }

    // Live pulse indicator dot at the leading edge (current sample)
    if width > 0 {
        let last_val = values.last().copied().unwrap_or(0.0);
        let norm = if scale > 0.0 {
            (last_val / scale).clamp(0.0, 1.0)
        } else {
            0.0
        };
        let reachable = (span - idle_offset).max(0.0);
        let dot_center_x = (width - 1) as f64;
        let dot_center_y = baseline_y + direction * (idle_offset + norm * reachable);

        let dot_radius = stroke * 0.95;
        let glow_radius = stroke * 2.6;

        draw_pulse_dot(
            canvas,
            dot_center_x,
            dot_center_y,
            dot_radius,
            glow_radius,
            color,
        );
    }
}

/// Renders a single direction track with Catmull-Rom spline stroke and pulse dot (minimalist, no area fill).
#[allow(clippy::too_many_arguments)]
fn draw_line_track(
    canvas: &mut Canvas,
    values: &[f64],
    scale: f64,
    baseline_y: f64,
    span: f64,
    direction: f64,
    color: Rgba,
    stroke: f64,
    idle_offset: f64,
) {
    let width = canvas.width;
    if width == 0 || values.len() < 2 || span <= 0.0 {
        return;
    }
    let last_index = (values.len() - 1) as f64;

    let x_factor = if width > 1 {
        last_index / (width - 1) as f64
    } else {
        0.0
    };
    let reachable = (span - idle_offset).max(0.0);
    let inv_scale = if scale > 0.0 { 1.0 / scale } else { 0.0 };

    let half = stroke / 2.0;
    for x in 0..width {
        let t = x as f64 * x_factor;
        let v = catmull_at(values, t);
        let norm = (v * inv_scale).clamp(0.0, 1.0);
        let y = baseline_y + direction * (idle_offset + norm * reachable);

        // Stroke only (no gradient area fill)
        canvas.fill_span(x, y - half, y + half, color, 1.0);
    }

    if width > 0 {
        let last_val = values.last().copied().unwrap_or(0.0);
        let norm = if scale > 0.0 {
            (last_val / scale).clamp(0.0, 1.0)
        } else {
            0.0
        };
        let reachable = (span - idle_offset).max(0.0);
        let dot_center_x = (width - 1) as f64;
        let dot_center_y = baseline_y + direction * (idle_offset + norm * reachable);

        let dot_radius = stroke * 0.95;
        let glow_radius = stroke * 2.6;

        draw_pulse_dot(
            canvas,
            dot_center_x,
            dot_center_y,
            dot_radius,
            glow_radius,
            color,
        );
    }
}

/// Renders discrete vertical bandwidth bars per sample without spline geometry.
#[allow(clippy::too_many_arguments)]
fn draw_bar_track(
    canvas: &mut Canvas,
    values: &[f64],
    scale: f64,
    baseline_y: f64,
    span: f64,
    direction: f64,
    color: Rgba,
    idle_offset: f64,
    supersample: u32,
) {
    let width = canvas.width;
    let n = values.len();
    if width == 0 || n == 0 || span <= 0.0 {
        return;
    }

    let reachable = (span - idle_offset).max(0.0);
    let inv_scale = if scale > 0.0 { 1.0 / scale } else { 0.0 };
    let col_w = width as f64 / n as f64;
    let gap = (supersample as f64 * 0.75).clamp(1.0, col_w * 0.35);
    let bar_w = (col_w - gap).max(1.0);

    for (i, &v) in values.iter().enumerate() {
        let norm = (v * inv_scale).clamp(0.0, 1.0);
        let bar_h = idle_offset + norm * reachable;
        let y = baseline_y + direction * bar_h;

        let (top, bottom) = if direction < 0.0 {
            (y, baseline_y)
        } else {
            (baseline_y, y)
        };

        let x_start = (i as f64 * col_w).round() as u32;
        let x_end = ((i as f64 * col_w + bar_w).round() as u32).min(width);

        let is_last = i == n - 1;
        let base_alpha = if is_last { 0.95 } else { 0.82 };

        for x in x_start..x_end {
            canvas.fill_span(x, top, bottom, color, base_alpha);
        }
    }
}

/// Renders a single direction track (download or upload) dispatching to the configured GraphStyle.
#[allow(clippy::too_many_arguments)]
fn draw_track(
    canvas: &mut Canvas,
    values: &[f64],
    scale: f64,
    baseline_y: f64,
    span: f64,
    direction: f64,
    color: Rgba,
    stroke: f64,
    idle_offset: f64,
    style: GraphStyle,
    supersample: u32,
) {
    match style {
        GraphStyle::Area => {
            draw_area_track(
                canvas,
                values,
                scale,
                baseline_y,
                span,
                direction,
                color,
                stroke,
                idle_offset,
            );
        }
        GraphStyle::Line => {
            draw_line_track(
                canvas,
                values,
                scale,
                baseline_y,
                span,
                direction,
                color,
                stroke,
                idle_offset,
            );
        }
        GraphStyle::Bar => {
            draw_bar_track(
                canvas,
                values,
                scale,
                baseline_y,
                span,
                direction,
                color,
                idle_offset,
                supersample,
            );
        }
    }
}

/// Draws a glowing pulse dot on the leading edge (current sample) of the sparkline.
fn draw_pulse_dot(
    canvas: &mut Canvas,
    cx: f64,
    cy: f64,
    dot_radius: f64,
    glow_radius: f64,
    color: Rgba,
) {
    if canvas.width == 0 || canvas.height == 0 || glow_radius <= 0.0 {
        return;
    }
    let min_x = (cx - glow_radius).floor().max(0.0) as u32;
    let max_x = (cx + glow_radius).ceil().min((canvas.width - 1) as f64) as u32;
    let min_y = (cy - glow_radius).floor().max(0.0) as u32;
    let max_y = (cy + glow_radius).ceil().min((canvas.height - 1) as f64) as u32;

    for py in min_y..=max_y {
        for px in min_x..=max_x {
            let dx = (px as f64 + 0.5) - cx;
            let dy = (py as f64 + 0.5) - cy;
            let dist = (dx * dx + dy * dy).sqrt();

            if dist <= dot_radius {
                let edge = (dot_radius - dist).clamp(0.0, 1.0);
                canvas.blend(px, py, color, (0.85 + 0.15 * edge) as f32);
            } else if dist <= glow_radius {
                let t = (dist - dot_radius) / (glow_radius - dot_radius);
                let glow_alpha = (1.0 - t).powi(2) * 0.40;
                canvas.blend(px, py, color, glow_alpha as f32);
            }
        }
    }
}

/// Renders the mirrored dual-stream chart with default Dark theme and Area style.
pub fn render_unified_dual_chart_png(
    history: &[HistorySample],
    sample_count: usize,
    width: u32,
    height: u32,
) -> Result<Vec<u8>, String> {
    render_unified_dual_chart_png_ss(
        history,
        sample_count,
        width,
        height,
        3,
        ResolvedTheme::Dark,
        GraphStyle::Area,
    )
}

pub fn render_unified_dual_chart_png_ss(
    history: &[HistorySample],
    sample_count: usize,
    width: u32,
    height: u32,
    supersample: u32,
    theme: ResolvedTheme,
    style: GraphStyle,
) -> Result<Vec<u8>, String> {
    if width == 0 || height == 0 {
        return Err("zero-sized chart".to_string());
    }
    let ss = supersample.max(1);

    let rx_all: Vec<f64> = history.iter().map(|s| s.rx_bps as f64).collect();
    let tx_all: Vec<f64> = history.iter().map(|s| s.tx_bps as f64).collect();

    let rx = apply_fluid_wave_smoothing(&windowed(&rx_all, sample_count));
    let tx = apply_fluid_wave_smoothing(&windowed(&tx_all, sample_count));

    // One shared scale keeps the mirrored halves honest: an upload spike only
    // looks as tall as a download spike when it actually is one.
    let scale = rx
        .iter()
        .chain(tx.iter())
        .cloned()
        .fold(0.0f64, f64::max)
        .max(MIN_CHART_SCALE_BPS);

    let mut canvas = Canvas::new(width * ss, height * ss);
    let baseline = (height * ss) as f64 / 2.0;
    // 82% span factor leaves ~18% headroom so peaks never bump against card boundaries.
    let span = (baseline - (ss as f64 * 2.0)).max(1.0) * 0.82;
    let stroke = (ss as f64 * 1.6).max(1.0);
    let idle_offset = ss as f64 * IDLE_OFFSET_PX;
    let palette = Palette::for_theme(theme);

    draw_track(
        &mut canvas,
        &rx,
        scale,
        baseline,
        span,
        -1.0,
        palette.download,
        stroke,
        idle_offset,
        style,
        ss,
    );
    draw_track(
        &mut canvas,
        &tx,
        scale,
        baseline,
        span,
        1.0,
        palette.upload,
        stroke,
        idle_offset,
        style,
        ss,
    );

    canvas.fill_row(baseline.round() as i64, palette.axis);

    let (out_w, out_h, rgba) = canvas.downsample(ss);
    encode_png(out_w, out_h, &rgba)
}

/// Renders a single-stream sparkline (download or upload only) as PNG bytes.
pub fn render_chart_png(
    history: &[HistorySample],
    sample_count: usize,
    width: u32,
    height: u32,
    track: Track,
) -> Result<Vec<u8>, String> {
    if width == 0 || height == 0 {
        return Err("zero-sized chart".to_string());
    }
    let ss = 3u32;
    let raw: Vec<f64> = history
        .iter()
        .map(|s| match track {
            Track::Download => s.rx_bps as f64,
            Track::Upload => s.tx_bps as f64,
        })
        .collect();

    let values = apply_fluid_wave_smoothing(&windowed(&raw, sample_count));
    let scale = values
        .iter()
        .cloned()
        .fold(0.0f64, f64::max)
        .max(MIN_CHART_SCALE_BPS);

    let mut canvas = Canvas::new(width * ss, height * ss);
    let baseline = (height * ss) as f64 - ss as f64;
    let span = (baseline - (ss as f64 * 2.0)).max(1.0) * 0.82;
    let stroke = (ss as f64 * 1.6).max(1.0);

    draw_track(
        &mut canvas,
        &values,
        scale,
        baseline,
        span,
        -1.0,
        track.color(),
        stroke,
        0.0,
        GraphStyle::Area,
        ss,
    );

    let (out_w, out_h, rgba) = canvas.downsample(ss);
    encode_png(out_w, out_h, &rgba)
}

static IDLE_CHART_CACHE: std::sync::OnceLock<std::collections::HashMap<IdleChartCacheKey, String>> =
    std::sync::OnceLock::new();

/// Thread-safe immutable idle chart cache precomputed for standard size, window, theme, and style presets.
fn get_idle_chart_cache() -> &'static std::collections::HashMap<IdleChartCacheKey, String> {
    IDLE_CHART_CACHE.get_or_init(|| {
        let mut map = std::collections::HashMap::new();
        for size in [ChartSize::Small, ChartSize::Medium, ChartSize::Large] {
            for window in [15, 30, 60] {
                for theme in [ResolvedTheme::Dark, ResolvedTheme::Light] {
                    for style in [GraphStyle::Area, GraphStyle::Line, GraphStyle::Bar] {
                        let dims = ChartDimensions::for_chart_size(size, window);
                        let key = IdleChartCacheKey {
                            size,
                            chart_window: window,
                            width: dims.width,
                            height: dims.height,
                            supersample: dims.supersample,
                            resolved_theme: theme,
                            graph_style: style,
                        };
                        let empty_history: Vec<HistorySample> = Vec::new();
                        if let Ok(png) = render_unified_dual_chart_png_ss(
                            &empty_history,
                            dims.sample_count,
                            dims.width,
                            dims.height,
                            dims.supersample,
                            theme,
                            style,
                        ) {
                            map.insert(key, png_to_data_uri(&png));
                        }
                    }
                }
            }
        }
        map
    })
}

/// Renders the unified dual-stream chart for a widget size preset and returns a data URI.
/// When the history window contains zero network traffic, serves the result directly from
/// the immutable idle cache in O(1) time with zero rasterization.
pub fn render_idle_unified_chart_data_uri(
    size: &str,
    chart_window: u32,
    theme: ResolvedTheme,
    style: GraphStyle,
) -> String {
    let chart_size = ChartSize::from_str_name(size);
    let clamped_window = crate::clamp_chart_window(chart_window);
    let dims = ChartDimensions::for_chart_size(chart_size, clamped_window);
    let key = IdleChartCacheKey {
        size: chart_size,
        chart_window: clamped_window,
        width: dims.width,
        height: dims.height,
        supersample: dims.supersample,
        resolved_theme: theme,
        graph_style: style,
    };
    if let Some(cached_uri) = get_idle_chart_cache().get(&key) {
        return cached_uri.clone();
    }
    String::new()
}

/// Renders the unified dual-stream chart for a widget size preset and returns a data URI.
/// When the history window contains zero network traffic, serves the result directly from
/// the immutable idle cache in O(1) time with zero rasterization.
pub fn render_unified_chart_data_uri(
    history: &[HistorySample],
    size: &str,
    chart_window: u32,
    theme: ResolvedTheme,
    style: GraphStyle,
) -> String {
    let chart_size = ChartSize::from_str_name(size);
    let clamped_window = crate::clamp_chart_window(chart_window);
    let dims = ChartDimensions::for_chart_size(chart_size, clamped_window);

    // O(1) idle detection: if history is empty or all samples in the active window are 0 bps
    let window_samples = dims.sample_count.min(history.len());
    let is_idle = history.is_empty()
        || history[history.len() - window_samples..]
            .iter()
            .all(|s| s.rx_bps == 0 && s.tx_bps == 0);

    if is_idle {
        return render_idle_unified_chart_data_uri(size, chart_window, theme, style);
    }

    match render_unified_dual_chart_png_ss(
        history,
        dims.sample_count,
        dims.width,
        dims.height,
        dims.supersample,
        theme,
        style,
    ) {
        Ok(png) => png_to_data_uri(&png),
        Err(_) => String::new(),
    }
}

/// Renders separate download and upload charts and returns a tuple of base64 data URIs.
pub fn render_chart_data_uris(
    history: &[HistorySample],
    size: &str,
    chart_window: u32,
) -> (String, String) {
    let dims = ChartDimensions::for_size(size, chart_window);
    let half = (dims.height / 2).max(1);

    let render = |track: Track| match render_chart_png(
        history,
        dims.sample_count,
        dims.width,
        half,
        track,
    ) {
        Ok(png) => png_to_data_uri(&png),
        Err(_) => String::new(),
    };

    (render(Track::Download), render(Track::Upload))
}

/// Encodes raw PNG bytes into a standard `data:image/png;base64,...` URI string.
pub fn png_to_data_uri(png_bytes: &[u8]) -> String {
    format!(
        "data:image/png;base64,{}",
        BASE64_STANDARD.encode(png_bytes)
    )
}

/// Smooths discrete sample jitter using a Gaussian filter while preserving the true peak value.
pub fn apply_fluid_wave_smoothing(raw_values: &[f64]) -> Vec<f64> {
    if raw_values.len() <= 2 {
        return raw_values.to_vec();
    }

    let n = raw_values.len();
    let mut smoothed = vec![0.0f64; n];

    // 5-point Gaussian kernel.
    let kernel = [0.0625, 0.25, 0.375, 0.25, 0.0625];
    let radius = 2isize;

    for i in 0..n {
        let mut sum = 0.0;
        let mut weight_sum = 0.0;
        for (k_idx, &weight) in kernel.iter().enumerate() {
            let offset = k_idx as isize - radius;
            let sample_idx = i as isize + offset;
            if sample_idx >= 0 && (sample_idx as usize) < n {
                sum += raw_values[sample_idx as usize] * weight;
                weight_sum += weight;
            }
        }
        smoothed[i] = if weight_sum > 0.0 {
            sum / weight_sum
        } else {
            raw_values[i]
        };
    }

    // Restore the original peak so smoothing never understates a spike.
    let original_max = raw_values.iter().cloned().fold(0.0f64, f64::max);
    let smoothed_max = smoothed.iter().cloned().fold(0.0f64, f64::max);
    if original_max > 0.0 && smoothed_max > 0.0 {
        let scale = original_max / smoothed_max;
        for v in &mut smoothed {
            *v *= scale;
        }
    }

    // Anchor the leading edge (live sample) so the pulse dot and rightmost stroke
    // accurately reflect the instantaneous rate. Especially when current traffic is 0,
    // the trace must rest on the baseline without floating.
    if let (Some(&last_raw), Some(last_smoothed)) = (raw_values.last(), smoothed.last_mut()) {
        if last_raw <= 0.0 {
            *last_smoothed = 0.0;
        } else {
            *last_smoothed = last_raw;
        }
    }

    smoothed
}

/// Generates an interpolated Catmull-Rom point sequence from discrete coordinates.
pub fn smooth_flowing_curve(points: &[(f64, f64)], subdivisions: usize) -> Vec<(f64, f64)> {
    if points.len() < 2 || subdivisions <= 1 {
        return points.to_vec();
    }
    let values: Vec<f64> = points.iter().map(|p| p.1).collect();
    let n = points.len();
    let mut out = Vec::with_capacity((n - 1) * subdivisions + 1);

    for i in 0..(n - 1) {
        for step in 0..subdivisions {
            let t = i as f64 + step as f64 / subdivisions as f64;
            let x =
                points[i].0 + (points[i + 1].0 - points[i].0) * (step as f64 / subdivisions as f64);
            out.push((x, catmull_at(&values, t)));
        }
    }
    if let Some(&last) = points.last() {
        out.push((last.0, last.1.max(0.0)));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn history(n: usize) -> Vec<HistorySample> {
        (0..n)
            .map(|i| {
                HistorySample::from_bps(
                    ((i % 10) as u64) * 1000,
                    ((i % 7) as u64) * 400,
                    500_000_000,
                )
            })
            .collect()
    }

    fn decode(png: &[u8]) -> (u32, u32, Vec<u8>) {
        let img = image::load_from_memory(png).expect("valid png").to_rgba8();
        (img.width(), img.height(), img.into_raw())
    }

    #[test]
    fn unified_chart_renders_expected_dimensions() {
        let png = render_unified_dual_chart_png(&history(60), 60, 120, 40).unwrap();
        let (w, h, _) = decode(&png);
        assert_eq!((w, h), (120, 40));
    }

    #[test]
    fn empty_history_still_renders() {
        let png = render_unified_dual_chart_png(&[], 60, 120, 40).unwrap();
        let (w, h, _) = decode(&png);
        assert_eq!((w, h), (120, 40));
    }

    #[test]
    fn background_stays_transparent() {
        let png = render_unified_dual_chart_png(&history(60), 60, 120, 40).unwrap();
        let (w, h, rgba) = decode(&png);
        // The fixture starts at zero bandwidth, so the leading column is empty
        // above and below the axis.
        assert_eq!(rgba[3], 0, "chart must not paint its own background");
        let bottom_left = (((h - 1) * w) * 4 + 3) as usize;
        assert_eq!(rgba[bottom_left], 0);
    }

    #[test]
    fn download_draws_above_the_axis_and_upload_below() {
        let hist: Vec<HistorySample> = (0..40)
            .map(|_| HistorySample::from_bps(1_000_000, 1_000_000, 500_000_000))
            .collect();
        let png = render_unified_dual_chart_png(&hist, 40, 60, 40).unwrap();
        let (w, h, rgba) = decode(&png);

        let px = |x: u32, y: u32| {
            let i = ((y * w + x) * 4) as usize;
            (rgba[i], rgba[i + 1], rgba[i + 2], rgba[i + 3])
        };

        let upper = px(w / 2, (h / 2) - 8);
        let lower = px(w / 2, (h / 2) + 8);
        assert!(upper.3 > 0 && lower.3 > 0, "both tracks must be drawn");
        // Cyan above (blue dominant), amber below (red dominant).
        assert!(upper.2 > upper.0, "download track should read cyan");
        assert!(lower.0 > lower.2, "upload track should read amber");
    }

    #[test]
    fn tracks_share_one_scale() {
        // Upload is a tenth of download, so it must stay visibly shorter.
        let hist: Vec<HistorySample> = (0..40)
            .map(|_| HistorySample::from_bps(1_000_000, 100_000, 500_000_000))
            .collect();
        let png = render_unified_dual_chart_png(&hist, 40, 60, 40).unwrap();
        let (w, h, rgba) = decode(&png);
        let alpha = |x: u32, y: u32| rgba[((y * w + x) * 4 + 3) as usize];

        let x = w / 2;
        let down_height = (0..h / 2).filter(|&y| alpha(x, y) > 0).count();
        let up_height = (h / 2..h).filter(|&y| alpha(x, y) > 0).count();
        assert!(
            down_height > up_height * 2,
            "shared scale expected: down={} up={}",
            down_height,
            up_height
        );
    }

    #[test]
    fn idle_traffic_keeps_both_rails_visible() {
        let hist: Vec<HistorySample> = (0..40)
            .map(|_| HistorySample::from_bps(0, 0, 500_000_000))
            .collect();
        let png = render_unified_dual_chart_png(&hist, 40, 60, 40).unwrap();
        let (w, h, rgba) = decode(&png);
        let px = |x: u32, y: u32| {
            let i = ((y * w + x) * 4) as usize;
            (rgba[i], rgba[i + 1], rgba[i + 2], rgba[i + 3])
        };
        let x = w / 2;
        // Somewhere just above the axis must read cyan, just below must read amber.
        let above = (0..h / 2).map(|y| px(x, y)).find(|p| p.3 > 40);
        let below = (h / 2..h).map(|y| px(x, y)).find(|p| p.3 > 40);
        let above = above.expect("idle download rail missing");
        let below = below.expect("idle upload rail missing");
        assert!(above.2 > above.0, "idle download rail should read cyan");
        assert!(below.0 > below.2, "idle upload rail should read amber");
    }

    #[test]
    fn single_track_chart_renders() {
        let png = render_chart_png(&history(30), 30, 100, 20, Track::Upload).unwrap();
        let (w, h, _) = decode(&png);
        assert_eq!((w, h), (100, 20));
    }

    #[test]
    fn data_uri_helpers_produce_png_uris() {
        let uri = render_unified_chart_data_uri(
            &history(60),
            "Medium",
            30,
            ResolvedTheme::Dark,
            GraphStyle::Area,
        );
        assert!(uri.starts_with("data:image/png;base64,"));
        let (rx, tx) = render_chart_data_uris(&history(60), "Small", 15);
        assert!(rx.starts_with("data:image/png;base64,"));
        assert!(tx.starts_with("data:image/png;base64,"));
    }

    #[test]
    fn smoothing_preserves_peak() {
        let raw = vec![0.0, 0.0, 100.0, 0.0, 0.0];
        let smoothed = apply_fluid_wave_smoothing(&raw);
        let peak = smoothed.iter().cloned().fold(0.0f64, f64::max);
        assert!((peak - 100.0).abs() < 1e-6);
    }

    #[test]
    fn smoothing_anchors_leading_edge_to_zero() {
        // Even when preceded by heavy traffic, an idle leading edge must remain 0.0
        let raw = vec![0.0, 50.0, 100.0, 80.0, 0.0];
        let smoothed = apply_fluid_wave_smoothing(&raw);
        assert_eq!(smoothed.last().copied(), Some(0.0));
    }

    #[test]
    fn windowing_pads_short_history_at_the_front() {
        let padded = windowed(&[5.0, 6.0], 6);
        assert_eq!(padded, vec![0.0, 0.0, 0.0, 0.0, 5.0, 6.0]);
        // And trims when history is longer than the window.
        let trimmed = windowed(&[1.0, 2.0, 3.0, 4.0], 2);
        assert_eq!(trimmed, vec![3.0, 4.0]);
    }

    #[test]
    fn chart_window_uses_sampling_cadence_on_every_size() {
        for size in ["Small", "Medium", "Large"] {
            assert_eq!(ChartDimensions::for_size(size, 15).sample_count, 60);
            assert_eq!(ChartDimensions::for_size(size, 30).sample_count, 120);
            assert_eq!(ChartDimensions::for_size(size, 60).sample_count, 240);
            assert_eq!(
                ChartDimensions::for_size(size, 60).sample_count,
                crate::HISTORY_CAPACITY
            );
            // Invalid windows fall back to 30s.
            assert_eq!(ChartDimensions::for_size(size, 45).sample_count, 120);
        }
    }

    #[test]
    fn test_blend_alpha_edge_cases() {
        let test_alphas = [0.0f32, 1e-6f32, 1.0 / 255.0, 0.5, 254.0 / 255.0, 1.0];
        let test_colors = [
            Rgba(255, 0, 0, 255),
            Rgba(0, 255, 0, 128),
            Rgba(0, 0, 255, 1),
            Rgba(255, 255, 255, 0),
            Rgba(100, 150, 200, 50),
        ];

        let mut c_opt = Canvas::new(10, 10);
        let mut pixels_ref = vec![0u8; 400];

        // Reference scalar blend function
        let blend_ref = |pixels: &mut [u8], x: u32, y: u32, color: Rgba, alpha_scale: f32| {
            let a = (color.3 as f32 * alpha_scale.clamp(0.0, 1.0)) / 255.0;
            if a <= 0.0 {
                return;
            }
            let idx = ((y * 10 + x) * 4) as usize;
            let dst_a = pixels[idx + 3] as f32 / 255.0;
            let out_a = a + dst_a * (1.0 - a);
            if out_a <= 0.0 {
                return;
            }
            for (offset, src) in [(0, color.0), (1, color.1), (2, color.2)] {
                let dst = pixels[idx + offset] as f32;
                let val = (src as f32 * a + dst * dst_a * (1.0 - a)) / out_a;
                pixels[idx + offset] = val.round().clamp(0.0, 255.0) as u8;
            }
            pixels[idx + 3] = (out_a * 255.0).round().clamp(0.0, 255.0) as u8;
        };

        // Pass 1: Transparent destination
        for (i, &a) in test_alphas.iter().enumerate() {
            for (j, &color) in test_colors.iter().enumerate() {
                let x = (i % 10) as u32;
                let y = (j % 10) as u32;
                c_opt.blend(x, y, color, a);
                blend_ref(&mut pixels_ref, x, y, color, a);
                assert_eq!(c_opt.pixels, pixels_ref);
            }
        }

        // Pass 2: Semi-transparent and opaque destination
        for (i, &a) in test_alphas.iter().enumerate() {
            for (j, &color) in test_colors.iter().enumerate() {
                let x = (i % 10) as u32;
                let y = (j % 10) as u32;
                c_opt.blend(x, y, color, a);
                blend_ref(&mut pixels_ref, x, y, color, a);
                assert_eq!(c_opt.pixels, pixels_ref);
            }
        }
    }

    #[test]
    fn test_downsample_differential_deterministic() {
        let reference_downsample = |w: u32, h: u32, pixels: &[u8], factor: u32| -> Vec<u8> {
            let out_w = w / factor;
            let out_h = h / factor;
            let mut out = vec![0u8; (out_w * out_h * 4) as usize];
            let samples = (factor * factor) as f32;
            for oy in 0..out_h {
                for ox in 0..out_w {
                    let (mut r, mut g, mut b, mut a) = (0.0f32, 0.0f32, 0.0f32, 0.0f32);
                    for dy in 0..factor {
                        for dx in 0..factor {
                            let idx = (((oy * factor + dy) * w + ox * factor + dx) * 4) as usize;
                            let pa = pixels[idx + 3] as f32 / 255.0;
                            r += pixels[idx] as f32 * pa;
                            g += pixels[idx + 1] as f32 * pa;
                            b += pixels[idx + 2] as f32 * pa;
                            a += pa;
                        }
                    }
                    let o = ((oy * out_w + ox) * 4) as usize;
                    if a > 0.0 {
                        out[o] = (r / a).round().clamp(0.0, 255.0) as u8;
                        out[o + 1] = (g / a).round().clamp(0.0, 255.0) as u8;
                        out[o + 2] = (b / a).round().clamp(0.0, 255.0) as u8;
                        out[o + 3] = ((a / samples) * 255.0).round().clamp(0.0, 255.0) as u8;
                    }
                }
            }
            out
        };

        let mut rng: u64 = 0xabcdef01_23456789;
        let mut xor_shift = || {
            rng ^= rng << 13;
            rng ^= rng >> 7;
            rng ^= rng << 17;
            rng
        };

        let w = 12;
        let h = 12;
        for iter in 0..100 {
            let mut c = Canvas::new(w, h);
            let factor = if iter % 2 == 0 { 3 } else { 2 };
            for i in 0..(w * h) as usize {
                let val = xor_shift();
                let alpha = match val % 4 {
                    0 => 0u8,
                    1 => 255u8,
                    _ => (val & 0xFF) as u8,
                };
                c.pixels[i * 4] = ((val >> 8) & 0xFF) as u8;
                c.pixels[i * 4 + 1] = ((val >> 16) & 0xFF) as u8;
                c.pixels[i * 4 + 2] = ((val >> 24) & 0xFF) as u8;
                c.pixels[i * 4 + 3] = alpha;
            }

            let (_, _, opt_bytes) = c.downsample(factor);
            let ref_bytes = reference_downsample(w, h, &c.pixels, factor);
            assert_eq!(
                opt_bytes, ref_bytes,
                "Mismatch on random iteration {}",
                iter
            );
        }
    }

    #[test]
    fn test_idle_chart_caching() {
        let empty_history: Vec<HistorySample> = Vec::new();
        let uri1 = render_unified_chart_data_uri(
            &empty_history,
            "Medium",
            60,
            ResolvedTheme::Dark,
            GraphStyle::Area,
        );
        let uri2 = render_unified_chart_data_uri(
            &empty_history,
            "Medium",
            60,
            ResolvedTheme::Dark,
            GraphStyle::Area,
        );
        assert!(!uri1.is_empty());
        assert_eq!(uri1, uri2);

        // All zero-rate history is also served from idle cache
        let zero_traffic: Vec<HistorySample> = (0..60)
            .map(|_| HistorySample::from_bps(0, 0, 250_000_000))
            .collect();
        let uri3 = render_unified_chart_data_uri(
            &zero_traffic,
            "Medium",
            60,
            ResolvedTheme::Dark,
            GraphStyle::Area,
        );
        assert_eq!(uri1, uri3);

        // Light theme idle chart is distinct from Dark theme idle chart
        let uri_light = render_unified_chart_data_uri(
            &empty_history,
            "Medium",
            60,
            ResolvedTheme::Light,
            GraphStyle::Area,
        );
        assert!(!uri_light.is_empty());
        assert_ne!(
            uri1, uri_light,
            "Light and Dark idle charts must have distinct palette rendering"
        );
    }

    #[test]
    fn test_theme_resolution_and_fallback() {
        assert_eq!(ThemeMode::Dark.resolve(), ResolvedTheme::Dark);
        assert_eq!(ThemeMode::Light.resolve(), ResolvedTheme::Light);
        // Auto resolves to either Dark or Light without panicking
        let auto_resolved = ThemeMode::Auto.resolve();
        assert!(matches!(
            auto_resolved,
            ResolvedTheme::Dark | ResolvedTheme::Light
        ));
    }

    #[test]
    fn test_graph_styles_render() {
        let hist = history(60);
        for theme in [ResolvedTheme::Dark, ResolvedTheme::Light] {
            for style in [GraphStyle::Area, GraphStyle::Line, GraphStyle::Bar] {
                let uri = render_unified_chart_data_uri(&hist, "Medium", 30, theme, style);
                assert!(
                    uri.starts_with("data:image/png;base64,"),
                    "Failed to render style {:?} with theme {:?}",
                    style,
                    theme
                );
            }
        }
    }
}
