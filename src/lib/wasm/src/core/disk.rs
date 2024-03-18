// static mut DISKS: [Option<Disk>; 4_096] = Default::default();

use crate::{
    core::event,
    utils::{coordinate::Coordinate, vec2d::Vec2d},
};

use super::event::{EventId, SpawnDisk, UpdateDisk};

enum ShotType {
    Disk,
    Laser,
}

enum DiskBehavior {}

// #[derive(Clone, Copy)]
#[derive(Clone, Debug)]
pub struct Disk {
    // TODO: IDをつける必要がある
    pub event_id: EventId,
    pub coord: Coordinate,
    // pub bound_rect: BoundRect, // TODO:
    pub vec2d: Vec2d,
    pub vec2d_update: Option<Vec2d>, // TODO: Vecの見える化
}

impl Disk {
    pub fn new(
        event_id: EventId,
        coord: Coordinate,
        angle: f64,
        speed: f64,
        update_angle: Option<f64>,
        update_speed: Option<f64>,
    ) -> Self {
        let vec2d = Vec2d::new(angle, speed);
        let mut vec2d_update = None;

        match (update_angle, update_speed) {
            (Some(update_angle), Some(update_speed)) => {
                vec2d_update = Some(Vec2d::new(update_angle, update_speed));
            }
            _ => (),
        }
        Self {
            event_id,
            coord,
            vec2d,
            vec2d_update,
        }
    }

    // FIXME: Behavior, Typeに応じて更新される
    // レプリケーションを容易にできるように
    fn update(&mut self) {}

    // pub fn is_out_of_bound(&self, rect: &BoundRect) {
    pub fn is_out_of_bound(&self, x_min: u32, x_max: u32, y_min: u32, y_max: u32) -> bool {
        let x = self.coord.x;
        let y = self.coord.y;
        (x <= x_min as f64 || x_max as f64 <= x) || (y <= y_min as f64 || y_max as f64 <= y)
    }
}

impl From<SpawnDisk> for Vec<Disk> {
    fn from(spawn_disk: SpawnDisk) -> Self {
        let mut buff = vec![];
        for i in 0..spawn_disk.replicate_by {
            let mut coord = spawn_disk.event.coord.clone(); // TODO: いちいちクローンするならx,yだけでもいい？

            let angle = spawn_disk.angle as f64 + (i * spawn_disk.interval_degree) as f64;
            let speed = spawn_disk.speed;

            // offset計算
            let vec2d = Vec2d::new(angle as f64, speed as f64 + spawn_disk.offset as f64);
            coord.x += vec2d.x;
            coord.y += vec2d.y;

            let disk = Disk::new(
                spawn_disk.event.event_id,
                coord,
                angle,
                speed as f64,
                spawn_disk.update_angle,
                spawn_disk.update_speed,
            ); // TODO: angleがF64である意味ある？
            buff.push(disk);
        }
        buff
    }
}

fn init_disks(capacity: u32) -> Vec<Option<Disk>> {
    let mut disks = Vec::with_capacity(capacity as usize);
    for _ in 0..capacity {
        disks.push(None);
        // disks.push(Some(Disk::new(1, 2)));
    }
    disks
}

pub struct Manager {
    disks: Vec<Option<Disk>>,
}

impl Manager {
    pub fn new() -> Self {
        Self {
            disks: init_disks(4_096),
        }
    }

    pub fn disks(&self) -> Vec<Option<&Disk>> {
        self.disks
            .iter()
            .map(|disk| disk.as_ref())
            .collect::<Vec<Option<&Disk>>>()
    }

    pub fn add_disks_from_event(&mut self, spawn_disk: SpawnDisk) {
        self.add_disks(spawn_disk.into())
    }

    // FIXME: アルゴリズム
    pub fn add_disks(&mut self, disks: Vec<Disk>) {
        let mut disk_iter = disks.into_iter();
        self.disks.iter_mut().for_each(|disk| {
            if disk.is_none() {
                if let Some(assign_target) = disk_iter.next() {
                    *disk = Some(assign_target);
                } else {
                    return;
                }
            }
        });
    }

    pub fn update_disks_from_event(&mut self, update_disk: UpdateDisk) {
        // FIXME:
        // let vec2d_update = Vec2d::new(update_disk.angle as f64, update_disk.speed as f64);
        self.disks.iter_mut().for_each(|disk| {
            // FIXME: Update Disk
            // EventID持ってないから、更新対象が特定できない
            if let Some(disk) = disk {
                if disk.event_id == update_disk.target_event_id {
                    // disk.coord.x += vec2d_update.x;
                    // disk.coord.y += vec2d_update.y;
                    // disk.vec2d = vec2d_update.clone();
                    // disk.vec2d = disk.vec2d + vec2d_update.clone();
                    // disk.vec2d.x += vec2d_update.x;
                    // disk.vec2d.y += vec2d_update.y;
                    let angle = disk.vec2d.angle + update_disk.angle as f64;
                    let speed = disk.vec2d.speed + update_disk.speed as f64;
                    let vec2d = Vec2d::new(angle, speed);
                    disk.vec2d = vec2d;
                }
            }
        });
    }

    // FIXME: 弾幕の削除 アルゴリズムの最適化
    pub fn sweep(&mut self, width: u32, height: u32) {
        // FIXME: 開いているブロックを詰める 更新処理では、ディスク数以降のブロックがNoneであればそれ以降は評価しなくてよいようにする
        self.disks.iter_mut().for_each(|outer_disk| {
            if let Some(disk) = outer_disk {
                // FIXME: 画面外に出てもいい球の場合を考慮
                // if (x <= 0. || width as f64 <= x) || (y <= 0. || height as f64 <= y) {
                if disk.is_out_of_bound(0, width, 0, height) {
                    *outer_disk = None;
                }
            }
        });
    }

    pub fn update_disks(&mut self) {
        let disks: &mut Vec<Option<Disk>> = self.disks.as_mut();
        disks.iter_mut().for_each(|disk| {
            disk.as_mut().map(|disk| {
                let mut mut_x = disk.vec2d.x;
                let mut mut_y = disk.vec2d.y;

                // NOTE: 各フレームごとの加算Vec値がある場合はそれを加算
                if let Some(vec2d_update) = disk.vec2d_update.as_ref() {
                    mut_x += vec2d_update.x;
                    mut_y += vec2d_update.y;
                }

                disk.coord.x += mut_x;
                disk.coord.y += mut_y;
            });
        })
    }
}
