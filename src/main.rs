//! tv-app: a home screen in the style of a smart TV launcher on harry: a wallpaper, a greeting,
//! the time and the weather, a ribbon of device cards that scrolls sideways (a horizontal
//! ListView of #tile) and round buttons; written against the EveryWare UI library.
//!
//! Loads the design (<data-path>/tv.dcf, made from tv-patch.js by fig-to-dcf.sh), draws
//! #tv-root on the first DRM connector, writes the time once a second and the house's weather
//! when the dashboard bundle pushes it; the controls are library components. The
//! flow is harry's:
//!
//!   ticker, house link, touch --TvAction--> reducer: action.apply(state) -> presenter(state, model)
//!     -> customizations of the UI model -> looper -> pre-renderer -> GPU -> display

use crate::actions::TvAction;
use crate::presenter::UiModelPresenter;
use crate::resource_loader::ResourceManager;
use crate::state::State;
use crate::ui_event_listener::UiListener;
use crate::ui_model::UiModel;
use har_monitoring::default_performance_monitor::DefaultPerformanceMonitor;
use har_platform_api::commlib::api::DocumentSwitch;
use har_platform_api::commlib::responder::{ChannelResponder, ResponseMonitorThread};
use har_platform_api::graphics::FontInfo;
use har_platform_api::looper::{DisplayId, Looper, LooperOptions};
use har_user_input_evdev::EvdevService;
use harry::reducer::{Actions, DefaultReducer, InputDispatcherCallback};
use harry_app_utils::document_loader::{LocalDiskFigmaLoader, ShutdownOnGenericErrorResponseHandler};
use harry_app_utils::layout_helper::HarLayoutHelperManager;
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

mod actions;
mod clock;
mod config;
mod house_link;
mod presenter;
mod resource_loader;
mod state;
mod ui_event_listener;
mod ui_model;
mod weather;

fn main() {
    har_log::init_logger("tv-app");
    log::set_max_level(log::LevelFilter::Info);

    let args = config::args();
    let displays = config::displays();
    let performance_monitor = Arc::new(DefaultPerformanceMonitor::new().expect("Error creating performance monitor."));

    // No side effects: the reducer still needs a channel for them.
    let (side_effects_tx, side_effects_rx) = channel::<()>();
    thread::spawn(move || for _ in side_effects_rx {});

    let display_root_node_map = displays.iter().map(|d| (d.id, d.root_node_id.clone())).collect();
    let (actions_tx, actions_rx) = channel();
    // Text is measured with the layout helper's own fonts, not the renderer's.
    let mut layout_helper = HarLayoutHelperManager::new().expect("Failed to create layout helper");
    for (family, weight, data) in resource_loader::FONTS {
        // SAFETY: OpenType fonts are supported.
        unsafe { layout_helper.load_font(vec![FontInfo::new(family, *weight)], data) }
            .expect("Failed to load a font into the layout helper");
    }
    let reducer: DefaultReducer<State, TvAction, UiModel, (), _, _> = DefaultReducer::new(
        State::new(),
        UiModel::new(display_root_node_map),
        side_effects_tx,
        displays.iter().map(|d| d.id).collect::<Vec<DisplayId>>(),
        actions_tx.clone(),
        actions_rx,
        None,
        Some(performance_monitor.clone()),
        Some(layout_helper),
    );
    let actions = reducer.action_tx();
    let listener_actions = reducer.action_tx();
    let (looper_tx, looper_rx) = channel();
    let velocity = everyware_ui::Velocity::default();

    log::info!("tv: the design from {}", args.data_path);
    let reducer_thread = reducer.execute::<UiModelPresenter, _>(looper_tx.clone(), LocalDiskFigmaLoader::new(args.data_path.clone()));
    let (responder, doc_load_response_rx) = ChannelResponder::create_with_receiver();
    actions
        .send(Actions::SwapDocument(DocumentSwitch { document_id: args.document_id.clone() }, responder))
        .expect("Cannot send document load action");
    // Exit if the document cannot be loaded.
    let _document_load_monitor =
        ResponseMonitorThread::new(doc_load_response_rx, ShutdownOnGenericErrorResponseHandler::new(actions.clone(), looper_tx))
            .spawn();

    // The clock: one action a second.
    {
        let actions = actions.clone();
        thread::spawn(move || loop {
            if actions.send(Actions::CustomAction(TvAction::Tick(clock::now()))).is_err() {
                break;
            }
            thread::sleep(Duration::from_secs(1));
        });
    }
    // The house's weather from the dashboard bundle.
    house_link::spawn(actions.clone());

    // Touch from evdev; --touch-size maps panel units to pixels (RB3).
    let touch_size = args.touch_size.as_deref().map(|s| config::parse_size(s).unwrap_or_else(|| panic!("--touch-size {s}: expected WxH")));
    let callback = InputDispatcherCallback::new(actions_tx);
    let inputs: Vec<_> = displays
        .iter()
        .map(|display| {
            let callback = callback.clone();
            let velocity = velocity.clone();
            // the finger's speed, measured as the kernel hands the events over: a list flings with it
            let on_input = move |input| {
                everyware_ui::stamp(&velocity, &input);
                callback.process_input(input)
            };
            let service = match touch_size {
                Some(size) => EvdevService::new_scaled(display.id, display.input_devices.clone(), size, on_input),
                None => EvdevService::new(display.id, display.input_devices.clone(), on_input),
            };
            if let Err(e) = &service {
                log::warn!("tv: no input for display {}: {e:?}", display.id);
            }
            service
        })
        .collect();

    let mut looper = create_looper(&displays);
    looper.set_ui_event_listener(UiListener::new(listener_actions, velocity)).expect("Cannot set the UI event listener");
    log::info!("tv: started, loading document '{}'", args.document_id);
    looper.start(looper_rx).expect("Cannot start UI thread");

    for service in inputs.into_iter().flatten() {
        service.stop();
    }
    reducer_thread.join().expect("Error reported on the reducer thread.");
}

fn create_looper(displays: &[config::Display]) -> impl Looper<ResourceManager> {
    let mut options = displays.iter().map(|d| LooperOptions {
        display_id: Some(d.id),
        display_params: d.display_params.clone(),
        width: d.width,
        height: d.height,
        dpi: d.dpi,
        root_node_id: d.root_node_id.clone(),
        take_screenshots: false,
        driverui_health_node_id: String::new(),
    });
    let first = options.next().expect("At least one display needs to be configured.");
    let mut looper = harry_app_platform_specific::create_looper(first, ResourceManager).expect("Cannot initialize renderer");
    options.for_each(|o| {
        let id = o.display_id;
        looper.add_display(o).unwrap_or_else(|_| panic!("Cannot add display: {id:?}."));
    });
    looper
}
