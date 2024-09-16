use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlCanvasElement, CanvasRenderingContext2d};
use core::cell::RefCell;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let document = window().unwrap().document().unwrap();
    let canvas = document
        .get_element_by_id("animation-canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()?;

    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()?;

    let mut x_pos = 0.0;

    // Specify the concrete type for None
    let f: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        context.clear_rect(0.0, 0.0, canvas.width().into(), canvas.height().into());

        context.begin_path();
        context.arc(x_pos, 150.0, 50.0, 0.0, std::f64::consts::PI * 2.0).unwrap();
        context.set_fill_style(&JsValue::from_str("red"));
        context.fill();

        x_pos += 2.0;
        if x_pos > canvas.width() as f64 {
            x_pos = 0.0;
        }

        window()
            .unwrap()
            .request_animation_frame(f.borrow().as_ref().unwrap().as_ref().unchecked_ref())
            .unwrap();
    }) as Box<dyn FnMut()>));

    window()
        .unwrap()
        .request_animation_frame(g.borrow().as_ref().unwrap().as_ref().unchecked_ref())?;

    Ok(())
}
