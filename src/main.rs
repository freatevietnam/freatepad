#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod editor;
mod file;
mod preview;
mod settings;
mod ui;

use std::sync::Arc;

use app::FreatePad;

fn main() -> eframe::Result<()> {
    install_panic_hook();
    env_logger::init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([600.0, 400.0])
            .with_title("FreatePad"),
        ..Default::default()
    };

    eframe::run_native(
        "FreatePad",
        options,
        Box::new(|cc| {
            setup_custom_fonts(&cc.egui_ctx);
            Ok(Box::new(FreatePad::new(cc)))
        }),
    )
}

fn install_panic_hook() {
    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        default_hook(info);
        #[cfg(target_os = "windows")]
        {
            let payload = info.payload();
            let msg = if let Some(s) = payload.downcast_ref::<&str>() {
                s.to_string()
            } else if let Some(s) = payload.downcast_ref::<String>() {
                s.clone()
            } else {
                "Unknown panic".to_string()
            };
            let location = info
                .location()
                .map(|l| format!("\n\n{}:{}: {}", l.file(), l.line(), l.column()))
                .unwrap_or_default();
            let full_msg = format!("FreatePad panicked:{}\n{}", location, msg);
            windows_message_box(&full_msg);
        }
    }));
}

#[cfg(target_os = "windows")]
fn windows_message_box(msg: &str) {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;

    let wide: Vec<u16> = OsStr::new(msg)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();
    let title: Vec<u16> = OsStr::new("FreatePad")
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();

    extern "system" {
        fn MessageBoxW(hWnd: *mut core::ffi::c_void, lpText: *const u16, lpCaption: *const u16, uType: u32) -> i32;
    }

    unsafe {
        MessageBoxW(std::ptr::null_mut(), wide.as_ptr(), title.as_ptr(), 0x10);
    }
}

#[cfg(not(target_os = "windows"))]
#[allow(dead_code)]
fn windows_message_box(_msg: &str) {}

fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    let mut loaded_mono = false;
    let mut loaded_prop = false;

    let font_candidates: &[(&str, &str, bool)] = &[
        // Windows
        ("arial", "C:\\Windows\\Fonts\\arial.ttf", false),
        ("consolas", "C:\\Windows\\Fonts\\consola.ttf", true),
        ("courier", "C:\\Windows\\Fonts\\cour.ttf", true),
        ("times", "C:\\Windows\\Fonts\\times.ttf", false),
        // Linux
        ("dejavu_mono", "/usr/share/fonts/truetype/dejavu/DejaVuSansMono.ttf", true),
        ("dejavu_sans", "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf", false),
    ];

    for &(name, path, is_mono) in font_candidates {
        if load_font(&mut fonts, name, path) {
            if is_mono {
                loaded_mono = true;
            } else {
                loaded_prop = true;
            }
        }
    }

    let noto_paths = [
        "/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc",
        "/usr/share/fonts/noto-cjk/NotoSansCJK-Regular.ttc",
        "/usr/share/fonts/google-noto-cjk/NotoSansCJK-Regular.ttc",
        "/usr/share/fonts/truetype/noto/NotoSansCJK-Regular.ttc",
        "/usr/share/fonts/truetype/noto/NotoSans-Regular.ttf",
        "/usr/share/fonts/noto/NotoSans-Regular.ttf",
        "/usr/share/fonts/truetype/noto/NotoColorEmoji.ttf",
        "/usr/share/fonts/noto-emoji/NotoColorEmoji.ttf",
        "/usr/share/fonts/truetype/noto/NotoSansArabic-Regular.ttf",
        "/usr/share/fonts/noto-arabic/NotoSansArabic-Regular.ttf",
    ];

    for (i, path) in noto_paths.iter().enumerate() {
        let name = format!("noto_{}", i);
        load_font(&mut fonts, &name, path);
    }

    if loaded_prop {
        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(0, "arial".to_owned());
    }

    if loaded_mono {
        fonts
            .families
            .entry(egui::FontFamily::Monospace)
            .or_default()
            .insert(0, "consolas".to_owned());
    }

    ctx.set_fonts(fonts);
}

fn load_font(fonts: &mut egui::FontDefinitions, name: &str, path: &str) -> bool {
    if let Ok(font_data) = std::fs::read(path) {
        fonts.font_data.insert(
            name.to_owned(),
            Arc::new(egui::FontData::from_owned(font_data)),
        );
        true
    } else {
        false
    }
}
