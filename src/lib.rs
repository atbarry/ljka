use ai_logic::Ai;
use bevy::{prelude::*, render::camera::ScalingMode};
use rand::Rng;
use ai_logic::GenePool;
use resources::{StepController, SimState};

mod ai_logic;
mod target;
mod resources;
mod controls;

const NUM_AI: u32 = 5000;
const RADII: f32 = 50.0;
const AI_SPRITE_SCALE: f32 = 1.0;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(StepController::new(50., 100))
            .insert_resource(SimState::new())
            .insert_resource(GenePool::new(NUM_AI))
            .add_startup_system(spawn_camera)
            .add_startup_system(first_generation)
            .add_plugin(target::TargetPlugin)
            .add_plugin(controls::ControlPlugin)
            .add_system(next_generation.before(run_steps))
            .add_system(run_steps);
    }
}

fn run_steps(
    mut query: Query<(&mut Transform, &Ai)>,
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



fn first_generation(mut commands: Commands, pool: Res<GenePool>) {
    spawn_ai(&mut commands, &pool);
}

fn next_generation(
    mut commands: Commands,
    mut gen: ResMut<SimState>,
    mut pool: ResMut<GenePool>,
    target_query: Query<&target::Target>,
    ai_query: Query<(Entity, &Transform, &Ai)>,
) {
    if !gen.gen_is_complete() {
        return;
    }

    let genes = pool.get_successful_ai(&ai_query, &target_query);
    
    gen.save_successful(genes.len() as u32);
    gen.save_plots();

    pool.add_genes(genes);

    // Remove all ai
    for (entity, _, _) in ai_query.iter() {
        commands.entity(entity).despawn();
    }

    spawn_ai(&mut commands, &pool);
    gen.created_next_gen();
}

fn spawn_ai(
    commands: &mut Commands,
    pool: &GenePool,
) {
    let mut spawn = |x: f32, y: f32| {
        let ai = pool.create_new_ai();
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(1.0, 1.0)),
                    color: Color::rgb_u8(10, 10, 255),
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(x, y, 500.0),
                    scale: Vec3::new(AI_SPRITE_SCALE, AI_SPRITE_SCALE, 1.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(ai);
    };

    let mut rng = rand::thread_rng();
    for _ in 0..NUM_AI {
        let x = rng.gen_range(-RADII..RADII);
        let y = rng.gen_range(-RADII..RADII);
        spawn(x, y);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle {
        projection: OrthographicProjection {
            far: 1000.0,
            scaling_mode: ScalingMode::Auto {
                min_width: RADII * 3.,
                min_height: RADII * 3.,
            },
            ..Default::default()
        },
        ..Default::default()
    });
}