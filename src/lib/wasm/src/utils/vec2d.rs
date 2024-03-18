// 弾のベクトル(方向と速度=長さ/frame)を表す
// NOTE: https://codeknowledge.livedoor.blog/archives/12749420.html
// rad = deg * 2 * Math.PI / 360 // ラジアン
// deg = rad * 360 / 2 / Math.PI // 角度

// tan = y / x
// cos = x / rad -> x = cos * rad
// sin = y / rad -> y = sin * rad

use std::ops::{Add, Sub};

use std::f64::consts as math_const;

#[derive(Debug, Clone)]
pub struct Vec2d {
    pub x: f64,
    pub y: f64,
    pub angle: f64,
    pub speed: f64,
}

// 角度→ラジアン
pub fn deg_to_rad(deg: f64) -> f64 {
    deg * 2. * math_const::PI / 360.
}

// ラジアン→角度
pub fn rad_to_deg(rad: f64) -> f64 {
    rad * 360. / 2. / math_const::PI
}

impl Vec2d {
    pub fn new(angle: f64, speed: f64) -> Self {
        let rad = deg_to_rad(angle);
        let y = rad.sin() * speed;
        let x = rad.cos() * speed;
        Self { x, y, angle, speed }
    }
}

// impl Add for Vec2d {
//     type Output = Self;

//     fn add(self, other: Self) -> Self {
//         Self {
//             x: self.x + other.x,
//             y: self.y + other.y,
//         }
//     }
// }

// impl Sub for Vec2d {
//     type Output = Self;

//     fn sub(self, other: Self) -> Self {
//         Self {
//             x: self.x - other.x,
//             y: self.y - other.y,
//         }
//     }
// }

#[cfg(test)]
pub mod tests {
    use super::*;

    // #[test]
    // pub fn test01() {
    //     let vec2d = Vec2d::new(30.0, 5.0);

    // }
}
