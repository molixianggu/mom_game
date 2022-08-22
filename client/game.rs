use std::time::Duration;

use bevy::prelude::*;

use naia_bevy_client::{Client, ClientConfig, Plugin as ClientPlugin, Stage};

use naia_shared::{ConnectionConfig, PingConfig};
use shared_space::{
    protocol::{Auth, Protocol},
    shared_config, Channels,
};

use crate::events;
use crate::input::input;
use crate::sync::sync;
use crate::loading::LoadingPlugin;
use crate::resources::Global;
use crate::tick::tick;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    Loading,
    Start,
    // Playing,
    // Menu,
}

#[derive(Default)]
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ClientPlugin::<Protocol, Channels>::new(
            ClientConfig{
                connection: ConnectionConfig{
                    disconnection_timeout_duration: Duration::from_secs(5),
                    ping: PingConfig{
                        rtt_initial_estimate: Duration::from_millis(60),
                        jitter_initial_estimate: Duration::from_millis(10),
                        rtt_smoothing_factor: 0.1,
                        ..default()
                    },
                    ..default()
                },
                send_handshake_interval: Duration::from_millis(250),
                minimum_latency: Some(Duration::from_millis(20)),
                ..default()
            },
            shared_config(),
        ))
        .add_startup_system(init)
        .add_system_to_stage(Stage::Connection, events::connect_event)
        .add_system_to_stage(Stage::Disconnection, events::disconnect_event)
        .add_system_to_stage(Stage::ReceiveEvents, events::spawn_entity_event)
        .add_system_to_stage(
            Stage::ReceiveEvents,
            events::insert_component_event,
        )
        .add_system_to_stage(
            Stage::ReceiveEvents,
            events::update_component_event,
        )
        .add_system_to_stage(
            Stage::ReceiveEvents,
            events::receive_message_event,
        )
        .add_system_to_stage(Stage::Frame, input)
        .add_system_to_stage(Stage::PostFrame, sync)
        // .add_state(GameState::Loading)
        // .add_plugin(LoadingPlugin)
        .add_system_to_stage(Stage::Tick, tick);
    }
}

fn init(mut commands: Commands, mut client: Client<Protocol, Channels>) {
    info!("程序初始化...");

    client.auth(Auth::new("a", "123"));
    client.connect("http://172.26.228.211:14191");

    // Setup Camera
    commands.spawn_bundle(Camera2dBundle::default());

    // Setup Colors
    commands.init_resource::<Global>();
}
