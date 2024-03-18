// use web_sys::CanvasRenderingContext2d;

use super::disk::Disk;

pub trait Renderer {
    // fn instanciate();
    fn render(&self, render_targets: Vec<Option<&Disk>>);
}

pub struct Manager {
    // renderer: CanvasRenderingContext2d, // FIXME: 将来的にtraitにしたい
    // renderer: impl Renderer, // FIXME: 将来的にtraitにしたい
}

impl Manager {
    pub fn new() -> Self {
        Self {}
        // Self { renderer }
    }

    pub fn render(&self, renderer: &impl Renderer, disks: Vec<Option<&Disk>>) {
        // FIXME: disk をレンダリングに必要な要素に分解してrenderに渡す
        renderer.render(disks)
    }
}
