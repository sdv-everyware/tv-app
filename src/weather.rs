//! tv-app: the weather on this screen. The dashboard bundle sends the data (a WMO weather
//! code, temperatures, the chance of rain); what it looks like is decided here: which picture a
//! code gets, its words. The pictures are this app's own component set
//! (#tv-weather, design/weather-pictures.js), not a component of the UI library: the same
//! kinds in both.

use everyware_ui::{Change, Component, ComponentKind, Slotted};

/// A picture of #tv-weather: seven kinds of weather.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Picture {
    Clear,
    Partly,
    Cloudy,
    Fog,
    Rain,
    Snow,
    Thunder,
}

impl Picture {
    /// The picture for a WMO weather interpretation code (the bundle's `weather_code`).
    pub fn of(code: u32) -> Self {
        match code {
            0 | 1 => Picture::Clear,
            2 => Picture::Partly,
            3 => Picture::Cloudy,
            45 | 48 => Picture::Fog,
            51..=67 | 80..=82 => Picture::Rain,
            71..=77 | 85 | 86 => Picture::Snow,
            95..=99 => Picture::Thunder,
            _ => Picture::Cloudy,
        }
    }
}

impl Component for Picture {
    const KIND: ComponentKind = ComponentKind::App;

    fn variant(&self) -> String {
        let kind = match self {
            Picture::Clear => "clear",
            Picture::Partly => "partly",
            Picture::Cloudy => "cloudy",
            Picture::Fog => "fog",
            Picture::Rain => "rain",
            Picture::Snow => "snow",
            Picture::Thunder => "thunder",
        };
        format!("kind={kind}")
    }

    fn last_change(&self) -> Option<Change> {
        None
    }
}

impl Slotted for Picture {
    const SET: &'static str = "#tv-weather";
}

/// A WMO weather interpretation code as words, as the screen shows it.
pub fn words(code: u32) -> &'static str {
    match code {
        0 => "Clear",
        1 => "Mainly clear",
        2 => "Partly cloudy",
        3 => "Overcast",
        45 | 48 => "Fog",
        51 | 53 | 55 => "Drizzle",
        56 | 57 => "Freezing drizzle",
        61 | 63 | 65 => "Rain",
        66 | 67 => "Freezing rain",
        71 | 73 | 75 | 77 => "Snow",
        80..=82 => "Rain showers",
        85 | 86 => "Snow showers",
        95 => "Thunderstorm",
        96 | 99 => "Thunderstorm with hail",
        _ => "Unknown",
    }
}
