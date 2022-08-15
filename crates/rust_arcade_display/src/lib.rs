use std::collections::HashMap;

use bevy::{math::Vec3Swizzles, prelude::*};

use particles::{DestroyAfter, ParticleExplosion, ParticlesPlugin, Velocity};
use rust_arcade::bevy_rust_arcade::ArcadeInput;

#[derive(Default)]
pub struct ArcadeDisplayPlugin;

impl Plugin for ArcadeDisplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_set(
            SystemSet::new()
                .with_system(load_images)
                .with_system(load_sounds),
        )
        .add_startup_system_to_stage(StartupStage::PostStartup, setup)
        .add_plugin(ParticlesPlugin)
        .add_event::<InputReaction>()
        .add_system(handle_reaction_events);
    }
}

struct ButtonImages {
    round_button: Handle<Image>,
    arrow: Handle<Image>,
}
struct ButtonSounds {
    pub sounds: HashMap<ArcadeInput, EqAudioSource>,
}
struct ButtonSoundsFr {
    pub sounds: HashMap<ArcadeInput, EqAudioSource>,
}

#[derive(Eq)]
struct EqAudioSource(pub Handle<AudioSource>);

impl PartialEq for EqAudioSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl From<Handle<AudioSource>> for EqAudioSource {
    fn from(h: Handle<AudioSource>) -> Self {
        Self(h)
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
    Fun,
}

#[derive(Debug)]
pub struct InputReaction {
    pub key: ArcadeInput,
    pub feedback: FeedbackType,
}

#[derive(Component)]
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

fn load_images(mut commands: Commands, asset_server: Res<AssetServer>) {
    let button_images = ButtonImages {
        round_button: asset_server.load("round_button.png"),
        arrow: asset_server.load("arrow.png"),
    };
    commands.insert_resource(button_images);
}

fn load_sounds(mut commands: Commands, asset_server: Res<AssetServer>) {
    let sounds_map_name = [
        (ArcadeInput::JoyUp, "up"),
        (ArcadeInput::JoyDown, "down"),
        (ArcadeInput::JoyLeft, "left"),
        (ArcadeInput::JoyRight, "right"),
        (ArcadeInput::JoyButton, "mid"),
        (ArcadeInput::ButtonTop1, "1"),
        (ArcadeInput::ButtonTop2, "2"),
        (ArcadeInput::ButtonTop3, "3"),
        (ArcadeInput::ButtonTop4, "4"),
        (ArcadeInput::ButtonTop5, "5"),
        (ArcadeInput::ButtonTop6, "6"),
        (ArcadeInput::ButtonLeftSide, "up"),
        (ArcadeInput::ButtonRightSide, "cheat"),
        (ArcadeInput::ButtonFront1, "fun"),
        (ArcadeInput::ButtonFront2, "reset"),
    ];
    let sounds_en = ButtonSounds {
        sounds: HashMap::from(sounds_map_name.clone().map(|(key, name)| {
            let path = format!("{}{}{}", "sounds/", name, ".ogg");
            (key, asset_server.load(&path).into())
        })),
    };
    let sounds_fr = ButtonSoundsFr {
        sounds: HashMap::from(sounds_map_name.map(|(key, name)| {
            let path = format!("{}{}{}", "sounds/", name, "-fr.ogg");
            (key, asset_server.load(&path).into())
        })),
    };
    commands.insert_resource(sounds_en);
    commands.insert_resource(sounds_fr);
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, images: Res<ButtonImages>) {
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

    let texts = vec![
        ("Quit", TAU * -0.25f32),
        ("Fun", 0f32),
        ("Reset", 0f32),
        ("Cheat", TAU * 0.25f32),
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

    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let text_style = TextStyle {
        font,
        font_size: 60.0,
        color: Color::BLACK,
    };
    let text_alignment = TextAlignment {
        vertical: VerticalAlign::Bottom,
        horizontal: HorizontalAlign::Center,
    };
    for (i, p) in positions.into_iter().enumerate() {
        if i < texts.len() {
            commands.spawn_bundle(Text2dBundle {
                text: Text::with_section(texts[i].0, text_style.clone(), text_alignment),
                text_2d_size: bevy::text::Text2dSize {
                    size: Size::new(0f32, 400f32),
                },
                transform: Transform::from_translation(p.0.extend(layer_z + 50f32))
                    .with_rotation(Quat::from_rotation_z(texts[i].1)),
                ..default()
            });
        }
        commands
            .spawn_bundle(SpriteBundle {
                texture: images.round_button.clone(),
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
                texture: images.arrow.clone(),
                transform: Transform::from_translation(p.0.extend(layer_z))
                    .with_rotation(Quat::from_rotation_z(p.1)),
                ..default()
            })
            .insert(p.2);
    }
}

fn handle_reaction_events(
    mut commands: Commands,
    images: Res<ButtonImages>,
    sounds: Res<ButtonSounds>,
    sounds_fr: Res<ButtonSoundsFr>,
    audio: Res<Audio>,
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
            FeedbackType::Fun => (
                Color::Hsla {
                    hue: time.seconds_since_startup() as f32 * 30f32,
                    saturation: 1f32,
                    lightness: 0.5f32,
                    alpha: 1f32,
                },
                20f32,
            ),
        };
        if ev.feedback != FeedbackType::Cheat {
            audio.play(
                if ev.feedback == FeedbackType::Bad {
                    &sounds_fr.sounds
                } else {
                    &sounds.sounds
                }[&ev.key]
                    .0
                    .clone(),
            );
        }
        for (t, r) in q_reactables.iter() {
            if r.key != ev.key {
                continue;
            }
            let particle = ParticleExplosion {
                location: t.translation.xy(),
                color,
            };
            let image_handle = match r.key {
                ArcadeInput::JoyRight
                | ArcadeInput::JoyDown
                | ArcadeInput::JoyLeft
                | ArcadeInput::JoyUp => images.arrow.clone(),
                _ => images.round_button.clone(),
            };
            commands
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite { color, ..default() },
                    texture: image_handle,
                    transform: Transform::from_translation(t.translation.xy().extend(layer))
                        .with_rotation(t.rotation),
                    ..default()
                })
                .insert(DestroyAfter::new(
                    time.seconds_since_startup() as f32 + 0.5f32,
                ));
            particles.send(particle);
        }
    }
}
