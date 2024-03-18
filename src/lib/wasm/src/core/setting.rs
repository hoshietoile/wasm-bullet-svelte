use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Manager {
    pub canvas_id: String,
    pub width: u32,
    pub height: u32,

    // FIXME: ForDebug
    pub speed: u32,
    pub degree: u32,
    pub interval_degree: u32,
    pub disk_num: u32,
    pub spawn_interval: u32,
    pub degree_change_per: u32,
    pub offset: u32,
    pub update_angle: f64,
    pub update_speed: f64,
    pub change_angle: f64,
    pub change_speed: f64,
}
