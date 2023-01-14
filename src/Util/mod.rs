pub mod Util {
    pub fn scale(min: f32, max: f32, val: f32) -> f32 {
        ((val - min) / (max - min))
    }
}