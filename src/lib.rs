use ai_logic::AI;
use bevy::{prelude::*, render::camera::ScalingMode};
use ai_pool::Pool;
use resources::{StepController, SimState};

mod ai_logic;
mod ai_pool;
mod targets;
mod resources;
mod controls;

const NUM_AI: u32 = 20000;
const SPAWN_RADII: f32 = 75.0;
const AI_SPRITE_SCALE: f32 = 0.75;

const MOVE_SPEED : f32 = 4.0;
const LEARN_RATE : f32 = 0.1;
const NETWORK_LAYERS: [usize; 1] = [2];

const SIM_SPEED : f32 = 25.0;
const SIM_GEN_LENGTH : u32 = 100;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(StepController::new(SIM_SPEED, SIM_GEN_LENGTH))
            .insert_resource(SimState::new())
            .insert_resource(Pool::new(NUM_AI))
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
    mut step_controller: ResMut<StepController>,
    mut gen: ResMut<SimState>,
    time: Res<Time>,
) {
    let steps = step_controller.steps_next_frame(&time);

    for _ in 0..steps {
        // ! Makes sure that the generation is not complete before running the step
        if step_controller.add_step(){
            gen.completed();
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
    mut gen: ResMut<SimState>,
    mut pool: ResMut<Pool>,
    target_query: Query<&targets::Target>,
    ai_query: Query<(Entity, &Transform, &AI)>,
) {
    if !gen.gen_is_complete() {
        return;
    }

    let (good, bad) = pool.judge_ai(&ai_query, &target_query);
    
    gen.save_successful(good.len() as u32);
    gen.save_plots();

    pool.update_pool(good, bad);

    // Remove all ai
    for (entity, _, _) in ai_query.iter() {
        commands.entity(entity).despawn();
    }

    pool.spawn_ai(&mut commands);
    gen.created_next_gen();
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