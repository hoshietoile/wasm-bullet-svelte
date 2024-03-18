//  矩形構造体

#[derive(Debug, Clone)]
pub struct BoundRect {
    x_min: u32,
    x_max: u32,
    y_min: u32,
    y_max: u32,
}

impl BoundRect {
    pub fn new(x_min: u32, x_max: u32, y_min: u32, y_max: u32) -> Self {
        Self {
            x_min,
            x_max,
            y_min,
            y_max,
        }
    }

    // TODO: 衝突判定

    // TODO: 矩形内判定
}

// pub fn is_out_of_bound(&self, x_min: u32, x_max: u32, y_min: u32, y_max: u32) -> bool {
//         let x = self.coord.x;
//         let y = self.coord.y;
//         (x <= x_min as f64 || x_max as f64 <= x) || (y <= y_min as f64 || y_max as f64 <= y)
//     }
