mod timing_button;

use bevy::{
    app::AppExit,
    prelude::*,
    window::{PresentMode, WindowMode},
};
use progress::ProgressPlugin;
use rust_arcade::fake_arcade::KeyToArcade;
use rust_arcade::{
    bevy_rust_arcade::{ArcadeInput, ArcadeInputEvent, RustArcadePlugin},
    fake_arcade,
};
use rust_arcade_display::{ArcadeDisplayPlugin, InputReaction};

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
        .add_plugin(ProgressPlugin)
        .insert_resource(KeyToArcade::default())
        .add_system(arcade_event_system)
        .add_system(fake_arcade::input_system)
        .run();
}

// Read arcade input events
fn arcade_event_system(
    mut exit: EventWriter<AppExit>,
    mut arcade_input_events: EventReader<ArcadeInputEvent>,
    mut feedback_events: EventWriter<InputReaction>,
) {
    for event in arcade_input_events.iter() {
        if event.value == 1f32 {
            match &event.arcade_input {
                ArcadeInput::ButtonFront1 => {
                    feedback_events.send(InputReaction {
                        key: event.arcade_input.clone(),
                        feedback: rust_arcade_display::FeedbackType::Fun,
                    });
                    return;
                }
                ArcadeInput::ButtonLeftSide => {
                    feedback_events.send(InputReaction {
                        key: event.arcade_input.clone(),
                        feedback: rust_arcade_display::FeedbackType::Menu,
                    });
                    exit.send(AppExit);
                    return;
                }
                ArcadeInput::ButtonRightSide => {
                    feedback_events.send(InputReaction {
                        key: event.arcade_input.clone(),
                        feedback: rust_arcade_display::FeedbackType::Menu,
                    });
                    return;
                }
                _ => {
                    feedback_events.send(InputReaction {
                        key: event.arcade_input.clone(),
                        feedback: rust_arcade_display::FeedbackType::Good,
                    });
                }
            }
        }
    }
}
