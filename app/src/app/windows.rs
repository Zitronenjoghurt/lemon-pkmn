use crate::app::{icons, App};
use bitflags::{bitflags, bitflags_match};
use egui::{Context, Id, Ui, Widget, WidgetText};

pub mod pkmn_list;

#[derive(Default)]
pub struct Windows {
    pub active: ActiveWindows,
}

impl Windows {}

bitflags! {
    #[derive(Default, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct ActiveWindows: u8 {
        const PKMN_LIST = 0b0000_0001;
        const PKMN_GENERATOR = 0b0000_0010;
    }
}

impl ActiveWindows {
    pub fn icon(&self) -> &'static str {
        bitflags_match!(*self, {
            Self::PKMN_LIST => icons::LIST_DASHES,
            Self::PKMN_GENERATOR => icons::FACTORY,
            _ => ""
        })
    }
}

pub trait AppWindow: Sized {
    const ID: &'static str;
    const ACTIVE_FLAG: ActiveWindows;

    fn title(_app: &mut App) -> impl Into<WidgetText>;

    fn resizable(_app: &mut App) -> bool {
        true
    }

    fn movable(_app: &mut App) -> bool {
        true
    }

    fn collapsible(_app: &mut App) -> bool {
        true
    }

    fn ui(&mut self, ui: &mut Ui, app: &mut App);

    fn show(mut self, ctx: &Context, app: &mut App) {
        let mut is_open = app.is_window_active(Self::ACTIVE_FLAG);
        egui::Window::new(Self::title(app))
            .id(Id::new(Self::ID))
            .open(&mut is_open)
            .resizable(Self::resizable(app))
            .movable(Self::movable(app))
            .collapsible(Self::collapsible(app))
            .show(ctx, |ui| self.ui(ui, app));
        app.set_window_active(Self::ACTIVE_FLAG, is_open);
    }
}
