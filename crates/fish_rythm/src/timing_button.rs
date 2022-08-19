use bevy::prelude::*;

pub struct TimingButtonsPlugin;

impl Plugin for TimingButtonsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TimingButtonExpired>();
        app.add_system(move_timing_buttons)
            .add_system(update_timing_value)
            .add_system(add_start_time);
    }
}

#[derive(Component, PartialEq, Clone)]
pub enum TimingButton {
    TooEarly,
    BadEarly,
    GoodEarly,
    Perfect,
    GoodLate,
    BadLate,
    TooLate,
}

#[derive(Component)]
pub struct TimingButtonPath {
    pub path: Vec<Vec3>,
}

pub struct TimingButtonExpired(pub Entity);

pub struct DelayTimings {
    pub delay_before_perfect: f32,
    pub delay_before_final_too_late: f32,
}

#[derive(Component)]
pub struct TimingButtonStartTime {
    pub start_time: f32,
}

#[derive(Component)]
pub struct PathDriver {
    pub path_target_index: usize,
    pub speed: f32,
}

#[derive(Component, Clone)]
pub struct TimingMapping {
    pub expiry: f32,
    pub value: TimingButton,
}

impl TimingMapping {
    pub fn new(expiry: f32, value: TimingButton) -> Self {
        Self { expiry, value }
    }
}

#[derive(Component, Clone)]
pub struct TimingButtonMappings {
    /// Value to apply if timing is inferior to item.1
    /// Sorted by  ascendant `map.expiry`
    pub map: Vec<TimingMapping>,
}

fn add_start_time(
    mut commands: Commands,
    time: Res<Time>,
    q_new_timing_button: Query<Entity, Added<TimingButton>>,
) {
    for e in q_new_timing_button.iter() {
        commands.entity(e).insert(TimingButtonStartTime {
            start_time: time.seconds_since_startup() as f32,
        });
    }
}

fn move_timing_buttons(
    time: Res<Time>,
    mut q_path: Query<(&TimingButtonPath, &mut PathDriver, &mut Transform)>,
) {
    for (path, mut driver, mut transform) in q_path.iter_mut() {
        if path.path.len() <= driver.path_target_index {
            continue;
        }
        let mut distance_to_make = driver.speed as f32 * time.delta_seconds();
        let mut new_pos = transform.translation;
        while distance_to_make > 0f32 {
            if path.path.len() <= driver.path_target_index {
                break;
            }
            let target_pos = path.path[driver.path_target_index];
            if target_pos == new_pos {
                driver.path_target_index += 1;
                continue;
            }
            let to_target = target_pos - new_pos;
            let distance = to_target.length();
            if distance_to_make < distance {
                new_pos += to_target.normalize() * distance_to_make;
                break;
            }
            distance_to_make -= distance;
            new_pos = target_pos;
            driver.path_target_index += 1;
        }
        transform.translation = new_pos;
    }
}

fn update_timing_value(
    time: Res<Time>,
    delay_timings: Res<DelayTimings>,
    mut event_expired: EventWriter<TimingButtonExpired>,
    mut q_new_timing_button: Query<(
        Entity,
        &TimingButtonStartTime,
        &TimingButtonMappings,
        &mut TimingButton,
    )>,
) {
    for (e, start, map, mut state) in q_new_timing_button.iter_mut() {
        let elapsed = time.seconds_since_startup() as f32 - start.start_time;
        for mapping in map.map.iter() {
            if elapsed < mapping.expiry && *state != mapping.value {
                *state = mapping.value.clone();
                break;
            }
        }
        if delay_timings.delay_before_final_too_late <= elapsed {
            event_expired.send(TimingButtonExpired(e));
        }
    }
}
