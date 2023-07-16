use std::ops;

use rand::{distributions::Uniform, prelude::Distribution, Rng};

#[derive(Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn default() -> Self {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { x, y, z }
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl ops::Index<i32> for Vec3 {
    type Output = f32;
    fn index(&self, index: i32) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Vec3 index greater than 2"),
        }
    }
}

impl ops::IndexMut<i32> for Vec3 {
    fn index_mut(&mut self, index: i32) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Vec3 index greater than 2"),
        }
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        *self *= 1.0 / rhs;
    }
}

impl Vec3 {
    fn length(&self) -> f32 {
        f32::sqrt(self.length_squared())
    }

    pub fn length_squared(&self) -> f32 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }
}

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

impl ops::Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f32) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f32) -> Self::Output {
        (1.0 / rhs) * self
    }
}

impl Vec3 {
    pub fn dot(&self, v2: &Vec3) -> f32 {
        self.x * v2.x + self.y * v2.y + self.z * v2.z
    }

    pub fn cross(&self, v2: &Vec3) -> Self {
        Vec3 {
            x: self.y * v2.z - self.z * v2.y,
            y: self.z * v2.x - self.x * v2.z,
            z: self.x * v2.y - self.y * v2.x,
        }
    }

    pub fn normal(self) -> Self {
        let len = self.length();
        self / len
    }

    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        return Vec3 {
            x: rng.gen_range(0.0..1.0),
            y: rng.gen_range(0.0..1.0),
            z: rng.gen_range(0.0..1.0),
        };
    }

    pub fn random_range(min: f32, max: f32) -> Self {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(min..max);
        let y = rng.gen_range(min..max);
        let z = rng.gen_range(min..max);

        return Vec3 { x, y, z };
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        f32::abs(self.x) < s && f32::abs(self.y) < s && f32::abs(self.z) < s
    }

    pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
        *v - 2.0 * v.dot(n) * *n
    }
}

pub type Point3 = Vec3;
