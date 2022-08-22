use std::collections::HashMap;

use bevy::prelude::Entity;

use naia_bevy_server::{RoomKey, UserKey};

use shared_space::protocol::{KeyCommand, UserInfo};


pub struct UserDB {
    pub password: String,
}

pub struct Global {
    pub main_room_key: RoomKey,
    pub user_db: HashMap<String, UserDB>,
    pub user_to_prediction_map: HashMap<UserKey, Entity>,
    pub player_last_command: HashMap<Entity, KeyCommand>,
}
