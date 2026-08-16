//! jarvis — a circular holographic HUD in the Iron Man / JARVIS vein:
//! rotating instrument rings, a radar sweep, a pulsing core, live-drifting
//! telemetry gauges, and an audio waveform. No dialogue, no popups — just
//! an ambient instrument panel.
//!
//! Controls: SPACE pings the core (an expanding ripple), ESC quits.

// Rust binaries default to the console subsystem on Windows, which pops up
// a terminal window alongside the graphical one when double-clicked from
// Explorer. Suppress it in release builds; keep it in debug so `cargo run`
// still shows panics/eprintln in the terminal.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod hud;
mod orb;
mod telemetry;
mod text;

use macroquad::prelude::*;
use macroquad::rand::gen_range;

use orb::CodeOrb;
use telemetry::Telemetry;

const NODE_LABELS: &[(f32, &str)] = &[
    (-110.0, "SIGNAL RELAY"),
    (-70.0, "NAV ARRAY"),
    (60.0, "RECON DRONE"),
    (90.0, "COMM UPLINK"),
    (120.0, "DEFENSE GRID"),
];

const RIPPLE_MAX: f32 = 1.0;

fn window_conf() -> Conf {
    Conf {
        window_title: "JARVIS".to_owned(),
        window_width: 1280,
        window_height: 760,
        // miniquad has no windowed-borderless flag on Windows; fullscreen
        // (WS_POPUP) is the only way to drop the title bar.
        fullscreen: true,
        window_resizable: false,
        ..Default::default()
    }
}

/// Try a handful of fonts that ship with Windows, in order of preference.
/// Bahnschrift (a geometric technical sans) reads far more HUD-like than a
/// code-editor monospace font; Segoe UI and Consolas are fallbacks. Falls
/// back to macroquad's default font if none are found.
fn load_system_font() -> Option<Font> {
    const CANDIDATES: &[&str] = &[
        "C:/Windows/Fonts/bahnschrift.ttf",
        "C:/Windows/Fonts/segoeui.ttf",
        "C:/Windows/Fonts/consola.ttf",
    ];
    for path in CANDIDATES {
        if let Ok(bytes) = std::fs::read(path) {
            if let Ok(font) = load_ttf_font_from_bytes(&bytes) {
                return Some(font);
            }
        }
    }
    None
}

fn format_timestamp(t: f64) -> String {
    let total = t as u64;
    format!(
        "{:02}:{:02}:{:02}",
        (total / 3600) % 24,
        (total / 60) % 60,
        total % 60
    )
}

struct App {
    font: Option<Font>,
    telemetry: Telemetry,
    orb: CodeOrb,
    start_time: f64,
    ripple_life: f32,
    highlight_day: usize,
}

impl App {
    fn new(font: Option<Font>) -> Self {
        Self {
            font,
            telemetry: Telemetry::new(),
            orb: CodeOrb::new(220),
            start_time: get_time(),
            ripple_life: 0.0,
            highlight_day: gen_range(1, 29),
        }
    }

    fn update(&mut self, dt: f32) {
        self.telemetry.update(dt);
        self.orb.update(dt);
        if self.ripple_life > 0.0 {
            self.ripple_life -= dt;
        }
        if is_key_pressed(KeyCode::Space) {
            self.ripple_life = RIPPLE_MAX;
        }
    }

    fn draw(&self) {
        let w = screen_width();
        let h = screen_height();
        let t = get_time() as f32;
        let elapsed = get_time() - self.start_time;
        let font = self.font.as_ref();

        clear_background(Color::new(0.0, 0.02, 0.03, 1.0));

        hud::draw_starfield(w, h, t, 180);
        hud::draw_grid(w, h);
        hud::draw_edge_meter(16.0, h * 0.15, h * 0.7, 44, t, 6.0);
        hud::draw_edge_meter(w - 16.0, h * 0.15, h * 0.7, 44, t, -5.0);
        hud::draw_calendar_ribbon(font, w, self.highlight_day);

        let cx = w / 2.0;
        let cy = h / 2.0 + 10.0;
        hud::draw_rings(cx, cy, t);
        hud::draw_sweep(cx, cy, 230.0, t);

        let spoke_color = Color::new(0.35, 0.8, 1.0, 0.5);
        let satellites = [
            (cx - 360.0, cy - 200.0, "RAM", self.telemetry.ram.value),
            (cx + 360.0, cy - 200.0, "THERMAL", self.telemetry.thermal.value),
            (cx - 380.0, cy + 70.0, "COOLANT", self.telemetry.coolant.value),
            (cx + 380.0, cy + 70.0, "AUX PWR", self.telemetry.aux_power.value),
        ];
        for (sx, sy, label, value) in satellites {
            hud::draw_spoke(cx, cy, 230.0, sx, sy, 38.0, spoke_color);
            hud::draw_ring_gauge(font, sx, sy, 38.0, label, value);
        }

        hud::draw_orbiters(cx, cy, t);

        let node_color = Color::new(0.4, 0.85, 1.0, 0.65);
        for (angle, label) in NODE_LABELS {
            hud::draw_node_label(font, cx, cy, *angle, 128.0, 250.0, label, node_color);
        }

        let energy = (self.ripple_life / RIPPLE_MAX).clamp(0.0, 1.0);
        self.orb.draw(font, cx, cy, t, energy);
        hud::draw_ripple(cx, cy, self.ripple_life, RIPPLE_MAX);

        hud::draw_corner_brackets(w, h);

        let left = "JARVIS :: CORE ONLINE";
        text::text(font, left, 20.0, 30.0, 16.0, Color::new(0.4, 0.9, 1.0, 0.85));
        let right = format!("UPTIME {}   STATUS NOMINAL", format_timestamp(elapsed));
        let dims = text::measure(font, &right, 16.0);
        text::text(
            font,
            &right,
            w - dims.width - 20.0,
            30.0,
            16.0,
            Color::new(0.4, 0.8, 0.95, 0.75),
        );
        draw_rectangle(0.0, 40.0, w, 1.0, Color::new(0.25, 0.6, 0.75, 0.35));

        let lx = 40.0;
        let mut ly = h * 0.35;
        for (label, stat) in [
            ("CPU LOAD", &self.telemetry.load),
            ("NETWORK", &self.telemetry.network),
            ("SHIELD INTEGRITY", &self.telemetry.shield),
        ] {
            hud::draw_bar_gauge(font, lx, ly, 140.0, label, stat.value);
            ly += 40.0;
        }

        hud::draw_ring_gauge(font, 110.0, h * 0.68, 46.0, "POWER CORE", self.telemetry.power.value);

        text::text(font, "SYSTEM STATUS", lx, 508.0, 13.0, Color::new(0.5, 0.85, 0.95, 0.85));
        let mut sy = 534.0;
        for (label, value_str) in [
            ("DISK USAGE".to_string(), format!("{:.0}%", self.telemetry.disk_usage.value)),
            ("GPU LOAD".to_string(), format!("{:.0}%", self.telemetry.gpu_load.value)),
            ("CACHE HIT RATE".to_string(), format!("{:.0}%", self.telemetry.cache_rate.value)),
            ("SIGNAL".to_string(), format!("{:.0}%", self.telemetry.signal.value)),
        ] {
            hud::draw_stat_line(font, lx, sy, 180.0, &label, &value_str);
            sy += 26.0;
        }

        let lat_line = format!("LAT {:>8.4} N", self.telemetry.lat);
        let lon_line = format!("LON {:>8.4} W", -self.telemetry.lon);
        let dims1 = text::measure(font, &lat_line, 14.0);
        let dims2 = text::measure(font, &lon_line, 14.0);
        text::text(
            font,
            &lat_line,
            w - dims1.width - 40.0,
            h * 0.4,
            14.0,
            Color::new(0.5, 0.85, 0.95, 0.8),
        );
        text::text(
            font,
            &lon_line,
            w - dims2.width - 40.0,
            h * 0.4 + 20.0,
            14.0,
            Color::new(0.5, 0.85, 0.95, 0.8),
        );

        hud::draw_event_log(font, w - 360.0, 500.0, 280.0, 350.0, "EVENT LOG", &self.telemetry.events);

        hud::draw_waveform(w / 2.0 - 160.0, h - 90.0, 320.0, t, 28);

        hud::draw_sparkline(
            font,
            w - 360.0,
            h - 170.0,
            280.0,
            50.0,
            "NET I/O",
            &self.telemetry.net_history,
        );
        let throughput = format!(
            "UP {:.1}K   DN {:.1}K",
            self.telemetry.net_throughput.value * 0.06,
            self.telemetry.net_throughput.value * 0.21
        );
        text::text(font, &throughput, w - 240.0, h - 178.0, 12.0, Color::new(0.5, 0.85, 0.95, 0.75));

        let ticker_line = format!("\u{bb} {}", self.telemetry.ticker);
        hud::draw_control_bar(font, w / 2.0 - 220.0, h - 58.0, 440.0, 30.0, &ticker_line);

        text::text(
            font,
            "[ESC] Shutdown   [SPACE] Ping",
            40.0,
            h - 16.0,
            13.0,
            Color::new(0.3, 0.6, 0.7, 0.6),
        );

        let wordmark = "AEGIS CORE SYSTEMS";
        let wm_dims = text::measure(font, wordmark, 13.0);
        text::text(
            font,
            wordmark,
            w - wm_dims.width - 40.0,
            h - 16.0,
            13.0,
            Color::new(0.3, 0.6, 0.7, 0.55),
        );
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let font = load_system_font();
    let mut app = App::new(font);
    loop {
        let dt = get_frame_time();
        app.update(dt);
        app.draw();

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await;
    }
}
