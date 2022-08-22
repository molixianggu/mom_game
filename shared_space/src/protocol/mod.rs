mod auth;
mod color;
mod entity_assignment;
mod key_command;
mod position;
mod user;

use naia_shared::Protocolize;

pub use auth::Auth;
pub use color::{Color, ColorValue};
pub use entity_assignment::EntityAssignment;
pub use key_command::KeyCommand;
pub use position::Position;
pub use user::UserInfo;

#[derive(Protocolize)]
pub enum Protocol {
    Auth(Auth),
    KeyCommand(KeyCommand),
    EntityAssignment(EntityAssignment),
    Position(Position),
    Color(Color),
    UserInfo(UserInfo),
}
