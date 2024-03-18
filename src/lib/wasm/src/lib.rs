// NOTE: これがあるとビルドが反映されない
// extern crate wasm_bindgen;
mod core;
mod utils;
mod web_interface;

use core::{
    disk,
    event::{self, EventId, EventSetting, SpawnDisk},
    game, render, schedule, setting,
};
use utils::coordinate::Coordinate;
use wasm_bindgen::prelude::*;
use web_interface::canvas::{self, Canvas};
use web_sys::console::log_1;

pub fn log(s: &String) {
    log_1(&JsValue::from(s));
}

// const FRAMES_PER_SEC: u32 = 60;
// const MILLI_SECONDS: u32 = 1_000;
// const DISK_NUM: u32 = 4_096;
const MARGIN: u32 = 10;
const PADDING: u32 = 5;
const FONT_SIZE: u32 = 16;
const LINE_MARGIN: u32 = 2;

// WebApp専用のマネージャ
// #[derive(Debug)]
// #[wasm_bindgen]
// struct WebManager {
//     game_manager: game::Manager,
// }

// REF: https://stackoverflow.com/questions/70075966/is-it-possible-to-change-generic-lifetime-of-a-struct

// #[derive(Debug)]
#[wasm_bindgen]
pub struct Screen {
    game_manager: Option<game::Manager>,
    render_manager: Option<render::Manager>,
    disk_manager: Option<disk::Manager>,
    schedule_manager: Option<schedule::Manager>,
    canvas: Canvas,

    // for fps calculation
    fps_counter: u32,
    fps_time: u32,
    last_fps: u32,

    // FIXME: for debug
    speed: u32,
    degree: u32,
    disk_num: u32,
    interval_degree: u32,
    spawn_interval: u32,
    degree_change_per: u32,
    offset: u32,
    update_angle: Option<f64>,
    update_speed: Option<f64>,
    change_angle: f64,
    change_speed: f64,
}

#[wasm_bindgen]
impl Screen {
    pub fn new(
        id: String,
        width: u32,
        height: u32,
        speed: u32,
        degree: u32,
        disk_num: u32,
        interval_degree: u32,
        spawn_interval: u32,
        degree_change_per: u32,
        offset: u32,
        update_angle: Option<f64>,
        update_speed: Option<f64>,
        change_angle: f64,
        change_speed: f64,
    ) -> Self {
        let sprite_sheet = utils::dom::image("sprite").unwrap();
        let canvas = canvas::Canvas::new(id.clone(), width, height, sprite_sheet);
        // renderer.prepare();

        Self {
            game_manager: None,
            render_manager: None,
            disk_manager: None,
            schedule_manager: None,
            canvas,
            fps_counter: 0,
            fps_time: 0,
            last_fps: 0,

            speed,
            degree,
            disk_num,
            interval_degree,
            spawn_interval,
            degree_change_per,
            offset,
            update_angle,
            update_speed,
            change_angle,
            change_speed,
        }
    }

    pub fn update_frame(&mut self, time: u32) {
        log!("Hello at: {}", time);

        self.on_animation_frame(time);
    }
}

impl Screen {
    fn assign_schedule_manager(&mut self, schedule_manager: schedule::Manager) {
        self.schedule_manager = Some(schedule_manager);
    }
    fn assign_disk_manager(&mut self, disk_manager: disk::Manager) {
        self.disk_manager = Some(disk_manager);
    }
    fn assign_render_manager(&mut self, render_manager: render::Manager) {
        self.render_manager = Some(render_manager);
    }
    fn assign_game_manager(&mut self, game_manager: game::Manager) {
        self.game_manager = Some(game_manager);
    }

    // FIXME:
    // fn update_from_mock(&self, event_id: EventId) -> Option<event::EventMeta> {
    //     Some(event::EventMeta::UpdateDisk(event::UpdateDisk::new(
    //         event_id, 50, 10,
    //     )))
    // }

    // FIXME:
    fn disk_from_mock(&mut self) -> Option<event::EventMeta> {
        let manager = self.schedule_manager.as_mut().unwrap();
        let time = manager.time;
        if time % self.spawn_interval == 0 {
            let id = manager.generate_event_id();
            let coord = Coordinate::new(250., 250.);
            let new_event = EventSetting::new(id, coord, 10, 20);
            let degree_change = (time * self.degree_change_per) % 360; // FIXME:
            Some(event::EventMeta::SpawnDisk(SpawnDisk::new(
                self.degree as u16 + degree_change as u16,
                self.speed,
                self.disk_num,
                self.interval_degree,
                self.offset,
                self.update_angle,
                self.update_speed,
                new_event,
            )))
        } else {
            None
        }
    }

    fn on_animation_frame(&mut self, time: u32) -> Option<()> {
        let ev = self.disk_from_mock();
        let schedule_manager = self.schedule_manager.as_mut()?;
        let disk_manager = self.disk_manager.as_mut()?;
        // let render_manager = self.render_manager.as_ref()?;
        schedule_manager.gain_age();

        // FIXME: ダミーのイベント登録
        if let Some(ev) = ev {
            // let ev_update = self.update_from_mock(ev.event);
            schedule_manager.schedule_events(ev.clone(), schedule_manager.time + 60);
        }
        let event_meta_list = schedule_manager.flush_events();

        if let Some(event_meta_list) = event_meta_list {
            event_meta_list.into_iter().for_each(|e| match e {
                event::EventMeta::SpawnDisk(spawn_disk) => {
                    // FIXME: ひとまず10fr後にUpdateする
                    let ev_update = event::EventMeta::UpdateDisk(event::UpdateDisk::new(
                        spawn_disk.event.event_id,
                        self.change_angle as u16,
                        self.change_speed as u32,
                    ));
                    schedule_manager.schedule_events(ev_update, schedule_manager.time + 10);
                    disk_manager.add_disks_from_event(spawn_disk);
                }
                event::EventMeta::UpdateDisk(update_disk) => {
                    disk_manager.update_disks_from_event(update_disk);
                }
                // TODO: add arms for each event_meta_type
                _ => (),
            });
        }

        let disks = disk_manager.disks();
        let disk_count = disks.iter().filter(|disk| disk.is_some()).count();

        // FIXME:
        if disk_count > 2000 {
            disk_manager.sweep(self.canvas.width, self.canvas.height);
        }

        // disk_manager.add_disks(disks);
        disk_manager.update_disks();
        // log!("disks");
        // log!("{:?}", disk_manager.disks());

        /* FPS更新 */
        self.calc_fps(time);

        /* 画面表示更新 */
        self.draw();

        Some(())
    }

    fn draw(&self) -> Option<()> {
        let disk_manager = self.disk_manager.as_ref()?;
        let render_manager = self.render_manager.as_ref()?;
        let ctxt = &self.canvas.context;

        ctxt.save();

        let bg_color = "rgb(80, 80, 80, 1.0)";
        ctxt.set_fill_style(&JsValue::from(bg_color));
        ctxt.fill_rect(0., 0., self.canvas.width as f64, self.canvas.height as f64);

        /* 描画処理 */
        let disks = disk_manager.disks();
        let disk_count = disks.iter().filter(|disk| disk.is_some()).count();

        // FIXME:
        // if disk_count > 2000 {
        //     disk_manager.sweep(self.canvas.width, self.canvas.height);
        // }
        // log!(
        //     "{:?}",
        //     disks
        //         .iter()
        //         .filter_map(|&disk| disk)
        //         .collect::<Vec<&Disk>>()
        // );
        render_manager.render(&self.canvas, disks);

        /* Draw Monitor */
        self.draw_monitor(disk_count);

        ctxt.restore();

        Some(())
    }

    fn draw_monitor(&self, disk_count: usize) {
        let lines = vec![
            format!("FPS: {}", self.last_fps),
            format!("NUM: {}", disk_count),
        ];
        let line_count = lines.len();
        let width = 180.; // ひとまず固定値
        let height = PADDING * 2 + (FONT_SIZE + LINE_MARGIN * 2) * line_count as u32;
        let y_bound = self.canvas.height - (MARGIN + height);
        let x_bound = MARGIN;
        let ctxt = &self.canvas.context;

        // canvasのcontext設定
        ctxt.set_fill_style(&JsValue::from("rgba(0, 0, 0, 0.5)"));
        ctxt.fill_rect(x_bound as f64, y_bound as f64, width as f64, height as f64);
        ctxt.set_fill_style(&JsValue::from("rgb(255, 255, 255)"));
        ctxt.set_stroke_style(&JsValue::from("rgb(255, 255, 255)"));
        ctxt.set_font(format!("{}px sans-serif", FONT_SIZE).as_str());

        // モニタリング文字列の描画
        for (i, line) in lines.into_iter().enumerate() {
            ctxt.fill_text(
                line.as_str(),
                (MARGIN + PADDING) as f64,
                (y_bound + PADDING + FONT_SIZE + (FONT_SIZE + LINE_MARGIN * 2) * i as u32) as f64,
            )
            .unwrap();
        }
    }

    fn calc_fps(&mut self, time: u32) {
        if self.fps_time + 1000 < time {
            self.last_fps = self.fps_counter;
            self.fps_counter = 0;
            self.fps_time = time;
        }
        self.fps_counter += 1;
    }
}

#[wasm_bindgen]
pub fn initialize(options: JsValue) -> Screen {
    log!("Options: {:?}", options);
    let settings: setting::Manager = options.into_serde().unwrap();
    log!("{:?}", settings);

    let game_manager = game::Manager::new();
    let disk_manager = disk::Manager::new();
    let schedule_manager = schedule::Manager::new();
    let render_manager = render::Manager::new();

    let update_angle = if settings.update_angle == 0.0 {
        None
    } else {
        Some(settings.update_angle)
    };
    let update_speed = if settings.update_speed == 0.0 {
        None
    } else {
        Some(settings.update_speed)
    };
    // let change_angle = if settings.change_angle == 0.0 {
    //     None
    // } else {
    //     Some(settings.change_angle)
    // };
    // let change_speed = if settings.change_speed == 0.0 {
    //     None
    // } else {
    //     Some(settings.change_speed)
    // };

    let id = settings.canvas_id;
    let mut screen = Screen::new(
        id.clone(),
        settings.width,
        settings.height,
        settings.speed,
        settings.degree,
        settings.disk_num,
        settings.interval_degree,
        settings.spawn_interval,
        settings.degree_change_per,
        settings.offset,
        update_angle,
        update_speed,
        settings.change_angle,
        settings.change_speed,
    );
    screen.assign_disk_manager(disk_manager);
    screen.assign_schedule_manager(schedule_manager);
    screen.assign_game_manager(game_manager);
    screen.assign_render_manager(render_manager);
    screen.draw();
    screen
}

#[cfg(test)]
pub mod tests {
    #[test]
    pub fn test1() {
        assert_eq!(2 + 2, 4);
    }
}
