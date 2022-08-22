use std::fmt::{write, Display, Debug};

use bevy_ecs::prelude::Component;

use naia_shared::{EntityProperty, Property, Replicate};

#[derive(Component, Replicate)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct KeyCommand {
    pub entity: EntityProperty,
    pub w: Property<bool>,
    pub s: Property<bool>,
    pub a: Property<bool>,
    pub d: Property<bool>,
}

impl Display for KeyCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write(f, format_args!("<"))?;

        if *self.w {
            write(f, format_args!("w"))?;
        }
        if *self.a {
            write(f, format_args!("w"))?;
        }
        if *self.s {
            write(f, format_args!("s"))?;
        }
        if *self.d {
            write(f, format_args!("d"))?;
        }
        write(f, format_args!(">"))
    }
}

impl Debug for KeyCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut d = f.debug_struct("KeyCommand");
        if *self.w {
            d.field("w", &"");
        }
        if *self.a {
            d.field("a", &"");
        }
        if *self.s {
            d.field("s", &"");
        }
        if *self.d {
            d.field("d", &"");
        }

        d.finish()
    }
}

impl KeyCommand {
    pub fn new(w: bool, s: bool, a: bool, d: bool) -> Self {
        KeyCommand::new_complete(w, s, a, d)
    }
}
