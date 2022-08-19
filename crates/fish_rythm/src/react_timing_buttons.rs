use bevy::prelude::*;
use rust_arcade::bevy_rust_arcade::ArcadeInput;
use rust_arcade_display::InputReaction;

use crate::timing_button::{TimingButton, TimingButtonExpired, TimingButtonStartTime};

pub struct ReactTimingButtons;

impl Plugin for ReactTimingButtons {
    fn build(&self, app: &mut App) {
        app.add_event::<ValidateButtonEvent>();
        app.add_system_to_stage(CoreStage::PostUpdate, validate_buttons);
        app.add_system_to_stage(CoreStage::PostUpdate, react_to_expired_buttons);
    }
}

#[derive(Component)]
pub struct ArcadeKey {
    pub key: ArcadeInput,
}

pub struct ValidateButtonEvent(pub ArcadeInput);

fn validate_buttons(
    mut commands: Commands,
    mut feedback_events: EventWriter<InputReaction>,
    mut ev_validate: EventReader<ValidateButtonEvent>,
    q_button: Query<(Entity, &TimingButton, &TimingButtonStartTime, &ArcadeKey)>,
) {
    for ev in ev_validate.iter() {
        let mut best_button = None;
        for (e, t, start, k) in q_button.iter() {
            if k.key == ev.0 {
                if best_button.is_none() {
                    best_button = Some((e, t, start, k));
                } else if let Some(last_best) = &best_button {
                    // Only apply reaction to the first spawned corresponding button.
                    // it won't be the best choice if another faster timing button spawns after the first one,
                    // but it should be avoided because it makes the order for the user not obvious.
                    if start.start_time < last_best.2.start_time {
                        best_button = Some((e, t, start, k));
                    }
                }
            }
        }
        if let Some((e, t, s, k)) = best_button {
            feedback_events.send(InputReaction {
                key: k.key.clone(),
                feedback: match t {
                    TimingButton::TooEarly => rust_arcade_display::FeedbackType::Bad,
                    TimingButton::BadEarly => rust_arcade_display::FeedbackType::New,
                    TimingButton::GoodEarly => rust_arcade_display::FeedbackType::Good,
                    TimingButton::Perfect => rust_arcade_display::FeedbackType::Menu,
                    TimingButton::GoodLate => rust_arcade_display::FeedbackType::Good,
                    TimingButton::BadLate => rust_arcade_display::FeedbackType::New,
                    TimingButton::TooLate => rust_arcade_display::FeedbackType::Bad,
                },
            });
            commands.entity(e).despawn();
        } else {
            feedback_events.send(InputReaction {
                key: ev.0.clone(),
                feedback: rust_arcade_display::FeedbackType::Cheat,
            });
        }
    }
}

fn react_to_expired_buttons(
    mut commands: Commands,
    mut feedback_events: EventWriter<InputReaction>,
    mut ev_expired: EventReader<TimingButtonExpired>,
    q_position: Query<(&ArcadeKey)>,
) {
    for r in ev_expired.iter() {
        if let Ok(p) = q_position.get(r.0) {
            feedback_events.send(InputReaction {
                key: p.key.clone(),
                feedback: rust_arcade_display::FeedbackType::Cheat,
            });
        }
        commands.entity(r.0).despawn();
    }
}
