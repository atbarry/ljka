use rand::{thread_rng, seq::SliceRandom, Rng};
use bevy::{prelude::*, render::color};

use crate::{ai_logic::AI, targets::Target, AI_SPRITE_SCALE, SPAWN_RADII, NUM_AI, LEARN_RATE};



pub struct Pool{
    ai: Vec<AI>,
}

pub struct JudgeInfo{
    pub non_mutated_survivors: u32,
    pub reached_target: Vec<AI>,
}

impl Pool{
    pub fn judge_ai(
        &mut self,
        ai_query: &Query<(Entity,&Transform, &AI)>,
        target_query: &Query<&Target>,
    ) -> JudgeInfo {
        let mut reached_target = Vec::new();
        let mut non_mutated_surviors = 0;

        for (_, transform, ai) in ai_query.iter() {
            let in_target = target_query.iter().any(|target| {
                let pos = transform.translation;
                let pos = Vec2::new(pos.x, pos.y);
                target.is_in_target(pos)
            });

            if in_target {
                if ai.old_mutation{
                    non_mutated_surviors += 1;
                }
                let mut i = ai.clone();
                i.old_mutation = true;
                reached_target.push(i);
            } 
        }

        JudgeInfo{
            non_mutated_survivors: non_mutated_surviors,
            reached_target,
        }
    }

    pub fn update_pool(&mut self, info: JudgeInfo){
        let r_target = info.reached_target;
        if r_target.len() == 0{
            return;
        }

        self.ai.clear();


        let old_ai_to_spawn = num_good_ai_to_spawn(
            r_target.len() as u32,
             0.5
        );

        let mutated_ai_to_spawn;
        if info.non_mutated_survivors as f32 > NUM_AI as f32 / 2.1{
            mutated_ai_to_spawn = 0;
        } else {
            mutated_ai_to_spawn = NUM_AI - old_ai_to_spawn;
        }
        
        dbg!(old_ai_to_spawn, mutated_ai_to_spawn);

        let mut rng = thread_rng();

        for _ in 0..old_ai_to_spawn{
            let new_ai = r_target.choose(&mut rng).unwrap().clone();
            self.ai.push(new_ai);
        }


        //TODO figure out if this has an impact 
        // let learn_multiple = 1.0 - (2.0 * good_ai.len() as f32 / NUM_AI as f32);
        for _ in 0..mutated_ai_to_spawn{
            let new_ai = r_target.choose(&mut rng).unwrap().learn_reproduce(LEARN_RATE);
            self.ai.push(new_ai);
        }

        self.ai.shuffle(&mut rng);
    }

    pub fn spawn_ai(&self, commands: &mut Commands) {
        let mut spawn = |ai: &AI, x: f32, y: f32| {
            let color;
            let z_layer;
            
            match ai.old_mutation{
                true => {
                    color = Color::BLUE;
                    z_layer = 501.0;
                },
                false => {
                    color = Color::RED;
                    z_layer = 500.0;
                }
            }
            commands
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(1.0, 1.0)),
                        color,
                        ..Default::default()
                    },
                    transform: Transform {
                        translation: Vec3::new(x, y, z_layer),
                        scale: Vec3::new(AI_SPRITE_SCALE, AI_SPRITE_SCALE, 1.0),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(ai.clone());
        };
    
        let mut rng = rand::thread_rng();
        for ai in self.ai.iter() {
            let x = rng.gen_range(-SPAWN_RADII..SPAWN_RADII);
            let y = rng.gen_range(-SPAWN_RADII..SPAWN_RADII);
            spawn(ai, x, y);
        }
    }

    pub fn new(population: u32) -> Self{
        let mut ai = Vec::new();
        for _ in 0..population{
            ai.push(AI::new());
        }

        Self{
            ai,
        }
    }
}

fn num_good_ai_to_spawn(good_ai_num: u32, growth_rate: f32) -> u32 {
    // let t = NUM_AI as f32;
    // let g = good_ai_num as f32;
    // (2.0 * t - (2.0 * t) / (1.0 + (g / t).powf(growth_rate))) as u32
    
    NUM_AI/ 2
}

