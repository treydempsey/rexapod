use crate::{vector2::Vector2, vector3::Vector3};

/// Finds a 2D point on a bezier curve given control points and a `t` between `0` and `1`.
pub fn point_on_2d_curve(control_points: &[Vector2], t: f32) -> Vector2 {
    // Number of control points minus one
    let n = control_points.len() as i32 - 1;
    let mut result = control_points[0].clone();

    // De Casteljau's algorithm
    for i in 0..=n {
        let mut temp = control_points[i as usize].clone();
        for j in (i + 1..=n).rev() {
            // Interpolate between points
            temp = temp * (1.0 - t) + &control_points[j as usize] * t;
        }
        result = result + temp * binomial_coefficient(n, i) as f32 * libm::powf(t, i as f32);
    }

    result
}

/// Finds a 3D point on a bezier curve given control points and a `t` between `0` and `1`.
pub fn point_on_3d_curve(control_points: &[Vector3], t: f32) -> Vector3 {
    // Number of control points minus one
    let n = control_points.len() as i32 - 1;
    let mut result = control_points[0].clone();

    // De Casteljau's algorithm
    for i in 0..=n {
        let mut temp = control_points[i as usize].clone();
        for j in (i + 1..=n).rev() {
            // Interpolate between points
            temp = temp * (1.0 - t) + &control_points[j as usize] * t;
        }
        result = result + temp * binomial_coefficient(n, i) as f32 * libm::powf(t, i as f32);
    }

    result
}

fn binomial_coefficient(n: i32, k: i32) -> i32 {
    // Handle edge cases where k is 0 or n == k
    if k == 0 || k == n {
        return 1;
    }

    // Choose the smaller of k and n - k to minimize the number of iterations
    let k = if k > n - k { n - k } else { k };

    let mut result = 1;

    // Calculate the binomial coefficient using the formula:
    // (n * (n-1) * (n-2) ... * (n-k+1)) / (k * (k-1) * (k-2) ... * 1)
    for i in 1..=k {
        result *= n - (k - i);
        result /= i;
    }

    result
}
