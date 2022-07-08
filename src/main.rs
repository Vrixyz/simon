mod arcade_display;
mod bevy_rust_arcade;
mod fake_arcade;

use arcade_display::ArcadeDisplayPlugin;
use bevy::{
    input::keyboard::KeyboardInput,
    prelude::*,
    window::{PresentMode, WindowMode},
};
use bevy_rust_arcade::{ArcadeInput, ArcadeInputEvent, RustArcadePlugin};
use fake_arcade::KeyToArcade;

#[derive(Default)]
struct UserSequence {
    sequence: Vec<ArcadeInput>,
}

#[derive(Default)]
struct UserProgress {
    index: usize,
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Simon".to_string(),
            width: 1280f32,
            height: 1024f32,
            present_mode: PresentMode::Fifo,
            mode: WindowMode::Windowed,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(ArcadeDisplayPlugin)
        .add_plugin(RustArcadePlugin)
        .insert_resource(KeyToArcade::default())
        .insert_resource(UserSequence::default())
        .insert_resource(UserProgress::default())
        .add_system(arcade_event_system)
        .add_system(fake_arcade::input_system)
        .run();
}

// Read arcade input events
fn arcade_event_system(
    mut arcade_input_events: EventReader<ArcadeInputEvent>,
    mut sequence: ResMut<UserSequence>,
    mut progress: ResMut<UserProgress>,
) {
    for event in arcade_input_events.iter() {
        if event.value == 1f32 {
            if sequence.sequence.len() <= progress.index {
                // Add to the list
                sequence.sequence.push(event.arcade_input.clone());
                progress.index = 0;
                info!("New key: {:?}", event.arcade_input);
                return;
            }
            if event.arcade_input == sequence.sequence[progress.index] {
                // Win!
                progress.index += 1;
                info!(
                    "Correct! progress: {}/{} ({:?})",
                    progress.index,
                    sequence.sequence.len(),
                    event.arcade_input
                );
            } else {
                progress.index = 0;
                info!(
                    "Incorrect! progress: {}/{} ({:?})",
                    progress.index,
                    sequence.sequence.len(),
                    event.arcade_input
                );
            }
        }
    }
}

pub fn input_system(
    mappings: Res<KeyToArcade>,
    mut key_evr: EventReader<KeyboardInput>,
    mut arcade_gamepad_event: EventWriter<ArcadeInputEvent>,
) {
    use bevy::input::ElementState;
    for ev in key_evr.iter() {
        if ev.key_code.is_none() {
            continue;
        }
        let key_code = ev.key_code.unwrap();
        match mappings.mappings.get(&key_code) {
            Some(arcade_input) => {
                arcade_gamepad_event.send(ArcadeInputEvent {
                    gamepad: Gamepad(0),
                    arcade_input: (*arcade_input).clone(),
                    value: match ev.state {
                        ElementState::Pressed => 1f32,
                        ElementState::Released => 0f32,
                    },
                });
            }
            None => {
                info!("Key {:?} without mappings", key_code);
            }
        }
    }
}
