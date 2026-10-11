pub mod components;
pub mod constants;
pub mod messages;
pub mod types;
pub mod utils;

use bevy::prelude::*;

use self::messages::attacked::Attacked;

pub fn register(app: &mut App) {
    app.add_message::<Attacked>();
}
