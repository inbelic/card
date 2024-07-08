use bevy::{
    prelude::*,
};

mod mouse;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, mouse::MousePlugin))
        .run();
}
