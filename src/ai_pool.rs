use rand::{thread_rng, seq::SliceRandom, Rng};
use bevy::prelude::*;

use crate::{ai_logic::AI, targets::Target, AI_SPRITE_SCALE, SPAWN_RADII, NUM_AI, LEARN_RATE};



pub struct Pool{
    ai: Vec<AI>,
}

impl Pool{
    pub fn new(population: u32) -> Self{
        let mut ai = Vec::new();
        for _ in 0..population{
            ai.push(AI::new());
        }

        Self{
            ai,
        }
    }

    pub fn update_pool(&mut self, good_ai: Vec<AI>, bad_ai: Vec<AI>){
        self.ai.clear();


        let good_ai_to_spawn = num_good_ai_to_spawn(
            good_ai.len() as u32,
             0.5
        );

        let bad_ai_to_spawn = NUM_AI - good_ai_to_spawn;

        dbg!(good_ai_to_spawn, bad_ai_to_spawn);

        let mut rng = thread_rng();

        for _ in 0..good_ai_to_spawn{
            let mut new_ai = good_ai.choose(&mut rng).unwrap().clone();
            self.ai.push(new_ai);
        }

        for _ in 0..bad_ai_to_spawn{
            let mut new_ai = bad_ai.choose(&mut rng).unwrap().learn_reproduce(LEARN_RATE);
            self.ai.push(new_ai);
        }

        self.ai.shuffle(&mut rng);
    }

    pub fn judge_ai(
        &mut self,
        ai_query: &Query<(Entity,&Transform, &AI)>,
        target_query: &Query<&Target>,
    ) -> (Vec<AI>, Vec<AI>) {
        let mut good_ai = Vec::new();
        let mut bad_ai = Vec::new();

        for (_, transform, ai) in ai_query.iter() {
            let in_target = target_query.iter().any(|target| {
                let pos = transform.translation;
                let pos = Vec2::new(pos.x, pos.y);
                target.is_in_target(pos)
            });

            if in_target {
                good_ai.push(ai.clone());
            } else {
                bad_ai.push(ai.clone());
            }
        }

        (good_ai, bad_ai)
    }

    fn create_new_ai(&self) -> AI{
        let mut rng = thread_rng();
        let ai = self.ai.choose(&mut rng).unwrap();

        ai.clone()
    }

    pub fn spawn_ai(&self, commands: &mut Commands) {
        let mut spawn = |x: f32, y: f32| {
            let ai = self.create_new_ai();
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
            let x = rng.gen_range(-SPAWN_RADII..SPAWN_RADII);
            let y = rng.gen_range(-SPAWN_RADII..SPAWN_RADII);
            spawn(x, y);
        }
    }
}

fn num_good_ai_to_spawn(good_ai_num: u32, growth_rate: f32) -> u32 {
    // let t = NUM_AI as f32;
    // let g = good_ai_num as f32;
    // (2.0 * t - (2.0 * t) / (1.0 + (g / t).powf(growth_rate))) as u32

    if good_ai_num == 0{
        0
    } else if good_ai_num < 10 {
        NUM_AI
    }
    else if good_ai_num < (NUM_AI as f32 / 2.2) as u32 {
        NUM_AI / 2
    } else {
        NUM_AI
    }
}

