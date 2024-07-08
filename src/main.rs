use bevy::{
    prelude::*,
};

mod card;
mod card_drag;
mod mouse;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((mouse::MousePlugin, card::CardPlugin, card_drag::CardDragPlugin))
        .run();
}
