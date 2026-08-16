//! launcher — starts jarvis and hacker_screen together, placing jarvis on
//! the topmost monitor and hacker_screen on the bottommost monitor of a
//! vertically-stacked multi-monitor setup. No window of its own; it exits
//! once both child windows are positioned.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::Command;
use std::thread::sleep;
use std::time::{Duration, Instant};

use windows::core::PCWSTR;
use windows::Win32::Foundation::{BOOL, HWND, LPARAM, RECT};
use windows::Win32::Graphics::Gdi::{EnumDisplayMonitors, GetMonitorInfoW, HDC, HMONITOR, MONITORINFO};
use windows::Win32::UI::WindowsAndMessaging::{FindWindowW, SetWindowPos, HWND_TOP, SWP_NOZORDER, SWP_SHOWWINDOW};

struct MonitorRect {
    left: i32,
    top: i32,
    width: i32,
    height: i32,
}

unsafe extern "system" fn monitor_enum_proc(
    hmonitor: HMONITOR,
    _hdc: HDC,
    _rect: *mut RECT,
    lparam: LPARAM,
) -> BOOL {
    unsafe {
        let monitors = &mut *(lparam.0 as *mut Vec<MonitorRect>);
        let mut info = MONITORINFO {
            cbSize: std::mem::size_of::<MONITORINFO>() as u32,
            ..Default::default()
        };
        if GetMonitorInfoW(hmonitor, &mut info).as_bool() {
            let r = info.rcMonitor;
            monitors.push(MonitorRect {
                left: r.left,
                top: r.top,
                width: r.right - r.left,
                height: r.bottom - r.top,
            });
        }
        BOOL(1)
    }
}

fn enum_monitors() -> Vec<MonitorRect> {
    let mut monitors: Vec<MonitorRect> = Vec::new();
    unsafe {
        let _ = EnumDisplayMonitors(
            HDC(std::ptr::null_mut()),
            None,
            Some(monitor_enum_proc),
            LPARAM(&mut monitors as *mut _ as isize),
        );
    }
    monitors
}

fn wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

fn find_window(title: &str, timeout: Duration) -> Option<HWND> {
    let wide_title = wide(title);
    let deadline = Instant::now() + timeout;
    loop {
        if let Ok(hwnd) = unsafe { FindWindowW(PCWSTR::null(), PCWSTR(wide_title.as_ptr())) } {
            if !hwnd.0.is_null() {
                return Some(hwnd);
            }
        }
        if Instant::now() >= deadline {
            return None;
        }
        sleep(Duration::from_millis(150));
    }
}

fn place_window(hwnd: HWND, mon: &MonitorRect) {
    unsafe {
        let _ = SetWindowPos(
            hwnd,
            HWND_TOP,
            mon.left,
            mon.top,
            mon.width,
            mon.height,
            SWP_NOZORDER | SWP_SHOWWINDOW,
        );
    }
}

fn main() {
    let mut monitors = enum_monitors();
    if monitors.is_empty() {
        return;
    }
    monitors.sort_by_key(|m| m.top);
    let top = monitors.first().unwrap();
    let bottom = monitors.last().unwrap();

    let exe_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .unwrap_or_default();

    let _ = Command::new(exe_dir.join("jarvis.exe")).spawn();
    let _ = Command::new(exe_dir.join("hacker_screen.exe")).spawn();

    if let Some(hwnd) = find_window("JARVIS", Duration::from_secs(6)) {
        place_window(hwnd, top);
    }
    if let Some(hwnd) = find_window("SYSTEM BREACH", Duration::from_secs(6)) {
        place_window(hwnd, bottom);
    }
}
