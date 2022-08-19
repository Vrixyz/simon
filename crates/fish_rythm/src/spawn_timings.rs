use std::time::Duration;

use bevy::{math::Vec3Swizzles, prelude::*};
use rand::Rng;
use rust_arcade::bevy_rust_arcade::ArcadeInput;
use rust_arcade_display::Reactable;

use crate::{
    react_timing_buttons::ArcadeKey,
    timing_button::{
        DelayTimings, PathDriver, TimingButton, TimingButtonMappings, TimingButtonPath,
        TimingMapping,
    },
};

pub struct SpawnTiming;

#[derive(Component)]
pub struct SpawnWaves {
    pub timer: Timer,
}

pub struct MappingHelper {
    pub mapping_raw: TimingButtonMappings,
    pub ideal_perfect_time: f32,
}

impl Plugin for SpawnTiming {
    fn build(&self, app: &mut App) {
        app.insert_resource(DelayTimings {
            delay_before_perfect: 2f32,
            delay_before_final_too_late: 4f32,
        });
        app.add_system(spawn_timing_buttons);
        app.add_startup_system(spawn_timing_setup);
        app.insert_resource(MappingHelper {
            mapping_raw: TimingButtonMappings {
                map: vec![
                    TimingMapping::new(0.2, TimingButton::TooEarly),
                    TimingMapping::new(0.4, TimingButton::BadEarly),
                    TimingMapping::new(0.6, TimingButton::GoodEarly),
                    TimingMapping::new(0.7, TimingButton::Perfect),
                    TimingMapping::new(0.9, TimingButton::GoodLate),
                    TimingMapping::new(1.1, TimingButton::BadLate),
                    TimingMapping::new(1.3, TimingButton::TooLate),
                ],
            },
            ideal_perfect_time: 0.65,
        });
    }
}

fn spawn_timing_setup(mut commands: Commands) {
    commands.spawn().insert(SpawnWaves {
        timer: Timer::new(Duration::from_secs_f32(1f32), true),
    });
}

fn spawn_timing_buttons(
    mappingHelper: Res<MappingHelper>,
    mut commands: Commands,
    time: Res<Time>,
    mut q_spawn_waves: Query<(&mut SpawnWaves)>,
    mut q_reactable: Query<(&Transform, &Reactable)>,
) {
    for (mut s) in q_spawn_waves.iter_mut() {
        s.timer.tick(time.delta());
        if s.timer.just_finished() {
            let mut rng = rand::thread_rng();
            let forbidden_inputs = vec![
                ArcadeInput::ButtonFront1,
                ArcadeInput::ButtonFront2,
                ArcadeInput::ButtonLeftSide,
                ArcadeInput::ButtonRightSide,
            ];
            let elligible_reactables = q_reactable
                .iter()
                .filter(|(t, r)| !forbidden_inputs.contains(&r.key))
                .collect::<Vec<_>>();
            let mut reactables_iter = elligible_reactables.iter();
            let reactable_count = reactables_iter.len();
            if reactable_count == 0 {
                continue;
            }
            let target = reactables_iter
                .nth(rng.gen_range(0..reactable_count))
                .expect("reactables_iter and reactable_count should have been checked before.");

            let position_z = 25f32;
            let speed = 100f32;
            let delay_to_hit = 2f32;
            let target_position = target.0.translation.xy();
            let random_offset = (Vec2::new(
                rng.gen_range(-150f32..150f32),
                rng.gen_range(-150f32..150f32),
            )
            .normalize_or_zero()
                * speed
                * delay_to_hit);
            let random_position = (random_offset + target_position).extend(position_z);
            let path = vec![
                random_position,
                random_position - (random_offset * 2f32).extend(position_z),
            ];

            commands
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: Color::GOLD,
                        custom_size: Some(Vec2::splat(150f32)),
                        ..default()
                    },
                    transform: Transform::from_translation(random_position),
                    ..default()
                })
                .insert(ArcadeKey {
                    key: target.1.key.clone(),
                })
                .insert(TimingButton::TooEarly)
                .insert(TimingButtonPath { path })
                .insert(TimingButtonMappings {
                    map: mappingHelper
                        .mapping_raw
                        .map
                        .iter()
                        .map(|v| {
                            TimingMapping::new(
                                v.expiry - mappingHelper.ideal_perfect_time + delay_to_hit,
                                v.value.clone(),
                            )
                        })
                        .collect(),
                })
                .insert(PathDriver {
                    path_target_index: 0,
                    speed,
                });
        }
    }
}
