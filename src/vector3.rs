use core::{
    fmt::Display,
    ops::{Add, Mul},
};

use crate::{
    math::{lerp, radians},
    vector2::Vector2,
};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
/// Data structure for holding 3D vectors
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    /// Creates a new `Vector3` from a triple of `f32`
    pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 { x, y, z }
    }

    /// Rotate to `angle_deg` about a 2D `pivot` point
    pub fn rotate(&self, angle_deg: i32, pivot: Vector2) -> Vector3 {
        // Translate line so pivot point is at the origin
        if angle_deg == 0 {
            return self.clone();
        }

        let x = self.x - pivot.x;
        let y = self.y - pivot.y;
        let angle_rad = radians(angle_deg);

        // Rotate point by angle
        let x_rotated = x * libm::cosf(angle_rad) - y * libm::sinf(angle_rad);
        let y_rotated = x * libm::sinf(angle_rad) + y * libm::cosf(angle_rad);

        // Translate point back to original position
        let x = x_rotated + pivot.x;
        let y = y_rotated + pivot.y;

        Vector3 { x, y, z: self.z }
    }

    pub fn distance_to(&self, v: &Vector3) -> f32 {
        let dx = v.x - self.x;
        let dy = v.y - self.y;
        let dz = v.z - self.z;
        libm::sqrtf(dx * dx + dy * dy + dz * dz)
    }

    /// Linear interpolate from one 3D vector to another by amount `f`
    pub fn lerp(&self, b: Vector3, f: f32) -> Vector3 {
        Vector3 {
            x: lerp(self.x, b.x, f),
            y: lerp(self.y, b.y, f),
            z: lerp(self.z, b.z, f),
        }
    }
}

/// Vector3 addition
impl Add for Vector3 {
    type Output = Vector3;

    fn add(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Add for &Vector3 {
    type Output = Vector3;

    fn add(self, other: &Vector3) -> Vector3 {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

/// Scalar multiplication
impl Mul<f32> for Vector3 {
    type Output = Vector3;

    fn mul(self, scalar: f32) -> Vector3 {
        Vector3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl Mul<f32> for &Vector3 {
    type Output = Vector3;

    fn mul(self, scalar: f32) -> Vector3 {
        Vector3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

/// Hadamard product or element-wise product
impl Mul<Vector3> for Vector3 {
    type Output = Vector3;

    fn mul(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Mul<&Vector3> for &Vector3 {
    type Output = Vector3;

    fn mul(self, other: &Vector3) -> Vector3 {
        Vector3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Display for Vector3 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!("({},{},{})", self.x, self.y, self.z))
    }
}
