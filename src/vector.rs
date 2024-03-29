use std::ops::{Add, Sub, Mul, Div, Neg, AddAssign};
use std::iter::Sum;
use num::Float;
use num::Num;
use num::Signed;

use serde::{Serialize, Deserialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T
}

impl<T> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

impl<T: Float> Vec3<T> {
    pub fn mag(&self) -> T {
        (self.x.powi(2)+self.y.powi(2)+self.z.powi(2)).sqrt()
    }

    pub fn normalized(mut self) -> Self {
        let mag = self.mag();
        self.x = self.x /mag;
        self.y = self.y /mag;
        self.z = self.z /mag;

        self
    }

    pub fn dot(&self, other: &Self) -> T {
        self.x*other.x+self.y*other.y+self.z*other.z
    }

    pub fn cross(&self, other: &Self) -> Self {
        Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x
        )
    }

}

impl Vec3<f32> {
    pub fn reflect(&self, n: &Self) -> Self {
        *self - *n * self.dot(n) * 2.0
    }
}

impl<T: Num> Add for Vec3<T> {
    type Output = Vec3<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(
            self.x+rhs.x,
            self.y+rhs.y,
            self.z+rhs.z
        )
    }
}

impl<T: Num> Sub for Vec3<T> {
    type Output = Vec3<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::new(
            self.x-rhs.x,
            self.y-rhs.y,
            self.z-rhs.z
        )
    }
}

impl<T: Num> Mul for Vec3<T> {
    type Output = Vec3<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3::new(
            self.x*rhs.x,
            self.y*rhs.y,
            self.z*rhs.z
        )
    }
}

impl<T: Num> Div for Vec3<T> {
    type Output = Vec3<T>;

    fn div(self, rhs: Self) -> Self::Output {
        Vec3::new(
            self.x/rhs.x,
            self.y/rhs.y,
            self.z/rhs.z
        )
    }
}

impl<T: Num + Copy> Add<T> for Vec3<T> {
    type Output = Vec3<T>;

    fn add(self, rhs: T) -> Self::Output {
        Vec3::new(
            self.x+rhs,
            self.y+rhs,
            self.z+rhs
        )
    }
}

impl<T: Num + Copy> Sub<T> for Vec3<T> {
    type Output = Vec3<T>;

    fn sub(self, rhs: T) -> Self::Output {
        Vec3::new(
            self.x-rhs,
            self.y-rhs,
            self.z-rhs
        )
    }
}

impl<T: Num + Copy> Mul<T> for Vec3<T> {
    type Output = Vec3<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Vec3::new(
            self.x*rhs,
            self.y*rhs,
            self.z*rhs
        )
    }
}

impl<T: Num + Copy> Div<T> for Vec3<T> {
    type Output = Vec3<T>;

    fn div(self, rhs: T) -> Self::Output {
        Vec3::new(
            self.x/rhs,
            self.y/rhs,
            self.z/rhs
        )
    }
}

impl<T: Signed + Copy> Neg for Vec3<T> {
    type Output = Vec3<T>;

    fn neg(self) -> Self::Output {
        Vec3::new(
            -self.x,
            -self.y,
            -self.z
        )
    }
}

impl<T: Num + Copy> AddAssign for Vec3<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
        self.z = self.z + rhs.z;
    }
}

impl Sum for Vec3<f32> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut res = Vec3::new(0.0, 0.0, 0.0);

        for i in iter {
            res += i;
        }

        res
    }
}
