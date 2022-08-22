use bevy::{
    ecs::{event::EventReader, system::ResMut},
    log::info,
};

use naia_bevy_server::{
    events::{
        AuthorizationEvent, ConnectionEvent, DisconnectionEvent, MessageEvent,
    },
    shared::Random,
    Server,
};

use shared_space::{
    protocol::{
        Color, ColorValue, EntityAssignment, Position, Protocol,
    },
    Channels,
};

use crate::resources::Global;

pub fn authorization_event(
    mut event_reader: EventReader<AuthorizationEvent<Protocol>>,
    mut server: Server<Protocol, Channels>,
    global: ResMut<Global>,
) {
    for event in event_reader.iter() {
        if let AuthorizationEvent(user_key, Protocol::Auth(auth)) = event {
            info!("auth: {:?}", *auth.username);
            if let Some(user) = global.user_db.get(&*auth.username) {
                if user.password == *auth.password {
                    server.accept_connection(user_key);
                    continue;
                }
            }
            server.reject_connection(user_key);
            // if *auth.username == "charlie" && *auth.password == "12345" {
            //     // Accept incoming connection
            //     server.accept_connection(user_key);
            // } else {
            //     // Reject incoming connection
            //     server.reject_connection(user_key);
            // }
        }
    }
}

pub fn connection_event<'world, 'state>(
    mut event_reader: EventReader<ConnectionEvent>,
    mut global: ResMut<Global>,
    mut server: Server<'world, 'state, Protocol, Channels>,
) {
    for event in event_reader.iter() {
        let ConnectionEvent(user_key) = event;
        let address = server
            .user_mut(user_key)
            // Add User to the main Room
            .enter_room(&global.main_room_key)
            // Get User's address for logging
            .address();

        info!("Naia Server connected to: {}", address);

        // Create components for Entity to represent new player

        // Position component
        let position = {
            let x = 16 * ((Random::gen_range_u32(0, 40) as i16) - 20);
            let y = 16 * ((Random::gen_range_u32(0, 30) as i16) - 15);
            Position::new(x, y)
        };

        // Spawn entity
        let entity_id = server
            // Spawn new Square Entity
            .spawn()
            // Add Entity to main Room
            .enter_room(&global.main_room_key)
            // Insert Position component
            .insert(position)
            // return Entity id
            .id();

        global.user_to_prediction_map.insert(*user_key, entity_id);

        // Send an Entity Assignment message to the User that owns the Square
        let mut assignment_message = EntityAssignment::new(true);
        assignment_message.entity.set(&server, &entity_id);

        server.send_message(
            user_key,
            Channels::EntityAssignment,
            &assignment_message,
        );

        info!("连接成功");
    }
}

pub fn disconnection_event(
    mut event_reader: EventReader<DisconnectionEvent>,
    mut global: ResMut<Global>,
    mut server: Server<Protocol, Channels>,
) {
    for event in event_reader.iter() {
        let DisconnectionEvent(user_key, user) = event;
        info!("Naia Server disconnected from: {:?}", user.address);

        if let Some(entity) = global.user_to_prediction_map.remove(user_key) {
            server
                .entity_mut(&entity)
                .leave_room(&global.main_room_key)
                .despawn();
        }
    }
}

pub fn receive_message_event(
    mut event_reader: EventReader<MessageEvent<Protocol, Channels>>,
    mut global: ResMut<Global>,
    mut server: Server<Protocol, Channels>,
) {
    for event in event_reader.iter() {
        match event {
            MessageEvent(
                _user_key,
                Channels::PlayerCommand,
                Protocol::KeyCommand(key_command),
            ) => {
                if let Some(entity) = &key_command.entity.get(&server) {
                    global
                        .player_last_command
                        .insert(*entity, key_command.clone());
                }
            }
            MessageEvent(
                _user_key,
                Channels::UserSyncSignal,
                Protocol::UserInfo(info),
            ) => {
                if let Some(entity) = &info.entity.get(&server) {
                    let color = {
                        let color_value = match server.users_count() % 3 {
                            0 => ColorValue::Yellow,
                            1 => ColorValue::Red,
                            _ => ColorValue::Blue,
                        };
                        Color::new(color_value)
                    };
                    server.entity_mut(entity).insert(color);
                }
            }
            _ => {
                info!("未知消息");
            }
        }
    }
}
