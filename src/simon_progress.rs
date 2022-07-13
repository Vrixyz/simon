use bevy::prelude::*;

use crate::{
    progress::{ProgressRatio, ProgressScale, ProgressText},
    UserProgress, UserSequence,
};

pub struct SimonProgressPlugin;

impl Plugin for SimonProgressPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(test_startup)
            .add_system(progress_visibility)
            .add_system(new_key_visibility)
            .add_system(progress_update);
    }
}

#[derive(Component)]
struct ProgressSimon;
#[derive(Component)]
struct ProgressSimonNewKey;

fn test_startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let text_style = TextStyle {
        font,
        font_size: 60.0,
        color: Color::WHITE,
    };
    let text_alignment = TextAlignment {
        vertical: VerticalAlign::Center,
        horizontal: HorizontalAlign::Center,
    };

    commands.spawn_bundle(Text2dBundle {
        text: Text::with_section(
            "'Simon' Game",
            TextStyle {
                font_size: 80.0,
                color: Color::rgb(0.7, 0.7, 1.0),
                ..text_style.clone()
            },
            text_alignment,
        ),
        transform: Transform::from_translation(Vec3::new(0f32, 700f32, 500f32)),
        ..default()
    });
    commands
        .spawn_bundle(Text2dBundle {
            text: Text::with_section(
                "Share your progress with #metalmancy",
                text_style.clone(),
                text_alignment,
            ),
            transform: Transform::from_translation(Vec3::new(0f32, 600f32, 500f32)),
            ..default()
        })
        .insert(ProgressSimon);
    commands
        .spawn_bundle(Text2dBundle {
            text: Text::with_section("Push ANY key :D !", text_style.clone(), text_alignment),
            transform: Transform::from_translation(Vec3::new(0f32, 500f32, 500f32)),
            ..default()
        })
        .insert(ProgressSimonNewKey);

    let progress_entity = commands
        .spawn()
        .insert(ProgressRatio { ratio: 0.0 })
        .insert(ProgressSimon)
        .id();
    commands
        .spawn_bundle(Text2dBundle {
            text: Text::with_section("0/50", text_style.clone(), text_alignment),
            transform: Transform::from_translation(Vec3::new(0f32, 500f32, 500f32)),
            ..default()
        })
        .insert(ProgressText {
            max: 50,
            progress_entity,
        })
        .insert(ProgressSimon);

    let width = 500f32;
    let height = 100f32;
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::BLACK,
                custom_size: Some(Vec2::new(width, height)),
                anchor: bevy::sprite::Anchor::CenterLeft,
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(-width / 2f32, 500f32, 498f32)),
            ..default()
        })
        .insert(ProgressSimon);
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::YELLOW_GREEN,
                custom_size: Some(Vec2::new(width, height)),
                anchor: bevy::sprite::Anchor::CenterLeft,
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(-width / 2f32, 500f32, 499f32)),
            ..default()
        })
        .insert(ProgressScale {
            scale_base: Vec3::new(0f32, 1f32, 1f32),
            scale_mult: Vec3::new(1f32, 0f32, 0f32),
            progress_entity,
        })
        .insert(ProgressSimon);
}

fn progress_visibility(
    progress: Res<UserProgress>,
    sequence: Res<UserSequence>,
    mut q_toggles: Query<&mut Visibility, With<ProgressSimon>>,
) {
    if progress.is_changed() {
        if progress.index == 0 {
            for mut v in q_toggles.iter_mut() {
                v.is_visible = true;
            }
        }
        if progress.index == sequence.sequence.len() {
            for mut v in q_toggles.iter_mut() {
                v.is_visible = false;
            }
        }
    }
}
fn new_key_visibility(
    progress: Res<UserProgress>,
    sequence: Res<UserSequence>,
    mut q_toggles: Query<&mut Visibility, With<ProgressSimonNewKey>>,
) {
    if progress.is_changed() {
        if progress.index == 0 {
            for mut v in q_toggles.iter_mut() {
                v.is_visible = false;
            }
        }
        if progress.index == sequence.sequence.len() {
            for mut v in q_toggles.iter_mut() {
                v.is_visible = true;
            }
        }
    }
}

fn progress_update(
    progress: Res<UserProgress>,
    sequence: Res<UserSequence>,
    mut q_ratio: Query<&mut ProgressRatio, With<ProgressSimon>>,
    mut q_text: Query<&mut ProgressText, With<ProgressSimon>>,
) {
    if progress.is_changed() {
        let len = sequence.sequence.len();
        let ratio = if len > 0 {
            progress.index as f32 / len as f32
        } else {
            0f32
        };
        for mut r in q_ratio.iter_mut() {
            r.ratio = ratio;
        }
    }
    if sequence.is_changed() {
        let len = sequence.sequence.len();
        for mut t in q_text.iter_mut() {
            t.max = len;
        }
    }
}
