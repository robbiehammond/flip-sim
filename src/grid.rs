pub mod particle;

pub mod action_grid {
    use rand::Rng;

    use crate::{grid::particle::Particle, util::util::mag};

    pub const CELL_SIZE : u32 = 50;
    //Leave PLAYGROUND_WIDTH and PLAYGROUND_HEIGHT as multiples of CELL_SIZE ensure correct behavior 
    pub const PLAYGROUND_WIDTH: u32 = 20 * CELL_SIZE;
    pub const PLAYGROUND_HEIGHT: u32 = 10 * CELL_SIZE;

    pub const NUM_HEIGHT_CELLS: u32 = PLAYGROUND_HEIGHT / CELL_SIZE;
    pub const NUM_WIDTH_CELLS: u32 = PLAYGROUND_WIDTH / CELL_SIZE;

    pub const NUM_PARTICLES: u32 = 20;

    pub const TIMESTEP: f32 = 1.0 / 10.0;
    pub const G: f32 = 9.8;
    pub const DAMPING_COEF: f32 = 1.0; //1 means no damping 

    #[derive(Copy, Clone, PartialEq)]
    pub enum State {
        Paused,
        Running,
    }

    pub struct PhysSystem {
        particles: [Particle; NUM_PARTICLES as usize],
        velocity: [f32; (NUM_WIDTH_CELLS * NUM_HEIGHT_CELLS) as usize],
        cells: [bool; (PLAYGROUND_WIDTH * PLAYGROUND_HEIGHT) as usize],
        state: State,
    }

    impl PhysSystem {
        pub fn new() -> PhysSystem {
            let playground = [false; (PLAYGROUND_WIDTH * PLAYGROUND_HEIGHT) as usize];
            let velocity = [0.0; (NUM_WIDTH_CELLS * NUM_HEIGHT_CELLS) as usize];

            let mut particles = [Particle::new(rand::thread_rng().gen_range(0..100) as f32, rand::thread_rng().gen_range(0..100) as f32); NUM_PARTICLES as usize];
            for i in 0..NUM_PARTICLES {
                particles[(i as usize)] = Particle::new(rand::thread_rng().gen_range(0..PLAYGROUND_WIDTH) as f32, rand::thread_rng().gen_range(0..PLAYGROUND_HEIGHT) as f32);
                particles[(i as usize)].vel.0 = rand::thread_rng().gen_range(-100..100) as f32;
            }

            PhysSystem {
                particles: particles,
                velocity: velocity, 
                cells: playground,
                state: State::Running,
            }
        }

        pub fn get(&self, x: i32, y: i32) -> Option<bool> {
            if x >= 0 && y >= 0 && (x as u32) < PLAYGROUND_WIDTH && (y as u32) < PLAYGROUND_HEIGHT {
                Some(self.cells[(x as u32 + (y as u32) * PLAYGROUND_WIDTH) as usize])
            } else {
                None
            }
        }

        pub fn get_vel(&self, x: u32, y: u32) -> Option<f32> {
            if x < NUM_WIDTH_CELLS && y < NUM_HEIGHT_CELLS {
                Some(self.velocity[(x as u32 + (y as u32) * NUM_WIDTH_CELLS) as usize])
            }
            else {
                Some(0.0)
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
                if p.pos.0 >= PLAYGROUND_WIDTH as f32 {
                    p.pos.0 = (PLAYGROUND_WIDTH - 1) as f32;
                    p.vel.0 *= -1.0 * DAMPING_COEF;
                }
                if p.pos.0 < 0.0 {
                    p.pos.0 = 0.0;
                    p.vel.0 *= -1.0 * DAMPING_COEF;

                }
                if p.pos.1 >= PLAYGROUND_HEIGHT as f32 {
                    p.pos.1 = (PLAYGROUND_HEIGHT - 1) as f32;
                    p.vel.1 *= -1.0 * DAMPING_COEF;
                }
                if p.pos.0 < 0.0 {
                    p.pos.0 = 0.0;
                    p.vel.0 *= -1.0 * DAMPING_COEF;
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

        pub fn particles_to_grid(&mut self) {
            for i in 0..NUM_WIDTH_CELLS {
                for j in 0..NUM_HEIGHT_CELLS {
                   self.velocity[((i as u32) + (j as u32) * NUM_WIDTH_CELLS) as usize] = 0.0
                }
            }
            for p in self.particles {
                let top_left = ((p.pos.0 as u32 / CELL_SIZE) , (p.pos.1 as u32 / CELL_SIZE));
                let top_right = ((p.pos.0 as u32 / CELL_SIZE) + 1, (p.pos.1 as u32 / CELL_SIZE));
                let bottom_left = ((p.pos.0 as u32 / CELL_SIZE), (p.pos.1 as u32 / CELL_SIZE) + 1);
                let bottom_right = ((p.pos.0 as u32 / CELL_SIZE) + 1, (p.pos.1 as u32 / CELL_SIZE) + 1);

                let dx = p.pos.0 - top_left.0 as f32 * CELL_SIZE as f32;
                let dy = p.pos.1 - top_left.1 as f32 * CELL_SIZE as f32;
                //println!("{dx}, {dy}, {l}, {r}");
                let w1 = (1.0 - dx / CELL_SIZE as f32) * (1.0 - dy / CELL_SIZE as f32); //top left (in theory)
                let w2 = (dx / CELL_SIZE as f32) * (1.0 - dy / CELL_SIZE as f32); //top right (in theory)
                let w3 = (dx / CELL_SIZE as f32) * (dy / CELL_SIZE as f32); //bottom_left (in theory)
                let w4 = (1.0 - dx / CELL_SIZE as f32) * (dy / CELL_SIZE as f32); //bottom_right (in theory)

                if self.exists_in_vel_grid(top_left) {
                    self.velocity[((top_left.0) + (top_left.1) * NUM_WIDTH_CELLS) as usize] += w1 * mag(p.vel.0, p.vel.1);
                }
                if self.exists_in_vel_grid(top_right) {
                    self.velocity[((top_right.0) + (top_right.1) * NUM_WIDTH_CELLS) as usize] += w2 * mag(p.vel.0, p.vel.1);
                }
                if self.exists_in_vel_grid(bottom_right) {
                    self.velocity[((bottom_right.0) + (bottom_right.1) * NUM_WIDTH_CELLS) as usize] += w3 * mag(p.vel.0, p.vel.1);
                }
                if self.exists_in_vel_grid(bottom_left) {
                    self.velocity[((bottom_left.0) + (bottom_left.1) * NUM_WIDTH_CELLS) as usize] += w4 * mag(p.vel.0, p.vel.1);
                }
            }
        }

        fn exists_in_vel_grid(&self, p: (u32, u32)) -> bool {
            p.0 < NUM_WIDTH_CELLS && p.1 < NUM_HEIGHT_CELLS
        }

        pub fn update_velocity_grid(&mut self) {
            for i in 0..NUM_WIDTH_CELLS {
                for j in 0..NUM_HEIGHT_CELLS {
                   self.velocity[((i as u32) + (j as u32) * NUM_WIDTH_CELLS) as usize] = 0.0
                }
            }

            for p in self.particles {
                let cell = ((p.pos.0 as u32 / CELL_SIZE) , (p.pos.1 as u32 / CELL_SIZE));
                self.velocity[(cell.0 + cell.1 * NUM_WIDTH_CELLS) as usize] = mag(p.vel.0, p.vel.1)
            }

        }

        pub fn update(&mut self) {
            if self.state == State::Paused { return }
            self.step(); 
            self.particles_to_grid();
            for p in self.particles {
                let x = p.pos.0;
                let y = p.pos.1;
                if x >= 0.0 && y >= 0.0 && (x as i32) < PLAYGROUND_WIDTH as i32 && (y as i32) < PLAYGROUND_HEIGHT as i32 {
                    self.cells[(x as u32 + (y as u32) * PLAYGROUND_WIDTH) as usize] = true
                }
            }
        }
    }

    impl<'a> IntoIterator for &'a PhysSystem{
        type Item = &'a bool;
        type IntoIter = ::std::slice::Iter<'a, bool>;
        fn into_iter(self) -> ::std::slice::Iter<'a, bool> {
            self.cells.iter()
        }
    }
}