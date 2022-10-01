use ai_logic::Ai;
use bevy::{prelude::*, render::camera::ScalingMode};
use rand::Rng;
use ai_logic::GenePool;
use resources::{StepController, Generation};

mod ai_logic;
mod target;
mod resources;
mod controls;

const NUM_AI: u32 = 1000;
const RADII: f32 = 50.0;
const AI_SPRITE_SCALE: f32 = 1.0;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(StepController::new(50., 100))
            .insert_resource(Generation::new())
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
    mut gen: ResMut<Generation>,
    time: Res<Time>,
) {
    let steps = step_controller.steps_next_frame(&time);

    for _ in 0..steps {
        // ! Makes sure that the generation is not complete before running the step
        if step_controller.add_step(){
            gen.completed();
            break;
        }

        move_ai(&mut query);
    }
}

pub fn move_ai(query: &mut Query<(&mut Transform, &Ai)>) {
    // move in a random direction for now
    for (mut transform, _) in query.iter_mut() {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(-1.0..1.0);
        let y = rng.gen_range(-1.0..1.0);
        transform.translation += Vec3::new(1.0, y, 0.0);
    }
}

fn first_generation(mut commands: Commands, pool: Res<GenePool>) {
    spawn_ai(&mut commands, &pool);
}

fn next_generation(
    mut commands: Commands,
    mut gen: ResMut<Generation>,
    mut pool: ResMut<GenePool>,
    target_query: Query<&target::Target>,
    ai_query: Query<(Entity, &Transform, &Ai)>,
) {
    if !gen.is_complete() {
        return;
    }

    pool.add_successful_ai(&ai_query, &target_query);

    // Remove all ai
    for (entity, _, _) in ai_query.iter() {
        commands.entity(entity).despawn();
    }

    spawn_ai(&mut commands, &pool);
    gen.created();
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