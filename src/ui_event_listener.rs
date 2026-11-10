/*
 * tv-app: touch input -> actions. everyware_ui::Input turns harry's touch events into
 * gestures (down, up, cancel) per node, so components can show their pressed state.
 */

use crate::actions::TvAction;
use har_platform_api::looper::{DisplayId, HandleEventStatus, UiEventListener};
use har_platform_api::user_input::UserInputEvent;
use harry::reducer::Actions;
use everyware_ui::{Input, Velocity};
use std::sync::mpsc::Sender;

#[derive(Debug)]
pub struct UiListener {
    actions: Sender<Actions<TvAction>>,
    input: Input,
}

impl UiListener {
    pub fn new(actions: Sender<Actions<TvAction>>, velocity: Velocity) -> Self {
        Self { actions, input: Input::with_velocity(velocity) }
    }
}

impl UiEventListener for UiListener {
    fn handle_event(&mut self, event: UserInputEvent) -> HandleEventStatus {
        // the status is Input's: NotConsumed on a control inside a row, so that harry hands the
        // touch on to the row (everyware_ui::Input)
        let (gestures, status) = self.input.feed(&event);
        for (node, gesture) in gestures {
            let _ = self.actions.send(Actions::CustomAction(TvAction::Gesture(node, gesture)));
        }
        status
    }

    /// The looper drew a frame: the reducer paces its frames by this (harry FramePacing).
    fn frame_presented(&mut self, _display_id: DisplayId) {
        let _ = self.actions.send(Actions::FramePresented);
    }
}
