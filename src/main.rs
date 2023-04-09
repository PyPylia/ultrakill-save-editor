#![windows_subsystem = "windows"]

use std::io::Cursor;

mod app;
mod class;
mod enums;

const WINDOW_NAME: &str = "ULTRAKILL Save Editor";
const ICON_FILE: &[u8] = include_bytes!("../Icon.ico");

fn main() -> eframe::Result<()> {
    let icon_dir = ico::IconDir::read(Cursor::new(ICON_FILE)).unwrap();
    let mut largest_icon_size = 0;
    let mut largest_icon = &icon_dir.entries()[0];

    for icon in icon_dir.entries() {
        let size = icon.width() as usize * icon.height() as usize;
        if size > largest_icon_size {
            largest_icon_size = size;
            largest_icon = icon;
        }
    }

    let image = largest_icon.decode().unwrap();

    let native_options = eframe::NativeOptions {
        maximized: true,
        icon_data: Some(eframe::IconData {
            rgba: image.rgba_data().to_vec(),
            width: image.width(),
            height: image.height(),
        }),
        ..Default::default()
    };

    eframe::run_native(
        WINDOW_NAME,
        native_options,
        Box::new(|cc| Box::new(app::SaveEditorApp::new(cc))),
    )
}
