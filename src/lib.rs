use ai_logic::AI;
use bevy::{prelude::*, render::camera::ScalingMode};
use ai_pool::Pool;
use resources::SimState;

mod ai_logic;
mod ai_pool;
mod targets;
mod resources;
mod controls;

const NUM_AI: u32 = 5000;
const SPAWN_RADII: f32 = 50.0;
const AI_SPRITE_SCALE: f32 = 0.75;

const MOVE_SPEED : f32 = 4.0;
const LEARN_RATE : f32 = 1.0; // don't go less than 0.5 i think 
const NETWORK_LAYERS: [usize; 2] = [7, 2];

const SIM_SPEED : f32 = 50.0;
const SIM_GEN_LENGTH : u32 = 75;
const STOP_MUTATE_THRESHOLD: f32 = 0.9;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SimState::new())
            .insert_resource(Pool::new(NUM_AI*2))
            .add_startup_system(spawn_camera)
            .add_startup_system(first_generation)
            .add_plugin(targets::TargetPlugin)
            .add_plugin(controls::ControlPlugin)
            .add_system(next_generation.before(run_steps))
            .add_system(run_steps);
    }
}

fn run_steps(
    mut query: Query<(&mut Transform, &AI)>,
    mut sim: ResMut<SimState>,
    time: Res<Time>,
) {
    if sim.is_paused() {
        return;
    }

    let step_controller = &mut sim.step_controller;
    let steps = step_controller.steps_next_frame(&time);

    for _ in 0..steps {
        // ! Makes sure that the generation is not complete before running the step
        if step_controller.add_step(){
            sim.gen_completed();
            break;
        }

        ai_logic::move_ai(&mut query);
    }
}

fn first_generation(mut commands: Commands, pool: Res<Pool>) {
    pool.spawn_ai(&mut commands);
}

fn next_generation(
    mut commands: Commands,
    mut sim: ResMut<SimState>,
    mut pool: ResMut<Pool>,
    target_query: Query<&targets::Target>,
    ai_query: Query<(Entity, &Transform, &AI)>,
) {
    // Only runs if the generation is complete
    if !sim.gen_is_complete() {
        return;
    }

    // Judge the AI
    let info = pool.judge_ai(&ai_query, &target_query);
    
    // Updating statistics 
    sim.save_successful(info.non_mutated_survivors);
    sim.save_plots();

    // Actually updating the pool
    pool.update_pool(info);

    // Remove all ai
    for (entity, _, _) in ai_query.iter() {
        commands.entity(entity).despawn();
    }

    // Spawn new ai
    pool.spawn_ai(&mut commands);
    sim.created_next_gen();
}


fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle {
        projection: OrthographicProjection {
            far: 1000.0,
            scaling_mode: ScalingMode::Auto {
                min_width: SPAWN_RADII * 3.,
                min_height: SPAWN_RADII * 3.,
            },
            ..Default::default()
        },
        ..Default::default()
    });
}