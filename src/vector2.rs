use core::{
    fmt::Display,
    ops::{Add, Mul},
};

use crate::math::{lerp, radians};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
/// Data structure for holding 2D vectors
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    /// Creates a new `Vector2` from a pair of `f32`
    pub fn new(x: f32, y: f32) -> Vector2 {
        Vector2 { x, y }
    }

    /// Calculate the magnitude of the vector
    pub fn magnitude(&self) -> f32 {
        libm::sqrtf(self.x * self.x + self.y * self.y)
    }

    /// Normalize the vector to a magnitude of 1.0
    pub fn normalize(&mut self) {
        let mag = self.magnitude();
        if mag > 0.0 {
            self.x /= mag;
            self.y /= mag;
        }
    }

    /// Rotate to `angle_deg` about a 2D `pivot` point
    pub fn rotate(&self, angle_deg: i32, pivot: Vector2) -> Vector2 {
        if angle_deg == 0 {
            return self.clone();
        }

        // Translate line so pivot point is at the origin
        let x = self.x - pivot.x;
        let y = self.y - pivot.y;
        // XXX The hexapod source doesn't convert to radians, is this an oversight?
        let angle_rad = radians(angle_deg);

        // Rotate point by angle
        let x_rotated = x * libm::cosf(angle_rad) - y * libm::sinf(angle_rad);
        let y_rotated = x * libm::sinf(angle_rad) + y * libm::cosf(angle_rad);

        // Translate point back to original position
        let x = x_rotated + pivot.x;
        let y = y_rotated + pivot.y;

        Vector2 { x, y }
    }

    /// Linear interpolate from one 2D vector to another by amount `f`
    pub fn lerp(&self, v: Vector2, f: f32) -> Vector2 {
        Vector2 {
            x: lerp(self.x, v.x, f),
            y: lerp(self.y, v.y, f),
        }
    }
}

/// Vector2 addition
impl Add for Vector2 {
    type Output = Vector2;

    fn add(self, other: Vector2) -> Vector2 {
        Vector2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Add for &Vector2 {
    type Output = Vector2;

    fn add(self, other: &Vector2) -> Vector2 {
        Vector2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

/// Scalar multiplication
impl Mul<f32> for Vector2 {
    type Output = Vector2;

    fn mul(self, scalar: f32) -> Vector2 {
        Vector2 {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl Mul<f32> for &Vector2 {
    type Output = Vector2;

    fn mul(self, scalar: f32) -> Vector2 {
        Vector2 {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

/// Hadamard product or element-wise product
impl Mul<Vector2> for Vector2 {
    type Output = Vector2;

    fn mul(self, other: Vector2) -> Vector2 {
        Vector2 {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl Mul<&Vector2> for &Vector2 {
    type Output = Vector2;

    fn mul(self, other: &Vector2) -> Vector2 {
        Vector2 {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl Display for Vector2 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!("({},{})", self.x, self.y))
    }
}
