use crate::app::App;
use crate::directories::eframe_save_file_path;

mod app;
mod directories;

fn main() {
    let native_options = eframe::NativeOptions {
        renderer: eframe::Renderer::Wgpu,
        viewport: egui::ViewportBuilder::default()
            .with_maximized(true)
            .with_title("Lemon PKMN")
            .with_app_id("io.github.zitronenjoghurt.lemon-pkmn"),
        persist_window: true,
        persistence_path: Some(eframe_save_file_path()),
        ..Default::default()
    };

    eframe::run_native(
        "Lemon PKMN",
        native_options,
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    )
    .expect("Failed to run egui application.");
}
