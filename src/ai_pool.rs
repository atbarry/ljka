use rand::{thread_rng, seq::SliceRandom, Rng};
use bevy::{prelude::*, render::color};

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

    pub fn update_pool(&mut self, good_ai: Vec<AI>){
        if good_ai.len() == 0{
            return;
        }

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

        //TODO figure out if this has an impact 
        let learn_multiple = 1.0 - (2.0 * good_ai.len() as f32 / NUM_AI as f32);
        for _ in 0..bad_ai_to_spawn{
            let mut new_ai = good_ai.choose(&mut rng).unwrap().learn_reproduce(LEARN_RATE * learn_multiple);
            new_ai.good = false;
            self.ai.push(new_ai);
        }

        self.ai.shuffle(&mut rng);
    }

    pub fn judge_ai(
        &mut self,
        ai_query: &Query<(Entity,&Transform, &AI)>,
        target_query: &Query<&Target>,
    ) -> u32 {
        let mut good_ai = Vec::new();
        let mut num_survived = 0;

        for (_, transform, ai) in ai_query.iter() {
            let in_target = target_query.iter().any(|target| {
                let pos = transform.translation;
                let pos = Vec2::new(pos.x, pos.y);
                target.is_in_target(pos)
            });

            let mut i = ai.clone();
            if in_target {
                if i.good == true{
                    num_survived += 1;
                } else {
                    i.good = true;
                }

                good_ai.push(i);
            } else {
                i.good = false;
            }
        }

        self.update_pool(good_ai);
        num_survived
    }

    fn create_new_ai(&self) -> AI{
        let mut rng = thread_rng();
        let ai = self.ai.choose(&mut rng).unwrap();

        ai.clone()
    }

    pub fn spawn_ai(&self, commands: &mut Commands) {
        let mut spawn = |x: f32, y: f32| {
            let ai = self.create_new_ai();
            let color;
            let z_layer;
            
            match ai.good{
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
                        color: color,
                        ..Default::default()
                    },
                    transform: Transform {
                        translation: Vec3::new(x, y, z_layer),
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
    let t = NUM_AI as f32;
    let g = good_ai_num as f32;
    (2.0 * t - (2.0 * t) / (1.0 + (g / t).powf(growth_rate))) as u32

    // if good_ai_num == 0{
    //     0
    // } 
    // // else if good_ai_num < (NUM_AI as f32 / 2.0) as u32 {
    // //     NUM_AI / 2
    // // } 
    // else {
    //     NUM_AI / 2
    // }
}

