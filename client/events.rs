use bevy::{
    ecs::{event::EventReader, system::Commands},
    prelude::*,
    render::color::Color as BevyColor,
    sprite::{Sprite, SpriteBundle},
};

use naia_bevy_client::{
    events::{
        InsertComponentEvent, MessageEvent, SpawnEntityEvent,
        UpdateComponentEvent,
    },
    shared::{sequence_greater_than, Tick},
    Client, CommandsExt,
};

use shared_space::{
    behavior,
    protocol::{Color, ColorValue, Position, Protocol, ProtocolKind, UserInfo},
    Channels,
};

use crate::resources::{Global, OwnedEntity};

pub fn connect_event(client: Client<Protocol, Channels>) {
    info!("Client connected to: {}", client.server_address());
}

pub fn disconnect_event(client: Client<Protocol, Channels>) {
    info!("Client disconnected from: {}", client.server_address());
}

pub fn spawn_entity_event(mut event_reader: EventReader<SpawnEntityEvent>) {
    for event in event_reader.iter() {
        match event {
            SpawnEntityEvent(_entity) => {
                info!("spawned entity");
            }
        }
    }
}

pub fn insert_component_event(
    mut event_reader: EventReader<InsertComponentEvent<ProtocolKind>>,
    mut local: Commands,
    color_query: Query<&Color>,
    global: Res<Global>,
) {
    for event in event_reader.iter() {
        if let InsertComponentEvent(entity, ProtocolKind::Color) = event {
            if let Ok(color) = color_query.get(*entity) {
                info!("add color to entity");
                info!("owned_entity: {:?}", global.owned_entity);

                // if let Some(owned) = &global.owned_entity {
                //     if owned.confirmed == *entity {
                //         local.entity(*entity).insert_bundle(TransformBundle {
                //             local: Transform::from_xyz(0.0, 0.0, 0.0),
                //             ..default()
                //         });
                //         continue;
                //     }
                // }

                let color = {
                    match *color.value {
                        ColorValue::Red => BevyColor::RED,
                        ColorValue::Blue => BevyColor::BLUE,
                        ColorValue::Yellow => BevyColor::YELLOW,
                    }
                };

                local.entity(*entity).insert_bundle(SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(100.0, 100.0)),
                        color,
                        ..default()
                    },
                    transform: Transform::from_xyz(0.0, 0.0, 0.0),
                    ..default()
                });
            }
        }
    }
}

pub fn update_component_event(
    mut event_reader: EventReader<UpdateComponentEvent<ProtocolKind>>,
    mut global: ResMut<Global>,
    mut position_query: Query<&mut Position>,
) {
    if let Some(owned_entity) = &global.owned_entity {
        let mut latest_tick: Option<Tick> = None;
        let server_entity = owned_entity.confirmed;
        let client_entity = owned_entity.predicted;

        for event in event_reader.iter() {
            let UpdateComponentEvent(server_tick, updated_entity, _) = event;
            // If entity is owned
            if *updated_entity == server_entity {
                if let Some(last_tick) = &mut latest_tick {
                    if sequence_greater_than(*server_tick, *last_tick) {
                        *last_tick = *server_tick;
                    }
                } else {
                    latest_tick = Some(*server_tick);
                }
            }
        }

        

        if let Some(server_tick) = latest_tick {
            if let Ok([server_position, mut client_position]) =
                position_query.get_many_mut([server_entity, client_entity])
            {
                let replay_commands =
                    global.command_history.replays(&server_tick);

                // set to authoritative state
                client_position.x.mirror(&server_position.x);
                client_position.y.mirror(&server_position.y);

                info!("重放指令: {}/", replay_commands.len());

                // Replay all stored commands
                for (_command_tick, command) in replay_commands {
                    behavior::input_command::process_command(
                        &command,
                        &mut client_position,
                    );
                }
            }
        }
    }
}

pub fn receive_message_event(
    mut event_reader: EventReader<MessageEvent<Protocol, Channels>>,
    mut local: Commands,
    mut global: ResMut<Global>,
    mut client: Client<Protocol, Channels>,
) {
    for event in event_reader.iter() {
        if let MessageEvent(
            Channels::EntityAssignment,
            Protocol::EntityAssignment(message),
        ) = event
        {
            let assign = *message.assign;

            let entity = message.entity.get(&client).unwrap();
            if assign {
                let prediction_entity =
                    CommandsExt::<Protocol>::duplicate_entity(
                        &mut local, entity,
                    )
                    .insert_bundle(SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(100.0, 100.0)),
                            color: BevyColor::WHITE,
                            ..Default::default()
                        },
                        transform: Transform::from_xyz(0.0, 0.0, 0.0),
                        ..Default::default()
                    })
                    .id();

                // local.entity(entity).remove::<Sprite>();

                info!("gave ownership of entity");

                global.owned_entity =
                    Some(OwnedEntity::new(entity, prediction_entity));

                let mut info = UserInfo::new();
                info.entity.set(&client, &entity);
                client.send_message(Channels::UserSyncSignal, &info);
            } else {
                let mut disowned: bool = false;
                if let Some(owned_entity) = &global.owned_entity {
                    if owned_entity.confirmed == entity {
                        local.entity(owned_entity.predicted).despawn();
                        disowned = true;
                    }
                }
                if disowned {
                    info!("removed ownership of entity");
                    global.owned_entity = None;
                }
            }
        }
    }
}
