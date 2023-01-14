pub mod particle;
use rand;

pub mod action_grid {
    use std::borrow::Borrow;

    use rand::Rng;

    use crate::Grid::particle::Particle;

    pub const PLAYGROUND_WIDTH: u32 = 800;
    pub const PLAYGROUND_HEIGHT: u32 = 500;
    pub const CELL_SIZE : u32 = 10;
    pub const NUM_HEIGHT_CELLS: u32 = PLAYGROUND_WIDTH / CELL_SIZE;
    pub const NUM_WIDTH_CELLS: u32 = PLAYGROUND_HEIGHT / CELL_SIZE;
    pub const NUM_PARTICLES: i32 = 1;
    pub const TIMESTEP: f32 = 1.0 / 10.0;
    pub const G: f32 = 9.8;

    #[derive(Copy, Clone)]
    pub enum State {
        Paused,
        Running,
    }

    pub struct phys_system {
        particles: [Particle; NUM_PARTICLES as usize],
        velocity: [f32; (NUM_WIDTH_CELLS * NUM_HEIGHT_CELLS) as usize],
        cells: [bool; (PLAYGROUND_WIDTH * PLAYGROUND_HEIGHT) as usize],
        state: State,
    }

    impl phys_system {
        pub fn new() -> phys_system {
            let mut playground = [false; (PLAYGROUND_WIDTH * PLAYGROUND_HEIGHT) as usize];
            let mut velocity = [0.0; (NUM_WIDTH_CELLS * NUM_HEIGHT_CELLS) as usize];

            let mut particles = [Particle::new((rand::thread_rng().gen_range(0..100) as f32), (rand::thread_rng().gen_range(0..100) as f32)); NUM_PARTICLES as usize];
            for i in (0..NUM_PARTICLES) {
                particles[(i as usize)] = Particle::new(rand::thread_rng().gen_range(0..PLAYGROUND_WIDTH) as f32, rand::thread_rng().gen_range(0..PLAYGROUND_HEIGHT) as f32);
            }

            phys_system {
                particles: particles,
                velocity: velocity, 
                cells: playground,
                state: State::Paused,
            }
        }

        pub fn get(&self, x: i32, y: i32) -> Option<bool> {
            if x >= 0 && y >= 0 && (x as u32) < PLAYGROUND_WIDTH && (y as u32) < PLAYGROUND_HEIGHT {
                Some(self.cells[(x as u32 + (y as u32) * PLAYGROUND_WIDTH) as usize])
            } else {
                None
            }
        }

        pub fn getVel(&self, x: i32, y: i32) -> Option<f32> {
            if x >= 0 && x < NUM_WIDTH_CELLS as i32 && y >= 0 && y < NUM_HEIGHT_CELLS as i32{
                Some(self.velocity[(x as u32 + (y as u32) * NUM_WIDTH_CELLS) as usize])
            }
            else {
                None
            }
        }

        pub fn particles(&self) -> [Particle; (NUM_PARTICLES as usize)] {
            self.particles 
        }

        pub fn get_mut(&mut self, x: i32, y: i32) -> Option<&mut bool> {
            if x >= 0 && y >= 0 && (x as u32) < PLAYGROUND_WIDTH && (y as u32) < PLAYGROUND_HEIGHT {
                Some(&mut self.cells[(x as u32 + (y as u32) * PLAYGROUND_WIDTH) as usize])
            } else {
                None
            }
        }

        pub fn step(&mut self) {
            for p in self.particles.iter_mut() {
                p.vel.1 += TIMESTEP * G; //gravity
                p.pos.0 += TIMESTEP * p.vel.0;
                p.pos.1 += TIMESTEP * p.vel.1;
                if (p.pos.1 > PLAYGROUND_HEIGHT as f32) {
                    p.pos.1 = PLAYGROUND_HEIGHT as f32;
                    p.vel.1 *= -1.0;
                }
                if (p.pos.0 < 0.0) {
                    p.pos.0 = 0.0;
                    p.vel.0 *= -1.0;
                }

            }

        }

        pub fn toggle_state(&mut self) {
            self.state = match self.state {
                State::Paused => State::Running,
                State::Running => State::Paused,
            }
        }

        pub fn state(&self) -> State {
            self.state
        }

        pub fn update_velocity_grid(&mut self) {
            for p in self.particles {
                let cell = ((p.pos.0 / CELL_SIZE as f32) as i32, (p.pos.1 / CELL_SIZE as f32) as i32);
                println!("{}", cell.0);
                println!("{}", cell.1);
                self.velocity[(cell.0 as u32 + (cell.1 as u32) * NUM_WIDTH_CELLS) as usize] = p.vel.0;
                println!();
                
            }

        }

        pub fn update(&mut self) {
            self.step();
            self.update_velocity_grid();
            for p in self.particles {
                let x = p.pos.0;
                let y = p.pos.1;
                if (x >= 0.0 && y >= 0.0 && (x as i32) < PLAYGROUND_WIDTH as i32 && (y as i32) < PLAYGROUND_HEIGHT as i32) {
                    self.cells[(x as u32 + (y as u32) * PLAYGROUND_WIDTH) as usize] = true
                }
            }
        }
    }

    impl<'a> IntoIterator for &'a phys_system{
        type Item = &'a bool;
        type IntoIter = ::std::slice::Iter<'a, bool>;
        fn into_iter(self) -> ::std::slice::Iter<'a, bool> {
            self.cells.iter()
        }
    }
}