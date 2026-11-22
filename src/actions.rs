//! tv-app: the clock's tick, the weather, a gesture on a node. A tap on a card chooses it (the
//! ribbon lifts it); the control inside the card changes its device: the toggle flips, the
//! dimmer follows the finger, the fan takes the tapped mode, the climate steps on minus and
//! plus. The card's value text shows it.

use crate::clock::Clock;
use crate::state::State;
use everyware_catalog::weather::Weather;
use everyware_ui::button::ButtonEvent;
use everyware_ui::list_view::ListEvent;
use everyware_ui::tile::{Tile, TileEvent};
use everyware_ui::{part_of, row_of, Gesture};
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

const CARDS: &str = "#tv/cards";

fn on_gesture(state: &mut State, node: &str, gesture: Gesture) {
    if matches!(gesture, Gesture::Down { .. }) {
        log::info!("tv: down on {node}");
    }
    if node == CARDS || node.starts_with("#tv/cards/") {
        let now = Instant::now();
        let row = row_of(CARDS, node);
        // a control inside a card, "#tv/cards/<n>/<part>": the device, not the ribbon; the
        // card it is in is chosen as well (it lifts), as a tap on the card would do
        if let (Some(part), Some(i)) = (part_of(CARDS, node), row) {
            if matches!(gesture, Gesture::Down { .. }) {
                state.cards.select(Some(i));
            }
            let x = state.ribbon_x + state.cards.row_offset(i, now);
            if let Some(card) = state.cards.row_mut(i) {
                if let Some(event) = card.on_part(Some(part), gesture, x, 0.0) {
                    show(card, event);
                    log::info!("tv: card {i} {}: {}", card.title, card.value);
                }
            }
            return;
        }
        // the ribbon (a drag sideways that starts between cards) or a card: a tap chooses it
        if let Some(ListEvent::Chosen(i)) = state.cards.on(row, gesture, now) {
            state.cards.select(Some(i));
            log::info!("tv: card {i} chosen");
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

/// The card's value text after its control changed the device.
fn show(card: &mut Tile, event: TileEvent) {
    match event {
        TileEvent::Opened => {}
        TileEvent::Toggled(on) => card.value = if on { "On" } else { "Off" }.to_string(),
        TileEvent::Moving(level) | TileEvent::Level(level) => card.value = format!("{level:.0} %"),
        TileEvent::Mode(_) => {}
        TileEvent::Step(by) => {
            let t: f32 = card.value.trim_end_matches('°').parse().unwrap_or(22.0);
            let t = (t + 0.5 * by as f32).clamp(18.0, 26.0);
            card.value = format!("{t:.1}°").replace(".0°", "°");
        }
    }
}
