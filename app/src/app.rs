use crate::app::widgets::toggle_button::ToggleButton;
use crate::app::windows::{AppWindow, Windows};
use eframe::Storage;
use egui::{Context, FontDefinitions, TopBottomPanel, Widget};
use egui_notify::Toasts;
pub use egui_phosphor::regular as icons;
use windows::ActiveWindows;

mod widgets;
pub mod windows;

#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct App {
    #[serde(skip, default)]
    toasts: Toasts,
    #[serde(skip, default)]
    windows: Windows,
}

impl App {
    pub fn new(cc: &eframe::CreationContext) -> Self {
        cc.egui_ctx.set_pixels_per_point(1.5);
        Self::setup_fonts(&cc.egui_ctx);
        cc.storage
            .and_then(|storage| eframe::get_value::<Self>(storage, eframe::APP_KEY))
            .unwrap_or_default()
    }

    fn setup_fonts(ctx: &Context) {
        let mut fonts = FontDefinitions::default();
        egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);
        ctx.set_fonts(fonts);
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        TopBottomPanel::top("top_panel").show(ctx, |ui| self.show_top_panel(ui));
        self.toasts.show(ctx);
        self.show_windows(ctx);
    }

    fn save(&mut self, storage: &mut dyn Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}

// Rendering
impl App {
    fn show_top_panel(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Lemon PKMN");

            ui.separator();

            self.window_toggle_button(ui, ActiveWindows::PKMN_LIST);
        });
    }
}

// Windows
impl App {
    fn show_windows(&mut self, ctx: &Context) {
        windows::pkmn_list::PkmnListWindow.show(ctx, self);
    }

    pub fn is_window_active(&self, window: ActiveWindows) -> bool {
        self.windows.active.contains(window)
    }

    pub fn set_window_active(&mut self, window: ActiveWindows, active: bool) {
        self.windows.active.set(window, active);
    }

    pub fn window_toggle_button(&mut self, ui: &mut egui::Ui, window: ActiveWindows) {
        let mut is_open = self.is_window_active(window);
        ToggleButton::new(&mut is_open, window.icon()).ui(ui);
        self.set_window_active(window, is_open);
    }
}
