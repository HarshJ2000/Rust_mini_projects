// Math Operations using Generics
use std::ops::{Add, Div, Mul, Sub};

pub struct MathOps<T> {
    pub value: T,
}

impl<T> MathOps<T>
where
    T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>,
{
    pub fn new(v: T) -> Self {
        Self { value: v }
    }

    pub fn add(&mut self, rhs: T) {
        self.value = self.value + rhs;
    }

    pub fn sub(&mut self, rhs: T) {
        self.value = self.value - rhs;
    }

    pub fn mul(&mut self, rhs: T) {
        self.value = self.value * rhs;
    }

    pub fn div(&mut self, rhs: T) {
        self.value = self.value / rhs;
    }

    pub fn get(&self) -> T {
        self.value
    }
}

fn main() {
    let mut num = MathOps::new(10);

    num.add(100);
    num.sub(10);
    num.mul(2);
    num.div(100);

    println!("Result: {}", num.get());
}
