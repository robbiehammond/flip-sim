pub mod Util {

    pub fn scale(min: f32, max: f32, val: f32) -> f32 {
        ((val - min) / (max - min))
    }

    pub fn mag(dX: f32, dY: f32) -> f32 {
        (dX * dX + dY * dY).sqrt()
    }
}