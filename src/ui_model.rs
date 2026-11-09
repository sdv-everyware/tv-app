//! tv-app: the UI model, one field per layer of #tv-root the app writes to or taps (design:
//! design/tv-patch.js). The names are the contract with the
//! design: rename both or neither.

use everyware_ui::model::prelude::*;

everyware_ui::ui_model! {
    time: String = "#tv/time";
    greeting: String = "#tv/greeting";
    date: String = "#tv/date";
    weather_icon: Slot = "#tv/weather/icon" ["SlotCustomizer"];
    weather_temp: String = "#tv/weather/temp";
    weather_condition: String = "#tv/weather/condition";
    cards: Slot = "#tv/cards" interactive ["SlotCustomizer"];
    nav_home: Slot = "#tv/nav/home" ["SlotCustomizer"];
    nav_scenes: Slot = "#tv/nav/scenes" ["SlotCustomizer"];
    nav_wifi: Slot = "#tv/nav/wifi" ["SlotCustomizer"];
    nav_settings: Slot = "#tv/nav/settings" ["SlotCustomizer"];
}
