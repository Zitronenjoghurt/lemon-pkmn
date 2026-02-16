use crate::app::windows::{ActiveWindows, AppWindow};
use crate::app::App;
use egui::{Ui, WidgetText};

pub struct PkmnListWindow;

impl AppWindow for PkmnListWindow {
    const ID: &'static str = "pkmn_list";
    const ACTIVE_FLAG: ActiveWindows = ActiveWindows::PKMN_LIST;

    fn title(_app: &mut App) -> impl Into<WidgetText> {
        "Pokémon List"
    }

    fn ui(&mut self, _ui: &mut Ui, _app: &mut App) {}
}
