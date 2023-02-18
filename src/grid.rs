pub mod particle;

pub mod action_grid {
    use rand::Rng;

    use crate::{grid::particle::Particle, Util::util::mag};

    pub const CELL_SIZE : u32 = 60;
    //Leave PLAYGROUND_WIDTH and PLAYGROUND_HEIGHT as multiples of CELL_SIZE ensure correct behavior 
    pub const PLAYGROUND_WIDTH: u32 = 20 * CELL_SIZE;
    pub const PLAYGROUND_HEIGHT: u32 = 10 * CELL_SIZE;

    pub const NUM_HEIGHT_CELLS: u32 = PLAYGROUND_HEIGHT / CELL_SIZE;
    pub const NUM_WIDTH_CELLS: u32 = PLAYGROUND_WIDTH / CELL_SIZE;

    pub const G_TO_P_PROPORTION: f32 = 0.005;
    pub const P_TO_G_PROPORTION: f32 = 0.9;
    pub const CLIPPING_SPEED: f32 = 20.0;

    pub const NUM_PARTICLES: u32 = 5;

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
        velocity: [(f32, f32); (NUM_WIDTH_CELLS * NUM_HEIGHT_CELLS) as usize],
        cells: [bool; (PLAYGROUND_WIDTH * PLAYGROUND_HEIGHT) as usize],
        state: State,
    }

    impl PhysSystem {
        pub fn new() -> PhysSystem {
            let playground = [false; (PLAYGROUND_WIDTH * PLAYGROUND_HEIGHT) as usize];
            let velocity = [(0.0, 0.0); (NUM_WIDTH_CELLS * NUM_HEIGHT_CELLS) as usize];

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

        pub fn get_vel(&self, x: u32, y: u32) -> Option<(f32, f32)> {
            if x < NUM_WIDTH_CELLS && y < NUM_HEIGHT_CELLS {
                Some(self.velocity[(x as u32 + (y as u32) * NUM_WIDTH_CELLS) as usize])
            }
            else {
                Some((0.0, 0.0))
            }
        }

        pub fn set_vel(&mut self, x: u32, y: u32, val: (f32, f32)) {
            if x < NUM_WIDTH_CELLS && y < NUM_HEIGHT_CELLS {
                self.velocity[(x as u32 + (y as u32) * NUM_WIDTH_CELLS) as usize] = val;
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
                if p.pos.1 < 0.0 {
                    p.pos.1 = 0.0;
                    p.vel.1 *= -1.0 * DAMPING_COEF;
                }
                println!("{}", p.vel.1)
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

        pub fn particles_to_grid(&mut self) -> [(f32, f32); (NUM_WIDTH_CELLS * NUM_HEIGHT_CELLS) as usize] {
            let old_velocities = self.velocity.clone();
            let mut r = [0.0; (NUM_WIDTH_CELLS * NUM_HEIGHT_CELLS) as usize];
            for i in 0..NUM_WIDTH_CELLS {
                for j in 0..NUM_HEIGHT_CELLS {
                   self.velocity[((i as u32) + (j as u32) * NUM_WIDTH_CELLS) as usize] = (0.0, 0.0);
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

                if Self::exists_in_vel_grid(top_left) {
                    let val: (f32, f32) = self.velocity[((top_left.0) + (top_left.1) * NUM_WIDTH_CELLS) as usize];
                    self.velocity[((top_left.0) + (top_left.1) * NUM_WIDTH_CELLS) as usize] = (val.0 + w1 * p.vel.0, val.1 + w1 * p.vel.1);
                    r[((top_left.0) + (top_left.1) * NUM_WIDTH_CELLS) as usize] += 10.;
                }
                if Self::exists_in_vel_grid(top_right) {
                    let val: (f32, f32) = self.velocity[((top_right.0) + (top_right.1) * NUM_WIDTH_CELLS) as usize];
                    self.velocity[((top_right.0) + (top_right.1) * NUM_WIDTH_CELLS) as usize] = (val.0 + w2 * p.vel.0, val.1 + w2 * p.vel.1);
                    r[((top_right.0) + (top_right.1) * NUM_WIDTH_CELLS) as usize] += 10.;
                }
                if Self::exists_in_vel_grid(bottom_right) {
                    let val: (f32, f32) = self.velocity[((bottom_right.0) + (bottom_right.1) * NUM_WIDTH_CELLS) as usize];
                    self.velocity[((bottom_right.0) + (bottom_right.1) * NUM_WIDTH_CELLS) as usize] = (val.0 + w3 * p.vel.0, val.1 + w3 * p.vel.1);
                    r[((bottom_right.0) + (bottom_right.1) * NUM_WIDTH_CELLS) as usize] += 10.;
                }
                if Self::exists_in_vel_grid(bottom_left) {
                    let val: (f32, f32) = self.velocity[((bottom_left.0) + (bottom_left.1) * NUM_WIDTH_CELLS) as usize];
                    self.velocity[((bottom_left.0) + (bottom_left.1) * NUM_WIDTH_CELLS) as usize] = (val.0 + w4 * p.vel.0, val.1 + w4 * p.vel.1);
                    r[((bottom_left.0) + (bottom_left.1) * NUM_WIDTH_CELLS) as usize] += 10.;
                }
            }
            for i in 0..NUM_WIDTH_CELLS {
                for j in 0..NUM_HEIGHT_CELLS {
                    let vel_X = self.get_vel(i, j).unwrap().0;
                    let vel_Y = self.get_vel(i, j).unwrap().1;
                    let x_updated = vel_X / 10.0;
                    let y_updated = vel_Y / 10.0;
                    self.set_vel(i, j, (x_updated, y_updated));
                }
            }
            old_velocities
        }

        fn exists_in_vel_grid(p: (u32, u32)) -> bool {
            p.0 < NUM_WIDTH_CELLS && p.1 < NUM_HEIGHT_CELLS
        }

        pub fn grid_to_particles(&mut self, old_grid: [(f32, f32); (NUM_WIDTH_CELLS * NUM_HEIGHT_CELLS) as usize]) {
            let mut delta_vel: [(f32, f32); (NUM_WIDTH_CELLS * NUM_HEIGHT_CELLS) as usize] = old_grid;
            for i in 0..NUM_WIDTH_CELLS {
                for j in 0..NUM_HEIGHT_CELLS {
                    delta_vel[(i + j * NUM_WIDTH_CELLS) as usize].0 = self.velocity[(i + j * NUM_WIDTH_CELLS) as usize].0 - old_grid[(i + j * NUM_WIDTH_CELLS) as usize].0;
                    delta_vel[(i + j * NUM_WIDTH_CELLS) as usize].1 = self.velocity[(i + j * NUM_WIDTH_CELLS) as usize].1 - old_grid[(i + j * NUM_WIDTH_CELLS) as usize].1;
                }
            }
            for p in self.particles.iter_mut() {
                let top_left = ((p.pos.0 as u32 / CELL_SIZE) , (p.pos.1 as u32 / CELL_SIZE));
                let top_right = ((p.pos.0 as u32 / CELL_SIZE) + 1, (p.pos.1 as u32 / CELL_SIZE));
                let bottom_left = ((p.pos.0 as u32 / CELL_SIZE), (p.pos.1 as u32 / CELL_SIZE) + 1);
                let bottom_right = ((p.pos.0 as u32 / CELL_SIZE) + 1, (p.pos.1 as u32 / CELL_SIZE) + 1);

                let dx = p.pos.0 - top_left.0 as f32 * CELL_SIZE as f32;
                let dy = p.pos.1 - top_left.1 as f32 * CELL_SIZE as f32;

                let mut w1 = (1.0 - dx / CELL_SIZE as f32) * (1.0 - dy / CELL_SIZE as f32); //top left (in theory)
                let mut w2 = (dx / CELL_SIZE as f32) * (1.0 - dy / CELL_SIZE as f32); //top right (in theory)
                let mut w3 = (dx / CELL_SIZE as f32) * (dy / CELL_SIZE as f32); //bottom_right (in theory)
                let mut w4 = (1.0 - dx / CELL_SIZE as f32) * (dy / CELL_SIZE as f32); //bottom_left (in theory)



                //might need to comment this out
                /* 
                w1 *= G_TO_P_PROPORTION;
                w2 *= G_TO_P_PROPORTION;
                w3 *= G_TO_P_PROPORTION;
                w4 += G_TO_P_PROPORTION;
                */

                let mut numerator_x: f32 = 0.0;
                let mut numerator_y: f32 = 0.0;
                let mut denominator: f32 = 0.0;

                if Self::exists_in_vel_grid(top_left) { 
                    let grid_vel = delta_vel[((top_left.0) + (top_left.1) * NUM_WIDTH_CELLS) as usize];
                    numerator_x += w1 * grid_vel.0;
                    numerator_y += w1 * grid_vel.1;
                    denominator += w1;
                }
                if Self::exists_in_vel_grid(top_right) { 
                    let grid_vel = delta_vel[((top_right.0) + (top_right.1) * NUM_WIDTH_CELLS) as usize];
                    numerator_x += w2 * grid_vel.0;
                    numerator_y += w2 * grid_vel.1;
                    denominator += w2;
                }
                if Self::exists_in_vel_grid(bottom_right) { 
                    let grid_vel = delta_vel[((bottom_right.0) + (bottom_right.1) * NUM_WIDTH_CELLS) as usize];
                    numerator_x += w3 * grid_vel.0;
                    numerator_y += w3 * grid_vel.1;
                    denominator += w3;
                }
                if Self::exists_in_vel_grid(bottom_left) { 
                    let grid_vel = delta_vel[((bottom_left.0) + (bottom_left.1) * NUM_WIDTH_CELLS) as usize];
                    numerator_x += w4 * grid_vel.0;
                    numerator_y += w4 * grid_vel.1;
                    denominator += w4;
                }
                p.vel.0 +=  numerator_x / denominator;
                p.vel.1 += numerator_y / denominator;

            }
        }

        fn measure_divergence(x: u32, y: u32) -> Option<f32> {
            None
        }

        pub fn enforce_incompressibility(&mut self, old_grid: [(f32, f32); (NUM_WIDTH_CELLS * NUM_HEIGHT_CELLS) as usize]) {

            
        }

        pub fn update(&mut self) {
            if self.state == State::Paused { return }
            let old_grid = self.particles_to_grid();
            self.enforce_incompressibility(old_grid);
            self.grid_to_particles(old_grid);
            self.step(); 
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
