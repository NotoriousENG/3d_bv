use bevy::prelude::Vec3;

/// Moves from toward to by the delta value and returns a new vector
pub fn move_toward(from: Vec3, to: Vec3, delta: f32) -> Vec3 {
    let mut result = to - from;
    let length = result.length();
    if length <= delta || length == 0.0 {
        return to;
    }
    result *= delta / length;
    result += from;
    result
}

// same as move toward but for f32
pub fn move_toward_f32(from: f32, to: f32, delta: f32) -> f32 {
    let mut result = to - from;
    let length = result.abs();
    if length <= delta || length == 0.0 {
        return to;
    }
    result *= delta / length;
    result += from;
    result
}

/// convert from degrees to radians
pub fn deg_to_rad(degrees: f32) -> f32 {
    degrees * std::f32::consts::PI / 180.0
}
