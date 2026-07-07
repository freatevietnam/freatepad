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

fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    load_font(&mut fonts, "dejavu_mono", "/usr/share/fonts/truetype/dejavu/DejaVuSansMono.ttf");
    load_font(&mut fonts, "dejavu_sans", "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf");

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

    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "dejavu_sans".to_owned());

    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .insert(0, "dejavu_mono".to_owned());

    ctx.set_fonts(fonts);
}

fn load_font(fonts: &mut egui::FontDefinitions, name: &str, path: &str) {
    if let Ok(font_data) = std::fs::read(path) {
        fonts.font_data.insert(
            name.to_owned(),
            Arc::new(egui::FontData::from_owned(font_data)),
        );
    }
}
