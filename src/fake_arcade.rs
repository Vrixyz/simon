use crate::bevy_rust_arcade::{ArcadeInput, ArcadeInputEvent};
use bevy::{input::keyboard::KeyboardInput, prelude::*};
use std::collections::HashMap;

pub struct KeyToArcade {
    pub mappings: HashMap<KeyCode, ArcadeInput>,
}

impl Default for KeyToArcade {
    fn default() -> Self {
        Self {
            mappings: HashMap::from([
                (KeyCode::Up, ArcadeInput::JoyUp),
                (KeyCode::Down, ArcadeInput::JoyDown),
                (KeyCode::Left, ArcadeInput::JoyLeft),
                (KeyCode::Right, ArcadeInput::JoyRight),
                (KeyCode::RShift, ArcadeInput::JoyButton),
                (KeyCode::Key1, ArcadeInput::ButtonTop1),
                (KeyCode::Key2, ArcadeInput::ButtonTop2),
                (KeyCode::Key3, ArcadeInput::ButtonTop3),
                (KeyCode::Key4, ArcadeInput::ButtonTop4),
                (KeyCode::Key5, ArcadeInput::ButtonTop5),
                (KeyCode::Key6, ArcadeInput::ButtonTop6),
                (KeyCode::LControl, ArcadeInput::ButtonLeftSide),
                (KeyCode::RControl, ArcadeInput::ButtonRightSide),
                (KeyCode::LAlt, ArcadeInput::ButtonFront1),
                (KeyCode::Space, ArcadeInput::ButtonFront2),
            ]),
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
