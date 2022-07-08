use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, RegisterInspectable, WorldInspectorPlugin};

use crate::bevy_rust_arcade::ArcadeInput;

#[derive(Default)]
pub struct ArcadeDisplayPlugin;

impl Plugin for ArcadeDisplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_plugin(WorldInspectorPlugin::new())
            .register_inspectable::<Reactable>();
    }
}

pub enum FeedbackType {
    Good,
    Bad,
}

pub struct InputReaction {
    key: ArcadeInput,
    feedback: FeedbackType,
}

#[derive(Component, Inspectable)]
pub struct Reactable {
    pub key: ArcadeInput,
}

impl Default for Reactable {
    fn default() -> Self {
        Self {
            key: ArcadeInput::ButtonFront1,
        }
    }
}

impl Reactable {
    pub fn new(key: ArcadeInput) -> Self {
        Self { key }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let scale = 1.5f32;
    let mut cam_bundle = OrthographicCameraBundle::new_2d();
    cam_bundle.orthographic_projection.scale = scale;
    commands.spawn_bundle(cam_bundle);
    let width = 1280f32 * scale;
    let height = 1024f32 * scale;

    let button_size = 256f32;
    let mut positions: Vec<(Vec2, Reactable)> = vec![
        (
            (-width / 2f32, 0f32).into(),
            Reactable::new(ArcadeInput::ButtonLeftSide),
        ),
        (
            (-width / 5f32, -height / 2f32).into(),
            Reactable::new(ArcadeInput::ButtonFront1),
        ),
        (
            (width / 5f32, -height / 2f32).into(),
            Reactable::new(ArcadeInput::ButtonFront2),
        ),
        (
            (width / 2f32, 0f32).into(),
            Reactable::new(ArcadeInput::ButtonRightSide),
        ),
        (
            (-button_size * 1.5f32, 0f32).into(),
            Reactable::new(ArcadeInput::JoyButton),
        ),
    ];
    let joyButtons = vec![
        ArcadeInput::ButtonTop1,
        ArcadeInput::ButtonTop2,
        ArcadeInput::ButtonTop3,
        ArcadeInput::ButtonTop4,
        ArcadeInput::ButtonTop5,
        ArcadeInput::ButtonTop6,
    ];
    for x in 0..3 {
        for y in 0..2 {
            positions.push((
                (
                    button_size / 2f32 + (x as f32) * button_size + (y as f32) * button_size / 2f32,
                    -(y as f32) * button_size,
                )
                    .into(),
                Reactable::new(joyButtons[x + y * 3].clone()),
            ));
        }
    }

    for p in positions {
        commands
            .spawn_bundle(SpriteBundle {
                texture: asset_server.load("round_button.png"),
                transform: Transform::from_translation(p.0.extend(0f32)),
                ..default()
            })
            .insert(p.1);
    }

    use std::f32::consts::TAU;
    let mut positions: Vec<(Vec2, f32, Reactable)> = vec![
        (
            (-button_size * 0.5f32, 0f32).into(),
            TAU * -0.25f32,
            Reactable::new(ArcadeInput::JoyRight),
        ),
        (
            (-button_size * 2.5f32, 0f32).into(),
            TAU * 0.25f32,
            Reactable::new(ArcadeInput::JoyLeft),
        ),
        (
            (-button_size * 1.5f32, -button_size).into(),
            TAU * -0.5f32,
            Reactable::new(ArcadeInput::JoyDown),
        ),
        (
            (-button_size * 1.5f32, button_size).into(),
            0f32,
            Reactable::new(ArcadeInput::JoyUp),
        ),
    ];
    for p in positions {
        commands.spawn_bundle(SpriteBundle {
            texture: asset_server.load("arrow.png"),
            transform: Transform::from_translation(p.0.extend(0f32))
                .with_rotation(Quat::from_axis_angle((0f32, 0f32, 1f32).into(), p.1)),
            ..default()
        });
    }
}
