mod arcade_display;
mod bevy_rust_arcade;
mod fake_arcade;
pub mod particles;
pub mod progress;
pub mod simon_progress;

use std::{
    fs::{self, OpenOptions},
    io::{BufReader, Write},
    path::Path,
};

use arcade_display::{ArcadeDisplayPlugin, InputReaction};
use bevy::{
    app::AppExit,
    input::keyboard::KeyboardInput,
    prelude::*,
    reflect::erased_serde::private::serde::Deserializer,
    window::{PresentMode, WindowMode},
};
use bevy_rust_arcade::{ArcadeInput, ArcadeInputEvent, RustArcadePlugin};
use fake_arcade::KeyToArcade;
use progress::ProgressPlugin;
use simon_progress::SimonProgressPlugin;

#[derive(Default)]
struct UserSequence {
    sequence: Vec<ArcadeInput>,
}

#[derive(Default)]
struct UserProgress {
    index: usize,
}

struct ShowNextPlay {
    pub next_play: f32,
    pub delta_between_displays: f32,
}

enum CheatState {
    Disabled,
    ShowNextPlay(ShowNextPlay),
}

struct ResetState {
    pub start_time: f32,
    pub seconds_before_reset: f32,
}

struct SequenceFileToLoad(pub Option<String>);

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
        .insert_resource(CheatState::Disabled)
        .add_plugins(DefaultPlugins)
        .add_plugin(ArcadeDisplayPlugin)
        .add_plugin(RustArcadePlugin)
        .add_plugin(ProgressPlugin)
        .add_plugin(SimonProgressPlugin)
        .insert_resource(KeyToArcade::default())
        .insert_resource(UserSequence::default())
        .insert_resource(UserProgress::default())
        .insert_resource(None as Option<ResetState>)
        .insert_resource(SequenceFileToLoad(Some("./current.json".into())))
        .add_startup_system(load_sequence)
        .add_system(arcade_event_system)
        .add_system(fake_arcade::input_system)
        .add_system(update_cheat_display_next)
        .run();
}

fn read_sequence_from_file<P: AsRef<std::path::Path>>(
    path: P,
) -> Result<Vec<ArcadeInput>, Box<dyn std::error::Error>> {
    // Open the file in read-only mode with buffer.
    let file = fs::File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let u = serde_json::from_reader(reader)?;

    // Return the `User`.
    Ok(u)
}

fn load_sequence(mut fileToLoad: ResMut<SequenceFileToLoad>, mut sequence: ResMut<UserSequence>) {
    if let SequenceFileToLoad(Some(path)) = &*fileToLoad {
        if let Ok(new_sequence) = read_sequence_from_file(path) {
            sequence.sequence = new_sequence
        };
    }
    *fileToLoad = SequenceFileToLoad(None);
}

// Read arcade input events
fn arcade_event_system(
    mut exit: EventWriter<AppExit>,
    mut cheat_state: ResMut<CheatState>,
    mut reset_state: ResMut<Option<ResetState>>,
    time: Res<Time>,
    mut arcade_input_events: EventReader<ArcadeInputEvent>,
    mut feedback_events: EventWriter<InputReaction>,
    mut sequence: ResMut<UserSequence>,
    mut progress: ResMut<UserProgress>,
) {
    for event in arcade_input_events.iter() {
        const reset_button: ArcadeInput = ArcadeInput::ButtonFront2;
        if event.value == 0f32 && event.arcade_input == reset_button {
            *reset_state = None;
            return;
        }
        if event.value == 1f32 {
            match event.arcade_input {
                ArcadeInput::ButtonLeftSide => {
                    feedback_events.send(InputReaction {
                        key: event.arcade_input.clone(),
                        feedback: arcade_display::FeedbackType::Menu,
                    });
                    info!("will write");
                    if let Ok(json_content) = serde_json::to_string(&sequence.sequence) {
                        info!("continue to file write");
                        let file_path = Path::new("./current.json");
                        match if file_path.exists() {
                            OpenOptions::new().write(true).open(file_path)
                        } else {
                            fs::File::create(file_path)
                        } {
                            Ok(mut file) => {
                                info!("writing to current.json");
                                file.write_all(json_content.as_bytes()).unwrap();
                            }
                            Err(_) => todo!(),
                        }
                    }
                    // TODO: quit after delay
                    exit.send(AppExit);
                    return;
                }
                reset_button => {
                    feedback_events.send(InputReaction {
                        key: event.arcade_input.clone(),
                        feedback: arcade_display::FeedbackType::Menu,
                    });
                    info!("will write");
                    if let Ok(json_content) = serde_json::to_string(&sequence.sequence) {
                        info!("continue to file write");
                        let file_path = Path::new("./current.json");
                        match if file_path.exists() {
                            fs::File::open(file_path)
                        } else {
                            fs::File::create(file_path)
                        } {
                            Ok(mut file) => {
                                info!("writing to current.json");
                                file.write_all(json_content.as_bytes()).unwrap();
                            }
                            Err(_) => todo!(),
                        }
                    }
                    // TODO: quit after delay
                    exit.send(AppExit);
                    return;
                }
                ArcadeInput::ButtonRightSide => {
                    *cheat_state = match *cheat_state {
                        CheatState::Disabled => CheatState::ShowNextPlay(ShowNextPlay {
                            next_play: time.seconds_since_startup() as f32,
                            delta_between_displays: 1f32,
                        }),
                        CheatState::ShowNextPlay(_) => CheatState::Disabled,
                    };
                    feedback_events.send(InputReaction {
                        key: event.arcade_input.clone(),
                        feedback: arcade_display::FeedbackType::Menu,
                    });
                    return;
                }
                _ => {}
            }
            match *cheat_state {
                CheatState::Disabled => {}
                CheatState::ShowNextPlay(ref mut show_next_play) => {
                    show_next_play.next_play = time.seconds_since_startup() as f32 + 0.2f32
                }
            };
            if sequence.sequence.len() <= progress.index {
                // Add to the list
                sequence.sequence.push(event.arcade_input.clone());
                progress.index = 0;
                info!("New key: {:?}", event.arcade_input);
                feedback_events.send(InputReaction {
                    key: event.arcade_input.clone(),
                    feedback: arcade_display::FeedbackType::New,
                });
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

                feedback_events.send(InputReaction {
                    key: event.arcade_input.clone(),
                    feedback: if progress.index == sequence.sequence.len() {
                        arcade_display::FeedbackType::Last
                    } else {
                        arcade_display::FeedbackType::Good
                    },
                });
            } else {
                feedback_events.send(InputReaction {
                    key: sequence.sequence[progress.index].clone(),
                    feedback: arcade_display::FeedbackType::Cheat,
                });
                progress.index = 0;
                info!(
                    "Incorrect! progress: {}/{} ({:?})",
                    progress.index,
                    sequence.sequence.len(),
                    event.arcade_input
                );
                feedback_events.send(InputReaction {
                    key: event.arcade_input.clone(),
                    feedback: arcade_display::FeedbackType::Bad,
                });
            }
        }
    }
}

fn update_cheat_display_next(
    mut cheat_state: ResMut<CheatState>,
    time: Res<Time>,
    mut feedback_events: EventWriter<InputReaction>,
    sequence: Res<UserSequence>,
    mut progress: Res<UserProgress>,
) {
    if let CheatState::ShowNextPlay(ref mut show_next_play) = *cheat_state {
        if sequence.sequence.len() <= progress.index {
            return;
        }
        let current_time = time.seconds_since_startup() as f32;
        if show_next_play.next_play <= current_time {
            feedback_events.send(InputReaction {
                key: sequence.sequence[progress.index].clone(),
                feedback: arcade_display::FeedbackType::Cheat,
            });
            show_next_play.next_play = current_time + show_next_play.delta_between_displays;
        }
    }
}
