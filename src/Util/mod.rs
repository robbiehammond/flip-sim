pub mod util {

    pub fn scale(min: f32, max: f32, val: f32) -> f32 {
        (val - min) / (max - min)
    }

    pub fn mag(dx: f32, dy: f32) -> f32 {
        (dx * dx + dy * dy).sqrt()
    }
}