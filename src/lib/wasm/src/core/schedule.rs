use std::collections::HashMap;

use crate::core::disk::Disk;
use crate::core::event::{Emittable, EventId, EventMeta, EventSetting};

use super::event::SpawnDisk;

pub struct Manager {
    pub event_id: EventId,
    pub time: u32,
    pub events: HashMap<u32, Vec<EventMeta>>,
}

impl Manager {
    pub fn new() -> Self {
        Self {
            event_id: 0,
            time: 0,
            events: HashMap::new(),
        }
    }

    pub fn gain_age(&mut self) {
        self.time += 1;
    }

    pub fn generate_event_id(&mut self) -> EventId {
        self.event_id += 1;
        self.event_id
    }

    // FIXME: Moveできないのか？
    pub fn flush_events(&mut self) -> Option<Vec<EventMeta>> {
        let current_time = self.time;
        let events = self.events.get(&current_time).cloned();
        self.events.remove(&current_time);
        events
    }

    // // EventListを返す？
    // pub fn flush_events(&mut self) -> Vec<Disk> {
    //     // FIXME:
    //     let mut spawn_disk_list = vec![];

    //     let current_time = self.time;
    //     let event_list = self.events.get_mut(&current_time);
    //     if let Some(event_list) = event_list {
    //         event_list.iter_mut().for_each(|event| {
    //             let ev = event.emit();
    //             match ev {
    //                 Some(EventMeta::SpawnDisk(disk)) => {
    //                     let replication_count = disk.event.replicate_by;
    //                     for _ in 0..replication_count {
    //                         spawn_disk_list.push(disk.disk.clone());
    //                     }
    //                 }
    //                 _ => (),
    //             }
    //         });
    //         self.events.remove(&current_time);
    //     }

    //     // FIXME:
    //     spawn_disk_list
    // }

    pub fn schedule_events(&mut self, new_event: EventMeta, at: u32) {
        // let id = self.generate_event_id();
        // let event = EventMetaSetting::new(id, 10, 20, 10);
        // https://doc.rust-jp.rs/book-ja/ch06-01-defining-an-enum.html
        // let spawn_disk_event = EventMeta::SpawnDisk(SpawnDisk::new(250, 250, event));
        // https://qiita.com/hystcs/items/75183bcf38bf95cc2ce0
        &self.events.entry(at).or_insert(Vec::new()).push(new_event);
    }

    pub fn refresh_events(&mut self) {}
}
