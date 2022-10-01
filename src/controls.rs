use bevy::prelude::*;
use crate::resources::StepController;

pub struct ControlPlugin;

impl Plugin for ControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(speed_control);
    }
}

fn speed_control(
    keyboard_input: Res<Input<KeyCode>>,
    mut step_controller: ResMut<StepController>,
) {
    if keyboard_input.just_pressed(KeyCode::Up) {
        step_controller.increase_speed();
    }
    if keyboard_input.just_pressed(KeyCode::Down) {
        step_controller.decrease_speed();
    }
}

