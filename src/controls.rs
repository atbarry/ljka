use bevy::prelude::*;
use crate::resources::SimState;

pub struct ControlPlugin;

impl Plugin for ControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(speed_control)
            .add_system(pause_control);
    }
}

fn speed_control(
    keyboard_input: Res<Input<KeyCode>>,
    mut sim: ResMut<SimState>,
) {
    let step_controller = &mut sim.step_controller;
    if keyboard_input.just_pressed(KeyCode::Up) {
        step_controller.change_speed(1.25);
    }
    if keyboard_input.just_pressed(KeyCode::Down) {
        step_controller.change_speed(0.8);
    }
}

fn pause_control(
    keyboard_input: Res<Input<KeyCode>>,
    mut sim: ResMut<SimState>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        sim.toggle_pause();
    }
}


