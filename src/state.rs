//! tv-app: everything the screen knows: the clock, the weather, the ribbon of device cards
//! (a horizontal ListView of #tile) and the round buttons.

use crate::clock::Clock;
use everyware_dashboard_grpc::weather::Weather;
use everyware_ui::button::{Icon, IconButton, IconButtonKind};
use everyware_ui::list_view::ListView;
use everyware_ui::segmented::Segmented;
use everyware_ui::slider::Slider;
use everyware_ui::symbol::SymbolIcon;
use everyware_ui::tile::{Tile, TileKind};

/// The ribbon's slot (tv-patch.js): 1728 wide, cards 438 wide and 24 apart.
pub const RIBBON_W: f32 = 1728.0;
pub const CARD_PITCH: f32 = 438.0 + 24.0;

pub struct State {
    pub clock: Clock,
    pub weather: Option<Weather>,
    pub cards: ListView<Tile>,
    pub nav_home: IconButton,
    pub nav_scenes: IconButton,
    pub nav_wifi: IconButton,
    pub nav_settings: IconButton,
}

impl State {
    pub fn new() -> Self {
        State {
            clock: Clock::default(),
            weather: None,
            cards: ListView::new(cards(), RIBBON_W, CARD_PITCH).horizontal(),
            nav_home: IconButton::new(IconButtonKind::Tonal, Icon::House),
            nav_scenes: IconButton::new(IconButtonKind::Tonal, Icon::Sliders),
            nav_wifi: IconButton::new(IconButtonKind::Tonal, Icon::Wifi),
            nav_settings: IconButton::new(IconButtonKind::Tonal, Icon::Settings),
        }
    }
}

/// The house's devices as cards: more than fit, so the ribbon scrolls.
fn cards() -> Vec<Tile> {
    let toggle = |symbol, title: &str, room: &str, on: bool| {
        let mut t = Tile::new(TileKind::Toggle, symbol, title, room).value(if on { "On" } else { "Off" });
        t.toggle.value = on;
        t
    };
    let slider = |symbol, title: &str, room: &str, level: f32| {
        let mut t = Tile::new(TileKind::Slider, symbol, title, room).value(format!("{level:.0} %"));
        t.slider = Slider::new(level, 0.0, 1.0);
        t
    };
    let mut fan = Tile::new(TileKind::Segmented, SymbolIcon::Armchair, "Fan", "Office");
    fan.segmented = Segmented::new(["Low", "Med", "High"]);
    vec![
        toggle(SymbolIcon::House, "Ceiling light", "Living room", true),
        slider(SymbolIcon::BedDouble, "Dimmer", "Bedroom", 65.0),
        fan,
        Tile::new(TileKind::Stepper, SymbolIcon::Droplets, "Climate", "Whole house").value("22°"),
        toggle(SymbolIcon::DoorOpen, "Front door", "Entrance", true),
        slider(SymbolIcon::House, "Floor lamp", "Living room", 30.0),
        toggle(SymbolIcon::BedDouble, "Night light", "Kids' room", false),
    ]
}
