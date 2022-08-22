use bevy::prelude::*;

use naia_bevy_server::Server;

use shared_space::{
    behavior,
    // behavior as shared_behavior,
    protocol::{Position, Protocol},
    Channels,
};

use crate::resources::Global;

pub fn tick(
    mut global: ResMut<Global>,
    mut server: Server<Protocol, Channels>,
    mut position_query: Query<&mut Position>,
) {
    // / All game logic should happen here, on a tick event
    // info!("tick: {}", global.player_last_command.len());

    // Update scopes of entities
    for (_, user_key, entity) in server.scope_checks() {
        // You'd normally do whatever checks you need to in here..
        // to determine whether each Entity should be in scope or not.

        // This indicates the Entity should be in this scope.
        server.user_scope(&user_key).include(&entity);

        // And call this if Entity should NOT be in this scope.
        // server.user_scope(..).exclude(..);
    }

    // Process all received commands
    for (entity, last_command) in global.player_last_command.drain() {
        if let Ok(mut position) = position_query.get_mut(entity) {
            behavior::input_command::process_command(
                &last_command,
                &mut position,
            );
            // info!("position: ({:?}) => ({}, {})", last_command, *position.x, *position.y);
        }
    }

    // This is very important! Need to call this to actually send all update
    // packets to all connected Clients!
    server.send_all_updates();
}
