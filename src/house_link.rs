//! tv-app: the house's weather (the news are not on this screen). The dashboard bundle (vendor/sdv-everyware/everyware/bundles/
//! dashboard) pushes the weather and the news over gRPC to 127.0.0.1:7100, where the panel app
//! listens; the TV screen takes the same port (one of the two runs at a time) and the same service.
//! Wi-Fi and Matter are not on this screen: their updates are acknowledged and dropped, their
//! command streams stay open and silent so the bundle does not keep reconnecting.

use crate::actions::TvAction;
use futures::channel::mpsc::unbounded;
use futures::{SinkExt, StreamExt};
use grpcio::{Environment, RpcContext, ServerBuilder, ServerCredentials, ServerStreamingSink, UnarySink, WriteFlags};
use harry::reducer::Actions;
use everyware_dashboard_grpc::dashboard::{Ack, HeartbeatRequest, MatterCommandResult, Subscribe, WifiCommandResult};
use everyware_dashboard_grpc::dashboard_grpc::{create_dashboard_service, DashboardService};
use everyware_dashboard_grpc::matter::MatterDevices;
use everyware_dashboard_grpc::matter_service::MatterCommand;
use everyware_dashboard_grpc::news::{NewsFeed, NewsThumbnail};
use everyware_dashboard_grpc::weather::Weather;
use everyware_dashboard_grpc::wifi::{WifiCommand, WifiNetworks, WifiState};
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use std::thread;

pub const ADDRESS: &str = "127.0.0.1:7100";

#[derive(Clone)]
struct Service {
    actions: Sender<Actions<TvAction>>,
    // the senders of the silent command streams, kept so the streams stay open
    streams: Arc<Mutex<Vec<Box<dyn std::any::Any + Send>>>>,
}

fn ack(ctx: &RpcContext<'_>, sink: UnarySink<Ack>) {
    ctx.spawn(async move {
        let _ = sink.success(Ack::new()).await;
    });
}

impl Service {
    fn act(&self, action: TvAction) {
        let _ = self.actions.send(Actions::CustomAction(action));
    }

    fn silent<T: Send + 'static>(&mut self, ctx: RpcContext<'_>, sink: ServerStreamingSink<T>) {
        let (tx, rx) = unbounded::<T>();
        self.streams.lock().expect("streams").push(Box::new(tx) as Box<dyn std::any::Any + Send>);
        let mut sink = sink;
        ctx.spawn(async move {
            let mut rx = rx.map(|item| Ok((item, WriteFlags::default())));
            let _ = sink.send_all(&mut rx).await;
            let _ = sink.close().await;
        });
    }
}

impl DashboardService for Service {
    fn publish_weather(&mut self, ctx: RpcContext<'_>, weather: Weather, sink: UnarySink<Ack>) {
        self.act(TvAction::Weather(weather));
        ack(&ctx, sink);
    }

    fn publish_news(&mut self, ctx: RpcContext<'_>, news: NewsFeed, sink: UnarySink<Ack>) {
        let _ = news;
        ack(&ctx, sink);
    }

    fn publish_news_thumbnail(&mut self, ctx: RpcContext<'_>, thumbnail: NewsThumbnail, sink: UnarySink<Ack>) {
        let _ = thumbnail;
        ack(&ctx, sink);
    }

    fn publish_wifi_networks(&mut self, ctx: RpcContext<'_>, _networks: WifiNetworks, sink: UnarySink<Ack>) {
        ack(&ctx, sink);
    }

    fn publish_wifi_state(&mut self, ctx: RpcContext<'_>, _state: WifiState, sink: UnarySink<Ack>) {
        ack(&ctx, sink);
    }

    fn publish_wifi_result(&mut self, ctx: RpcContext<'_>, _result: WifiCommandResult, sink: UnarySink<Ack>) {
        ack(&ctx, sink);
    }

    fn wifi_commands(&mut self, ctx: RpcContext<'_>, _request: Subscribe, sink: ServerStreamingSink<WifiCommand>) {
        self.silent(ctx, sink);
    }

    fn publish_matter_devices(&mut self, ctx: RpcContext<'_>, _devices: MatterDevices, sink: UnarySink<Ack>) {
        ack(&ctx, sink);
    }

    fn publish_matter_result(&mut self, ctx: RpcContext<'_>, _result: MatterCommandResult, sink: UnarySink<Ack>) {
        ack(&ctx, sink);
    }

    fn matter_commands(&mut self, ctx: RpcContext<'_>, _request: Subscribe, sink: ServerStreamingSink<MatterCommand>) {
        self.silent(ctx, sink);
    }

    fn heartbeat(&mut self, ctx: RpcContext<'_>, _request: HeartbeatRequest, sink: UnarySink<Ack>) {
        ack(&ctx, sink);
    }
}

/// The server thread: listens until the process ends.
pub fn spawn(actions: Sender<Actions<TvAction>>) {
    thread::spawn(move || {
        let service = Service { actions, streams: Arc::new(Mutex::new(Vec::new())) };
        let env = Arc::new(Environment::new(1));
        let server = ServerBuilder::new(env)
            .register_service(create_dashboard_service(service))
            .build()
            .and_then(|mut server| server.add_listening_port(ADDRESS, ServerCredentials::insecure()).map(|_| server));
        match server {
            Ok(mut server) => {
                server.start();
                log::info!("tv: listening on {ADDRESS} for the house's data");
                loop {
                    thread::park();
                }
            }
            Err(err) => log::error!("tv: cannot listen on {ADDRESS}: {err}"),
        }
    });
}

