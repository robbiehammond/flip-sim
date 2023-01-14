use rand::Rng;

#[derive(Copy, Clone, Debug)]
pub struct Particle {
    pub pos: (f32, f32),
    pub vel: (f32, f32),
}

impl Particle {
    pub fn new(posX: f32, posY: f32) -> Particle { 
        let rand_pos_x = posX;
        let rand_pos_y = posY;

        Particle { pos: (rand_pos_x, rand_pos_y), vel: (0.05, 0.0)}
    }
}