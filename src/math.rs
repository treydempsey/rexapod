/// Constant for PI as an `f32`
pub const PI_F32: f32 = 3.1415927410e+00;

/// Convert degrees to radians
pub fn radians(degrees: i32) -> f32 {
    (degrees as f32) * (PI_F32 / 180.0)
}

/// Linear interpolate
pub fn lerp(a: f32, b: f32, f: f32) -> f32 {
    a * (1.0 - f) + (b * f)
}

pub fn hypotenuse(x: f32, y: f32) -> f32 {
    libm::sqrtf(x * x + y * y)
}

/// This function scales the input value `x` (which lies within the input range `[in_min, in_max]`) 
/// and maps it to the corresponding value in the output range `[out_min, out_max]`. 
/// The output value maintains the same relative position within the new range as it had in the input range.
pub fn map_f32(x: f32, in_min: f32, in_max: f32, out_min: f32, out_max: f32) -> f32 {
    (x - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
}
