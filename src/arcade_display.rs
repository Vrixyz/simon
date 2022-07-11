use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_inspector_egui::{Inspectable, RegisterInspectable, WorldInspectorPlugin};
use rand::prelude::*;

use crate::{
    bevy_rust_arcade::ArcadeInput,
    particles::{DestroyAfter, ParticlesPlugin, Velocity},
};

#[derive(Default)]
pub struct ArcadeDisplayPlugin;

impl Plugin for ArcadeDisplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_plugin(ParticlesPlugin)
            .add_plugin(WorldInspectorPlugin::new())
            .register_inspectable::<Reactable>()
            .add_event::<InputReaction>()
            .add_event::<ParticleExplosion>()
            .add_system(handle_reaction_events)
            .add_system(handle_particle_events);
    }
}

#[derive(Debug, PartialEq)]
pub enum FeedbackType {
    Good,
    Bad,
    Last,
    New,
    Cheat,
    Menu,
}

#[derive(Debug)]
pub struct InputReaction {
    pub key: ArcadeInput,
    pub feedback: FeedbackType,
}

#[derive(Debug)]
pub struct ParticleExplosion {
    pub location: Vec2,
    pub color: Color,
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

    let layer_z = 10f32;
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
    let joy_buttons = vec![
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
                Reactable::new(joy_buttons[x + y * 3].clone()),
            ));
        }
    }

    for p in positions {
        commands
            .spawn_bundle(SpriteBundle {
                texture: asset_server.load("round_button.png"),
                transform: Transform::from_translation(p.0.extend(layer_z)),
                ..default()
            })
            .insert(p.1);
    }

    use std::f32::consts::TAU;
    let positions: Vec<(Vec2, f32, Reactable)> = vec![
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
        commands
            .spawn_bundle(SpriteBundle {
                texture: asset_server.load("arrow.png"),
                transform: Transform::from_translation(p.0.extend(layer_z))
                    .with_rotation(Quat::from_axis_angle((0f32, 0f32, 1f32).into(), p.1)),
                ..default()
            })
            .insert(p.2);
    }
}

fn handle_reaction_events(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut reactions: EventReader<InputReaction>,
    mut particles: EventWriter<ParticleExplosion>,
    q_reactables: Query<(&Transform, &Reactable)>,
) {
    for ev in reactions.iter() {
        let (color, layer) = match ev.feedback {
            FeedbackType::Bad => (Color::RED, 20f32),
            FeedbackType::Good => (Color::GREEN, 20f32),
            FeedbackType::Last => (Color::YELLOW, 20f32),
            FeedbackType::New => (Color::BLUE, 20f32),
            FeedbackType::Cheat => (Color::GRAY, 19f32),
            FeedbackType::Menu => (Color::FUCHSIA, 20f32),
        };
        info!("event: {:?}", ev);
        for (t, r) in q_reactables.iter() {
            if r.key != ev.key {
                continue;
            }
            let particle = ParticleExplosion {
                location: t.translation.xy(),
                color,
            };
            let sprite_to_load = match r.key {
                ArcadeInput::JoyRight
                | ArcadeInput::JoyDown
                | ArcadeInput::JoyLeft
                | ArcadeInput::JoyUp => "arrow.png",
                _ => "round_button.png",
            };
            commands
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite { color, ..default() },
                    texture: asset_server.load(sprite_to_load),
                    transform: Transform::from_translation(t.translation.xy().extend(layer))
                        .with_rotation(t.rotation),
                    ..default()
                })
                .insert(DestroyAfter::new(
                    time.seconds_since_startup() as f32 + 0.5f32,
                ));
            info!("send: {:?}", particle);
            particles.send(particle);
        }
    }
}

fn handle_particle_events(
    mut commands: Commands,
    time: Res<Time>,
    mut evt_particles: EventReader<ParticleExplosion>,
) {
    for p in evt_particles.iter() {
        info!("spawn particles {:?}", p);
        let time_to_die = time.seconds_since_startup() as f32 + 1f32;
        for i in 0..40 {
            let mut offset: Vec2 = rand::thread_rng().gen::<(f32, f32)>().into();
            offset -= Vec2::new(0.5f32, 0.5f32);
            let position = p.location + (offset * 50f32);
            commands
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: p.color,
                        custom_size: Some(Vec2::splat(50f32)),
                        ..default()
                    },
                    transform: Transform::from_translation(position.extend(1f32)),
                    ..default()
                })
                .insert(Velocity(offset * 2000f32))
                .insert(DestroyAfter::new(time_to_die));
        }
    }
}
