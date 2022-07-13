use std::{fs, path::Path};

use bevy::prelude::*;

use crate::{
    progress::{ProgressRatio, ProgressScale, ProgressTime},
    UserProgress, UserSequence,
};

pub struct SimonResetPlugin;

impl Plugin for SimonResetPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(startup)
            .insert_resource(ResetState(false))
            .add_system(progress_visibility)
            .add_system(timer_toggle)
            .add_system(reset_full.after(timer_toggle));
    }
}

pub struct ResetState(pub bool);

#[derive(Component)]
struct SimonReset;

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
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

    let progress_entity = commands
        .spawn()
        .insert(ProgressRatio { ratio: 0.0 })
        .insert(ProgressTime {
            start_time: 0.0,
            duration: 3.0,
            active: false,
        })
        .insert(SimonReset)
        .id();
    commands
        .spawn_bundle(Text2dBundle {
            text: Text::with_section(
                "Will reset game data...",
                text_style.clone(),
                text_alignment,
            ),
            transform: Transform::from_translation(Vec3::new(0f32, -500f32, 500f32)),
            ..default()
        })
        .insert(SimonReset);

    let width = 600f32;
    let height = 100f32;
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::BLACK,
                custom_size: Some(Vec2::new(width, height)),
                anchor: bevy::sprite::Anchor::CenterLeft,
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(-width / 2f32, -500f32, 498f32)),
            ..default()
        })
        .insert(SimonReset);
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::RED,
                custom_size: Some(Vec2::new(width, height)),
                anchor: bevy::sprite::Anchor::CenterLeft,
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(-width / 2f32, -500f32, 499f32)),
            ..default()
        })
        .insert(ProgressScale {
            scale_base: Vec3::new(0f32, 1f32, 1f32),
            scale_mult: Vec3::new(1f32, 0f32, 0f32),
            progress_entity,
        })
        .insert(SimonReset);
}

fn progress_visibility(
    progress: Res<ResetState>,
    mut q_toggles: Query<&mut Visibility, With<SimonReset>>,
) {
    if progress.is_changed() {
        for mut v in q_toggles.iter_mut() {
            v.is_visible = progress.0;
            if !v.is_visible {}
        }
    }
}
fn timer_toggle(
    time: Res<Time>,
    progress: Res<ResetState>,
    mut q: Query<&mut ProgressTime, With<SimonReset>>,
) {
    if progress.is_changed() {
        for mut p in q.iter_mut() {
            p.active = progress.0;
            if p.active {
                p.start_time = time.seconds_since_startup() as f32;
            }
        }
    }
}

fn reset_full(
    mut state: ResMut<ResetState>,
    mut seq: ResMut<UserSequence>,
    mut prog: ResMut<UserProgress>,
    q: Query<&ProgressRatio, With<SimonReset>>,
) {
    if state.0 {
        if let Ok(progress) = q.get_single() {
            if progress.ratio >= 1f32 {
                let file_path = Path::new("./current.json");
                fs::remove_file(file_path);
                *seq = default();
                *prog = default();
                state.0 = false;
            }
        }
    }
}
