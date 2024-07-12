use std::ops::{Add, Sub, Mul, Div, Neg};
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, Copy)]
struct Vector3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector3 {
    // Create a new Vector3
    fn new(x: f64, y: f64, z: f64) -> Self {
        Vector3 { x, y, z }
    }

    // Compute the magnitude of the vector
    fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    // Normalize the vector
    fn normalize(&self) -> Self {
        let mag = self.magnitude();
        Vector3 {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
        }
    }

    // Dot product of two vectors
    fn dot(&self, other: Vector3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    // Cross product of two vectors
    fn cross(&self, other: Vector3) -> Self {
        Vector3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

// Implementing the Add trait for Vector3
impl Add for Vector3 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

// Implementing the Sub trait for Vector3
impl Sub for Vector3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

// Implementing the Mul trait for scalar multiplication
impl Mul<f64> for Vector3 {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self::Output {
        Vector3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

// Implementing the Div trait for scalar division
impl Div<f64> for Vector3 {
    type Output = Self;

    fn div(self, scalar: f64) -> Self::Output {
        Vector3 {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

// Implementing the Neg trait for negating a vector
impl Neg for Vector3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vector3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

// Implementing the Display trait for Vector3
impl Display for Vector3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

fn main() {
    let v1 = Vector3::new(1.0, 2.0, 3.0);
    let v2 = Vector3::new(4.0, 5.0, 6.0);

    // Vector addition
    let v3 = v1 + v2;
    println!("{} + {} = {}", v1, v2, v3);

    // Vector subtraction
    let v4 = v1 - v2;
    println!("{} - {} = {}", v1, v2, v4);

    // Scalar multiplication
    let v5 = v1 * 2.0;
    println!("{} * 2.0 = {}", v1, v5);

    // Scalar division
    let v6 = v1 / 2.0;
    println!("{} / 2.0 = {}", v1, v6);

    // Vector negation
    let v7 = -v1;
    println!("-{} = {}", v1, v7);

    // Magnitude
    let mag = v1.magnitude();
    println!("|{}| = {}", v1, mag);

    // Normalization
    let norm = v1.normalize();
    println!("Normalized {} = {}", v1, norm);

    // Dot product
    let dot_product = v1.dot(v2);
    println!("{} . {} = {}", v1, v2, dot_product);

    // Cross product
    let cross_product = v1.cross(v2);
    println!("{} x {} = {}", v1, v2, cross_product
