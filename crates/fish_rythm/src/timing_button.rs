use bevy::{prelude::*, utils::HashMap};

pub enum TimingButton {
    TooEarly,
    BadEarly,
    GoodEarly,
    Perfect,
    GoodLate,
    BadLate,
    TooLate,
}

pub struct TimingButtonPath {
    pub path: Vec<Vec3>,
}

pub struct TimingMapping {
    pub value: TimingButton,
    pub expiry: f32,
}

pub struct TimingButtonRatios {
    /// Value to apply if timing is inferior to item.1
    /// Sorted by  ascendant `map.expiry`
    pub map: Vec<TimingMapping>,
}

impl Plugin for TimingButton {
    fn build(&self, app: &mut App) {
        // TODO: move from origin to perfectPosition to missedPosition
        // TODO: react to taps, send event too_soon, perfect, missed
        app.add_system(move_timing_buttons)
            .add_system(update_timing_value);
    }
}

fn move_timing_buttons() {}
fn update_timing_value() {}
