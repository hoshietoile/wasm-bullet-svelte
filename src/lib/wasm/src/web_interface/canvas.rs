use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement};

use crate::{
    core::{disk::Disk, render::Renderer},
    utils::dom::{canvas, get_context2d_by_id},
};

#[derive(Debug, Clone)]
pub struct Canvas {
    id: String,
    pub el: HtmlCanvasElement,
    pub context: CanvasRenderingContext2d,
    pub width: u32,
    pub height: u32,
    // pub bound_rect: BoundRect, // TODO:
    pub sprite_sheet: HtmlImageElement,
}

impl Canvas {
    pub fn new(id: String, width: u32, height: u32, sprite_sheet: HtmlImageElement) -> Self {
        let element = canvas(&id).unwrap();
        let context = get_context2d_by_id(&id, width).unwrap();
        Self {
            id,
            el: element,
            context,
            width,
            height,
            sprite_sheet,
        }
    }

    pub fn prepare(&mut self) {
        // let ctxt = self.context;
    }
}

impl Renderer for Canvas {
    // FIXME: render-targetsの抽象化
    fn render(&self, render_targets: Vec<Option<&Disk>>) {
        for disk in render_targets.iter() {
            match disk {
                Some(disk) => {
                    let sprite_space = (10., 0., 10., 10.);
                    self.context.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                        &self.sprite_sheet,
                        sprite_space.0,
                        sprite_space.1,
                        sprite_space.2,
                        sprite_space.3,
                        disk.coord.x as f64,
                        disk.coord.y as f64,
                        10.,
                        10.,
                    ).unwrap();
                }
                _ => continue,
            }
        }
    }
}
