use std::ops::Neg;

use auto_ops::*;

#[derive(Debug, Clone, Copy)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Default for Vec2 {
    fn default() -> Self {
        Self::zero()
    }
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self{x, y}
    }

    pub fn zero() -> Self {
        Self{x: 0.0, y: 0.0}
    }

    pub fn one() -> Self {
        Self{x: 1.0, y: 1.0}
    }

    pub fn sqr_magnitude(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    pub fn magnitude(&self) -> f32 {
        self.sqr_magnitude().sqrt()
    }

    pub fn normalize(&mut self) -> &mut Self {
        let m = self.magnitude();
        self.x /= m;
        self.y /= m;
        self
    }
    pub fn normalized(&self) -> Self {
        *self.clone().normalize()
    }

    pub fn dot(&self, b: &Vec2) -> f32 {
        self.x * b.x + self.y * b.y
    }
}

impl_op_ex!(+= |a: &mut Vec2, b: &Vec2| { a.x += b.x; a.y += b.y; });
impl_op_ex!(-= |a: &mut Vec2, b: &Vec2| { a.x -= b.x; a.y -= b.y; });
impl_op_ex!(*= |a: &mut Vec2, b: &Vec2| { a.x *= b.x; a.y *= b.y; });
impl_op_ex!(/= |a: &mut Vec2, b: &Vec2| { a.x /= b.x; a.y /= b.y; });

impl_op_ex!(*= |a: &mut Vec2, b: &f32| { a.x *= b; a.y *= b });
impl_op_ex!(/= |a: &mut Vec2, b: &f32| { a.x /= b; a.y /= b });

impl_op_ex!(+ |a: &Vec2, b: &Vec2| -> Vec2 { Vec2{x: a.x + b.x, y: a.y + b.y} });
impl_op_ex!(- |a: &Vec2, b: &Vec2| -> Vec2 { Vec2{x: a.x - b.x, y: a.y - b.y} });
impl_op_ex!(* |a: &Vec2, b: &Vec2| -> Vec2 { Vec2{x: a.x * b.x, y: a.y * b.y} });
impl_op_ex!(/ |a: &Vec2, b: &Vec2| -> Vec2 { Vec2{x: a.x / b.x, y: a.y / b.y} });

impl_op_ex_commutative!(* |a: &Vec2, b: &f32| -> Vec2 { Vec2{x: a.x * b, y: a.y * b} });
impl_op_ex_commutative!(/ |a: &Vec2, b: &f32| -> Vec2 { Vec2{x: a.x / b, y: a.y / b} });

impl Neg for Vec2 {
    type Output = Vec2;
    fn neg(self) -> Self::Output {
        Vec2{x: -self.x, y: -self.y}
    }
}
