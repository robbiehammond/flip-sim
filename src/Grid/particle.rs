#[derive(Copy, Clone, Debug)]
pub struct Particle {
    pub pos: (f32, f32),
    pub vel: (f32, f32),
}

impl Particle {
    pub fn new(pos_x: f32, pos_y: f32) -> Particle { 
        let rand_pos_x = pos_x;
        let rand_pos_y = pos_y;

        Particle { pos: (rand_pos_x, rand_pos_y), vel: (0.05, 0.0)}
    }
}