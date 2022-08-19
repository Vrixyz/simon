mod react_timing_buttons;
mod spawn_timings;
mod timing_button;

use bevy::{
    app::AppExit,
    prelude::*,
    window::{PresentMode, WindowMode},
};
use progress::ProgressPlugin;
use react_timing_buttons::{ReactTimingButtons, ValidateButtonEvent};
use rust_arcade::fake_arcade::KeyToArcade;
use rust_arcade::{
    bevy_rust_arcade::{ArcadeInput, ArcadeInputEvent, RustArcadePlugin},
    fake_arcade,
};
use rust_arcade_display::{ArcadeDisplayPlugin, InputReaction};
use spawn_timings::SpawnTiming;
use timing_button::TimingButtonsPlugin;

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
        .add_plugin(ReactTimingButtons)
        .add_plugin(ProgressPlugin)
        .add_plugin(TimingButtonsPlugin)
        .add_plugin(SpawnTiming)
        .insert_resource(KeyToArcade::default())
        .add_system(arcade_event_system)
        .add_system(fake_arcade::input_system)
        .run();
}

// Read arcade input events
fn arcade_event_system(
    mut exit: EventWriter<AppExit>,
    mut arcade_input_events: EventReader<ArcadeInputEvent>,
    mut ev_validate: EventWriter<ValidateButtonEvent>,
    mut feedback_events: EventWriter<InputReaction>,
) {
    for event in arcade_input_events.iter() {
        if event.value == 1f32 {
            match &event.arcade_input {
                ArcadeInput::ButtonFront1 => {
                    feedback_events.send(InputReaction {
                        key: event.arcade_input.clone(),
                        feedback: rust_arcade_display::FeedbackType::Cheat,
                    });
                    return;
                }
                ArcadeInput::ButtonLeftSide => {
                    feedback_events.send(InputReaction {
                        key: event.arcade_input.clone(),
                        feedback: rust_arcade_display::FeedbackType::Cheat,
                    });
                    exit.send(AppExit);
                    return;
                }
                ArcadeInput::ButtonRightSide => {
                    feedback_events.send(InputReaction {
                        key: event.arcade_input.clone(),
                        feedback: rust_arcade_display::FeedbackType::Cheat,
                    });
                    return;
                }
                _ => {
                    ev_validate.send(ValidateButtonEvent(event.arcade_input.clone()));
                }
            }
        }
    }
}
