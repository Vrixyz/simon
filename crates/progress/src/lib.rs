use bevy::prelude::*;

pub struct ProgressPlugin;

#[derive(Component)]
pub struct ProgressRatio {
    /// Usually from 0 to 1
    pub ratio: f32,
}

#[derive(Component)]
pub struct ProgressText {
    pub max: usize,
    pub progress_entity: Entity,
}
#[derive(Component)]
pub struct ProgressScale {
    pub scale_base: Vec3,
    pub scale_mult: Vec3,
    pub progress_entity: Entity,
}

#[derive(Component, Default)]
pub struct ProgressTime {
    pub start_time: f32,
    pub duration: f32,
    pub active: bool,
}

impl Plugin for ProgressPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_progress)
            .add_system(update_text)
            .add_system(update_scale);
    }
}

fn update_progress(
    time: Res<Time>,
    mut q_progress: Query<(&mut ProgressTime, &mut ProgressRatio)>,
) {
    for (progress, mut ratio) in q_progress.iter_mut() {
        if !progress.active {
            continue;
        }
        ratio.ratio =
            (time.seconds_since_startup() as f32 - progress.start_time) / progress.duration;
    }
}
fn update_text(q_data: Query<&ProgressRatio>, mut q_progress: Query<(&mut Text, &ProgressText)>) {
    for (mut text, max) in q_progress.iter_mut() {
        if let Ok(progress) = q_data.get_component::<ProgressRatio>(max.progress_entity) {
            let number = (max.max as f32 * progress.ratio) as usize;
            text.sections[0].value = format!("{}/{}", number, max.max);
        }
    }
}
fn update_scale(
    q_data: Query<&ProgressRatio>,
    mut q_progress: Query<(&mut Transform, &ProgressScale)>,
) {
    for (mut transform, scale) in q_progress.iter_mut() {
        if let Ok(progress) = q_data.get_component::<ProgressRatio>(scale.progress_entity) {
            transform.scale =
                scale.scale_base + (scale.scale_mult * progress.ratio.clamp(0f32, 1f32));
        }
    }
}
