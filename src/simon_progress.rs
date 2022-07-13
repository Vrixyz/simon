use bevy::prelude::*;

use crate::progress::{ProgressRatio, ProgressScale, ProgressText, ProgressTime};

pub struct SimonProgressPlugin;

impl Plugin for SimonProgressPlugin {
    fn build(&self, app: &mut App) {
        //app.add_startup_system(test_startup);
    }
}

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

    let progress_entity = commands
        .spawn()
        .insert(ProgressRatio { ratio: 0.0 })
        .insert(ProgressTime {
            start_time: 0.0,
            duration: 10f32,
        })
        .id();

    commands
        .spawn_bundle(Text2dBundle {
            text: Text::with_section("0/50", text_style.clone(), text_alignment),
            transform: Transform::from_translation(Vec3::new(0f32, 400f32, 500f32)),
            ..default()
        })
        .insert(ProgressText {
            max: 50,
            progress_entity,
        });

    let width = 500f32;
    let height = 100f32;
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::BLACK,
            custom_size: Some(Vec2::new(width, height)),
            anchor: bevy::sprite::Anchor::CenterLeft,
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(-width / 2f32, 400f32, 498f32)),
        ..default()
    });
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::YELLOW_GREEN,
                custom_size: Some(Vec2::new(width, height)),
                anchor: bevy::sprite::Anchor::CenterLeft,
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(-width / 2f32, 400f32, 499f32)),
            ..default()
        })
        .insert(ProgressScale {
            scale_base: Vec3::new(0f32, 1f32, 1f32),
            scale_mult: Vec3::new(1f32, 0f32, 0f32),
            progress_entity,
        });
}
