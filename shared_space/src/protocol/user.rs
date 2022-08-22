use bevy_ecs::prelude::Component;

use naia_shared::{EntityProperty, Replicate};

#[derive(Component, Replicate)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct UserInfo {
    pub entity: EntityProperty,
}

impl UserInfo {
    pub fn new() -> Self {
        Self::new_complete()
    }
}
