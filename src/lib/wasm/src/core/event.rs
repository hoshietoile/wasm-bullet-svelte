// use crate::core::disk::Disk;

use crate::utils::coordinate::Coordinate;

pub type EventId = u32;

#[derive(Clone)]
pub struct SpawnDisk {
    pub angle: u16,
    pub speed: u32,
    pub replicate_by: u32,
    pub interval_degree: u32,
    pub update_angle: Option<f64>,
    pub update_speed: Option<f64>,
    // pub spawn_interval: u32,
    // pub degree_change_per: u32,
    pub offset: u32,
    pub event: EventSetting,
}

impl SpawnDisk {
    pub fn new(
        angle: u16,
        speed: u32,
        replicate_by: u32,
        interval_degree: u32,
        offset: u32,
        update_angle: Option<f64>,
        update_speed: Option<f64>,
        event: EventSetting,
    ) -> Self {
        Self {
            angle,
            speed,
            replicate_by,
            interval_degree,
            // spawn_interval, // NOTE: NextEvent? 次のEventは角度と射出インターバルを反映した設定にする
            // degree_change_per,
            update_angle,
            update_speed,
            offset,
            event,
        }
    }
}

#[derive(Clone)]
pub struct UpdateDisk {
    pub target_event_id: EventId,
    pub angle: u16,
    pub speed: u32,
    // pub replicate_by: u32,
    // pub interval_degree: u32,
    // event: EventSetting,
}

impl UpdateDisk {
    pub fn new(target_event_id: EventId, angle: u16, speed: u32) -> Self {
        Self {
            target_event_id,
            angle,
            speed,
        }
    }
}

// Eventは
// 弾の射出
// 敵の登場
// 弾の挙動 更新
// 等様々なイベントをすべて統括する
#[derive(Clone)]
pub enum EventMeta {
    Nil,
    SpawnDisk(SpawnDisk),
    UpdateDisk(UpdateDisk),
}

pub trait Emittable {
    // https://doc.rust-jp.rs/rust-by-example-ja/generics/assoc_items/types.html
    type EmitValue;

    // FIXME: ActorからEventID指定で実行する場合？
    fn emit(&mut self) -> Option<Self::EmitValue>;
}

impl Emittable for EventMeta {
    type EmitValue = EventMeta;

    fn emit(&mut self) -> Option<EventMeta> {
        match self {
            EventMeta::Nil => None,
            EventMeta::SpawnDisk(disk) => Some(EventMeta::SpawnDisk(disk.clone())),
            EventMeta::UpdateDisk(_) => None,
        }
    }
}

// EventGroup?

#[derive(Clone)]
pub struct EventSetting {
    // pub x: u32,
    // pub y: u32,
    pub coord: Coordinate,
    pub event_id: EventId,
    // pub // event_type: EventType,
    pub start_at: u32,
    pub end_at: u32,
    // pub replicate_by: u32,
    // next_event_id: EventId,
    // prev_event_id: EventId,
    // start_event_id:
    // end_event_id:
}

impl EventSetting {
    pub fn new(event_id: EventId, coord: Coordinate, start_at: u32, end_at: u32) -> Self {
        Self {
            event_id,
            coord,
            // event_type: EventType::Nil,
            start_at,
            end_at,
            // replicate_by,
            // next_event_id: "NextEvent".into(),
            // prev_event_id: "PrevEvent".into(),
        }
    }

    pub fn emit(&self) {}
}
