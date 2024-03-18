use wasm_bindgen::JsCast;
use web_sys::{
    CanvasRenderingContext2d, Document, HtmlCanvasElement, HtmlImageElement, WebGlProgram,
    WebGlRenderingContext, WebGlShader, Window,
};

pub fn window() -> Option<Window> {
    web_sys::window()
}

pub fn document() -> Option<Document> {
    window().and_then(|w| w.document())
}

pub fn image(id: &str) -> Option<HtmlImageElement> {
    document()
        .and_then(|d| d.get_element_by_id(id))
        .and_then(|el| el.dyn_into::<HtmlImageElement>().ok())
}

pub fn canvas(id: &str) -> Option<HtmlCanvasElement> {
    document()
        .and_then(|d| d.get_element_by_id(id))
        .and_then(|el| el.dyn_into::<HtmlCanvasElement>().ok())
}

pub fn get_context2d_by_id(id: &str, width: u32) -> Option<CanvasRenderingContext2d> {
    canvas(id)
        .and_then(|c| {
            c.set_width(width);
            c.set_height(width);
            c.get_context("2d").ok()
        })
        .and_then(|c| c.unwrap().dyn_into::<CanvasRenderingContext2d>().ok())
}

pub fn get_webgl_context_by_id(id: &str, width: u32) -> Option<WebGlRenderingContext> {
    canvas(id)
        .and_then(|c| {
            c.set_width(width);
            c.set_height(width);
            c.get_context("webgl").ok()
        })
        .and_then(|c| c.unwrap().dyn_into::<WebGlRenderingContext>().ok())
        .and_then(|c| {
            c.viewport(0, 0, width as i32, width as i32);
            Some(c)
        })
}
