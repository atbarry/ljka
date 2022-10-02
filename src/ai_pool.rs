use rand::{thread_rng, seq::SliceRandom, Rng};
use bevy::prelude::*;

use crate::{ai_logic::{NeuralNetwork, AI}, target::Target, AI_SPRITE_SCALE, RADII, NUM_AI, LEARN_RATE};



pub struct Pool{
    ai: Vec<NeuralNetwork>,
}

impl Pool{
    pub fn new(population: u32) -> Self{
        let mut ai = Vec::new();
        for _ in 0..population{
            ai.push(NeuralNetwork::default());
        }

        Self{
            ai,
        }
    }

    pub fn update_pool(&mut self, good_ai: Vec<NeuralNetwork>){
        // randomly select genes to replace
        self.ai.shuffle(&mut thread_rng());
        self.ai.truncate(self.ai.len() - good_ai.len());

        self.ai.extend(good_ai);

        // randomly mutate genes
        for i in 0..self.ai.len(){
            // get the log of i
            // let log = (i as f32).log2();
            self.ai[i].mutate(LEARN_RATE);
        }

    }

    pub fn get_successful_ai(
        &mut self,
        ai_query: &Query<(Entity,&Transform, &AI)>,
        target_query: &Query<&Target>,
    ) -> Vec<NeuralNetwork> {
        let mut ai_pool = Vec::new();

        for (_, transform, ai) in ai_query.iter() {
            let in_target = target_query.iter().any(|target| {
                let pos = transform.translation;
                let pos = Vec2::new(pos.x, pos.y);
                target.is_in_target(pos)
            });

            if in_target {
                ai_pool.push(ai.brain.clone());
            }
        }

        ai_pool
    }

    fn create_new_ai(&self) -> AI{
        let mut rng = thread_rng();
        let gene = self.ai.choose(&mut rng).unwrap();
        AI{
            brain: gene.clone(),
        }
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
            let x = rng.gen_range(-RADII..RADII);
            let y = rng.gen_range(-RADII..RADII);
            spawn(x, y);
        }
    }
}