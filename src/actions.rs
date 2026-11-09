//! tv-app: the clock's tick, the weather, a gesture on a node. A tap on a card chooses it (the
//! ribbon lifts it) and steps its device: a light flips, a dimmer goes up a quarter, the fan
//! takes the next mode, the climate half a degree; the card's control shows it.

use crate::clock::Clock;
use crate::state::State;
use everyware_dashboard_grpc::weather::Weather;
use everyware_ui::button::ButtonEvent;
use everyware_ui::list_view::ListEvent;
use everyware_ui::tile::{Tile, TileKind};
use everyware_ui::{row_of, Gesture};
use harry::side_effects::SideEffects;
use harry::state::StateAction;
use std::time::Instant;

#[derive(Clone, Debug, PartialEq)]
pub enum TvAction {
    /// Once a second from the ticker thread (main.rs).
    Tick(Clock),
    /// A gesture on a node (ui_event_listener.rs).
    Gesture(String, Gesture),
    /// The weather of the house (house_link.rs).
    Weather(Weather),
}

impl StateAction<State, ()> for TvAction {
    fn apply(&self, state: &mut State, _side_effects: &mut Vec<SideEffects<()>>) {
        match self {
            TvAction::Tick(now) => state.clock = now.clone(),
            TvAction::Weather(w) => state.weather = Some(w.clone()),
            TvAction::Gesture(node, gesture) => on_gesture(state, node, *gesture),
        }
    }
}

fn on_gesture(state: &mut State, node: &str, gesture: Gesture) {
    if matches!(gesture, Gesture::Down { .. }) {
        log::info!("tv: down on {node}");
    }
    // the ribbon: the slot (a drag sideways that starts between cards) or a card, "#tv/cards/<n>"
    if node == "#tv/cards" || node.starts_with("#tv/cards/") {
        if let Some(ListEvent::Chosen(i)) = state.cards.on(row_of("#tv/cards", node), gesture, Instant::now()) {
            state.cards.select(Some(i));
            if let Some(card) = state.cards.row_mut(i) {
                step(card);
                log::info!("tv: card {i} {}: {}", card.title, card.value);
            }
        }
        return;
    }
    let button = match node {
        "#tv/nav/home" => &mut state.nav_home,
        "#tv/nav/scenes" => &mut state.nav_scenes,
        "#tv/nav/wifi" => &mut state.nav_wifi,
        "#tv/nav/settings" => &mut state.nav_settings,
        _ => return,
    };
    if button.on(gesture) == Some(ButtonEvent::Clicked) {
        log::info!("tv: {node}");
    }
}

/// The device of a card one step on.
fn step(card: &mut Tile) {
    match card.kind {
        TileKind::Toggle => {
            card.toggle.value = !card.toggle.value;
            card.value = if card.toggle.value { "On" } else { "Off" }.to_string();
        }
        TileKind::Slider => {
            card.slider.value = if card.slider.value >= 100.0 { 0.0 } else { (card.slider.value + 25.0).min(100.0) };
            card.value = format!("{:.0} %", card.slider.value);
        }
        TileKind::Segmented => {
            let next = (card.segmented.labels.iter().position(|l| l == card.segmented.value()).unwrap_or(0) + 1) % card.segmented.labels.len();
            card.segmented.select(next);
        }
        TileKind::Stepper => {
            let t: f32 = card.value.trim_end_matches('°').parse().unwrap_or(22.0);
            let t = if t >= 26.0 { 18.0 } else { t + 0.5 };
            card.value = format!("{t:.1}°").replace(".0°", "°");
        }
        TileKind::Image => {}
    }
}
