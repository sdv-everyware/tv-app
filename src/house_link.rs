//! tv-app: the house's weather (the news are not on this screen). The dashboard bundle
//! (vendor/sdv-everyware/everyware/bundles/dashboard) pushes the house's data over binder to
//! com.everyware.screen.IScreenLink/default, which the panel app serves; the TV screen takes the
//! same name (one of the two runs at a time) through the shared link
//! (vendor/sdv-everyware/everyware/interfaces/screen/link). Everything but the weather is
//! dropped by the link's defaults.

use crate::actions::TvAction;
use everyware_catalog::weather::Weather;
use everyware_screen_link::Screen;
use harry::reducer::Actions;
use std::sync::mpsc::Sender;
use std::sync::Mutex;

struct Tv {
    actions: Mutex<Sender<Actions<TvAction>>>,
}

impl Screen for Tv {
    fn weather(&self, weather: Weather) {
        let _ = self.actions.lock().expect("actions").send(Actions::CustomAction(TvAction::Weather(weather)));
    }
}

/// Serves the link until the process ends; returns at once.
pub fn spawn(actions: Sender<Actions<TvAction>>) {
    everyware_screen_link::spawn(Tv { actions: Mutex::new(actions) });
}
