//! tv-app: state -> UI model, after every action.

use crate::state::State;
use crate::ui_model::UiModel;
use crate::weather::{words, Picture};
use everyware_ui::{list, slot};
use harry::presentation::Presenter;

pub struct UiModelPresenter;

impl Presenter<State, UiModel> for UiModelPresenter {
    fn present(model: &mut UiModel, state: &State) {
        model.time = state.clock.time.clone();
        model.date = state.clock.date.clone();
        model.greeting = greeting(&state.clock.time).to_string();
        match &state.weather {
            Some(w) => {
                model.weather_icon = slot("#tv/weather/icon", &Picture::of(w.code));
                model.weather_temp = format!("{:.0}°", w.temperature_c);
                model.weather_condition = format!("{} in {}", words(w.code), w.place);
            }
            None => {
                model.weather_temp = "–".to_string();
                model.weather_condition = "Waiting for the weather".to_string();
            }
        }
        model.cards = list("#tv/cards", &state.cards);
        model.nav_home = slot("#tv/nav/home", &state.nav_home);
        model.nav_scenes = slot("#tv/nav/scenes", &state.nav_scenes);
        model.nav_wifi = slot("#tv/nav/wifi", &state.nav_wifi);
        model.nav_settings = slot("#tv/nav/settings", &state.nav_settings);
    }
}

/// The greeting for the hour of "HH:MM".
fn greeting(time: &str) -> &'static str {
    match time.get(..2).and_then(|h| h.parse::<u32>().ok()) {
        Some(5..=11) => "Good morning",
        Some(12..=17) => "Good afternoon",
        Some(18..=22) => "Good evening",
        _ => "Good night",
    }
}
