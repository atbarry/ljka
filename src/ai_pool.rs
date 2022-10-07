use rand::{thread_rng, seq::SliceRandom, Rng, rngs::ThreadRng};
use bevy::{prelude::*, render::color};

use crate::{ai_logic::AI, targets::Target, AI_SPRITE_SCALE, SPAWN_RADII, NUM_AI, LEARN_RATE, STOP_MUTATE_THRESHOLD};



pub struct Pool{
    ai: Vec<AI>,
    id: u64,
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

            if !in_target {
                continue;
            } 

            if !ai.new_mutation{
                non_mutated_surviors += 1;
            }

            let mut i = ai.clone();
            i.new_mutation = false;
            reached_target.push(i);
        }

        dbg!(self.id);

        JudgeInfo{
            non_mutated_survivors: non_mutated_surviors,
            reached_target,
        }
    }

    pub fn update_pool(&mut self, info: JudgeInfo){
        let r_target = info.reached_target;
        let nm_surv = info.non_mutated_survivors;

        if r_target.len() == 0{
            return;
        }

        self.ai.clear();
        let mut rng = thread_rng();

        let get_old_ai = |rng:&mut ThreadRng| {
            r_target.choose(rng).unwrap().clone() 
        };
        let mutated_old_ai = |rng:&mut ThreadRng, id: u64| {
            r_target.choose(rng).unwrap().clone().learn_reproduce(LEARN_RATE, id)
        };
        let get_new_ai = |_rng:&mut ThreadRng, id: u64| {
            AI::new(id)
        };

        for _ in 0..NUM_AI{
            let old_ai = get_old_ai(&mut rng);
            self.ai.push(old_ai);
        }

        let threshold = nm_surv as f32 / NUM_AI as f32;
        let mutated_to_spawn = if threshold > STOP_MUTATE_THRESHOLD{0} else {NUM_AI - nm_surv};
        let mut mutated_target: Box<dyn FnMut(&mut ThreadRng, u64) -> AI>;

        if nm_surv < 50 {
            mutated_target = Box::new(get_new_ai);
        } else {
            mutated_target = Box::new(mutated_old_ai);
        }

        for _ in 0..mutated_to_spawn{
            let new_ai = mutated_target(&mut rng, self.next_id());
            self.ai.push(new_ai);
        }

        // shuffle the pool
        self.ai.shuffle(&mut rng);
    }

    pub fn spawn_ai(&self, commands: &mut Commands) {
        let mut spawn = |ai: &AI, x: f32, y: f32| {
            let color;
            let z_layer;
            
            match ai.new_mutation{
                false => {
                    color = Color::BLUE;
                    z_layer = 501.0;
                },
                true => {
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
        let mut pool = Self{ ai: Vec::new(), id: 0};

        for _ in 0..population{
            let ai = AI::new(pool.next_id());
            pool.ai.push(ai);
        }

        pool
    }

    fn next_id(&mut self) -> u64{
        self.id += 1;
        self.id
    }
}


