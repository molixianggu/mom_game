mod events;
mod resources;
mod tick;

use bevy::{
    app::{App, ScheduleRunnerPlugin},
    core::CorePlugin,
    ecs::system::Commands,
    log::{info, LogPlugin},
};

use naia_bevy_server::{
    Plugin as ServerPlugin, Server, ServerAddrs, ServerConfig, Stage,
};
use naia_shared::{SharedConfig, SocketConfig};

use crate::resources::{Global, UserDB};
use crate::tick::tick;
use shared_space::{protocol::Protocol, shared_config, Channels, CHANNEL_CONFIG};

fn main() {
    info!("Naia Bevy Server Demo starting up");

    App::default()
        // Plugins
        .add_plugin(CorePlugin)
        .add_plugin(ScheduleRunnerPlugin::default())
        .add_plugin(LogPlugin::default())
        .add_plugin(ServerPlugin::<Protocol, Channels>::new(
            ServerConfig::default(),
            shared_config(),
        ))
        // Startup System
        .add_startup_system(init)
        // Receive Server Events
        .add_system_to_stage(Stage::ReceiveEvents, events::authorization_event)
        .add_system_to_stage(Stage::ReceiveEvents, events::connection_event)
        .add_system_to_stage(Stage::ReceiveEvents, events::disconnection_event)
        .add_system_to_stage(
            Stage::ReceiveEvents,
            events::receive_message_event,
        )
        // Gameplay Loop on Tick
        .add_system_to_stage(Stage::Tick, tick)
        // Run App
        .run();
}

fn init(mut commands: Commands, mut server: Server<Protocol, Channels>) {
    info!("服务器程序启动");
    // Naia Server initialization
    let server_addresses = ServerAddrs::new(
        "0.0.0.0:14191"
            .parse()
            .expect("could not parse Signaling address/port"),
        // IP Address to listen on for UDP WebRTC data channels
        "0.0.0.0:14192"
            .parse()
            .expect("could not parse WebRTC data address/port"),
        // The public WebRTC IP address to advertise
        "http://0.0.0.0:14192",
    );

    server.listen(&server_addresses);

    // Create a new, singular room, which will contain Users and Entities that
    // they can receive updates from
    let main_room_key = server.make_room().key();

    let mut db = std::collections::HashMap::new();

    db.insert("a".to_string(), UserDB { password: "123".to_string() });
    db.insert("b".to_string(), UserDB { password: "234".to_string() });
    db.insert("c".to_string(), UserDB { password: "345".to_string() });

    // Resources
    commands.insert_resource(Global {
        main_room_key,
        user_db: db,
        user_to_prediction_map: std::collections::HashMap::new(),
        player_last_command: std::collections::HashMap::new(),
    })
}


// pub fn server_config() -> SharedConfig<Channels> {
//     // Set tick rate to ~60 FPS
//     let tick_interval = Some(std::time::Duration::from_millis(20));

//     let link_condition = None;
//     // let link_condition = Some(LinkConditionerConfig::average_condition());
//     // let link_condition = Some(LinkConditionerConfig::good_condition());
//     // let link_condition = Some(LinkConditionerConfig {
//     //     incoming_latency: 100,
//     //     incoming_jitter: 1,
//     //     incoming_loss: 0.0,
//     // });
//     SharedConfig::new(
//         SocketConfig::new(link_condition, None),
//         CHANNEL_CONFIG,
//         tick_interval,
//         None,
//     )
// }
