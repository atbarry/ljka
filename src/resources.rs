use bevy::prelude::*;
use plotters::{prelude::*, style::Color};


use crate::{NUM_AI, SIM_SPEED, SIM_GEN_LENGTH};

pub struct SimState {
    gen_is_complete: bool,
    gen_num: u32,
    successful_num: Vec<u32>,
    plot_path: String,
    pub step_controller: StepController,
    is_paused: bool,
}

pub struct StepController {
    steps_per_second: f32,
    current_step: u32,
    max_steps: u32,
    timer: StepTimer,
}




impl SimState {
    pub fn new() -> Self {
        // gets time since epoch in seconds
        let time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let plot_path = format!("plots/plot_{}.png", time);
        let step_controller = StepController::new(SIM_SPEED, SIM_GEN_LENGTH);
    
        Self {
            gen_is_complete: false,
            gen_num: 0,
            successful_num: Vec::new(),
            plot_path,
            step_controller,
            is_paused: true,
        }
    }
    
    pub fn toggle_pause(&mut self) {
        self.is_paused = !self.is_paused;
    }

    pub fn is_paused(&self) -> bool { self.is_paused }

    pub fn gen_completed(&mut self) {
        self.gen_is_complete = true;
        self.gen_num += 1;
    }

    pub fn created_next_gen(&mut self) {
        self.gen_is_complete = false;
    }

    pub fn gen_is_complete(&self) -> bool {
        self.gen_is_complete
    }

    pub fn save_successful(&mut self, num: u32) {
        println!("Generation {} successful: {}", self.gen_num, num);
        self.successful_num.push(num);
    }

    pub fn save_plots(&self) {
        let root_area = BitMapBackend::new(&self.plot_path, (1000, 800))
        .into_drawing_area();
        root_area.fill(&WHITE).unwrap();

        let data = &self.successful_num.iter()
            .map(|x| (*x  as f32) / (NUM_AI as f32))
            .collect::<Vec<f32>>();

        let max: f32 = data.iter().fold(0.0, |acc, x| acc.max(*x));


        let x_range = 0..data.len();
        let y_range = 0.0..max;
    
        let mut ctx = ChartBuilder::on(&root_area)
            .set_label_area_size(LabelAreaPosition::Left, 40.0)
            .set_label_area_size(LabelAreaPosition::Bottom, 40.0)
            .caption("Successful By Generation", ("sans-serif", 40.0))
            .build_cartesian_2d(x_range, y_range)
            .unwrap();
    
        ctx.configure_mesh().draw().unwrap();
    
        // create a new area series
        
        let area = AreaSeries::new(
            data.iter().enumerate().map(|(x, y)| (x, *y)),
            0.0,
            &RED.mix(0.2),
        ).border_style(&RED);
    
        ctx.draw_series(area).unwrap();

    }
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
    
    pub fn change_speed(&mut self, multiple: f32) {
        self.steps_per_second *= multiple;
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


