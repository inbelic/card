use bevy::{
    prelude::*,
};

mod card;
mod mouse;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, mouse::MousePlugin, card::CardPlugin))
        .run();
}
