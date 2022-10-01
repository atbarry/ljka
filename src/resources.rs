use bevy::prelude::*;
use rand::{thread_rng, seq::SliceRandom};

pub struct StepController {
    steps_per_second: f32,
    current_step: u32,
    max_steps: u32,
    timer: StepTimer,
}

impl StepController {
    pub fn new(steps_per_second: f32, max_steps: u32) -> Self {
        Self {
            steps_per_second,
            current_step: 0,
            max_steps,
            timer: StepTimer::new(1. / steps_per_second),
        }
    }

    pub fn steps_next_frame(&mut self, time: &Time) -> u32 {
        let mut steps = 0;
        let finished = self.timer.tick(time.delta_seconds());

        if finished {
            steps = (self.timer.elapsed() * self.steps_per_second).round() as u32;
            self.timer.reset();
        }
     
        if steps > 20 {
            println!("Max steps reached");
            steps = 20;
        }

        steps
    }

    pub fn add_step(&mut self) -> bool {
        self.current_step += 1;

        if self.current_step >= self.max_steps {
            self.current_step = 0;
            true
        } else {
            false
        }
    }

    
    pub fn increase_speed(&mut self) {
        self.steps_per_second *= 2.0;
        self.timer = StepTimer::new(1. / self.steps_per_second);
    }

    pub fn decrease_speed(&mut self) {
        self.steps_per_second *= 0.5;
        self.timer = StepTimer::new(1. / self.steps_per_second);
    }
}


// Custom timer
struct StepTimer {
    elapsed: f32,
    duration: f32,
}

impl StepTimer {
    fn new(duration: f32) -> Self {
        Self {
            elapsed: 0.0,
            duration,
        }
    }

    fn tick(&mut self, delta: f32) -> bool {
        self.elapsed += delta;
        self.elapsed >= self.duration
    }

    fn elapsed(&self) -> f32 {
        self.elapsed
    }

    fn reset(&mut self) {
        self.elapsed = 0.0;
    }
}


pub struct Generation {
    complete: bool,
    num: u32,
}

impl Generation {
    pub fn new() -> Self {
        Self {
            complete: false,
            num: 0,
        }
    }

    pub fn completed(&mut self) {
        println!("Generation {} complete", self.num);
        self.complete = true;
        self.num += 1;
    }

    pub fn created(&mut self) {
        self.complete = false;
    }

    pub fn is_complete(&self) -> bool {
        self.complete
    }
    
}

