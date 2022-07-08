mod bevy_rust_arcade;
mod fake_arcade;

use bevy::prelude::*;
use bevy_rust_arcade::{ArcadeInputEvent, RustArcadePlugin};
use fake_arcade::KeyToArcade;

struct UserInput {}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RustArcadePlugin)
        .insert_resource(KeyToArcade::default())
        .add_system(arcade_event_system)
        .add_system(fake_arcade::input_system)
        .run();
}

// Read arcade input events
fn arcade_event_system(mut arcade_input_events: EventReader<ArcadeInputEvent>) {
    for event in arcade_input_events.iter() {
        info!(
            "{:?} of {:?} is changed to {}",
            event.arcade_input, event.gamepad, event.value
        );
    }
}
