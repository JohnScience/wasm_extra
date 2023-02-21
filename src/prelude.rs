use wasm_bindgen::prelude::*;

#[cfg(feature = "Window")]
pub trait WindowExt {
    fn inner_width_as_u32(&self) -> u32;
    fn inner_height_as_u32(&self) -> u32;
}

#[cfg(feature = "Window")]
impl WindowExt for web_sys::Window {
    #[inline]
    fn inner_width_as_u32(&self) -> u32 {
        self.inner_width().unwrap().as_f64().unwrap() as u32
    }
    #[inline]
    fn inner_height_as_u32(&self) -> u32 {
        self.inner_height().unwrap().as_f64().unwrap() as u32
    }
}

#[cfg(feature = "HtmlCanvasElement")]
pub trait HtmlCanvasElementExt {
    #[cfg(feature = "CanvasRenderingContext2d")]
    fn get_context_2d(&self) -> Result<web_sys::CanvasRenderingContext2d, JsValue>;
}

#[cfg(feature = "HtmlCanvasElement")]
impl HtmlCanvasElementExt for web_sys::HtmlCanvasElement {
    #[cfg(feature = "CanvasRenderingContext2d")]
    #[inline]
    fn get_context_2d(&self) -> Result<web_sys::CanvasRenderingContext2d, JsValue> {
        self.get_context("2d")?
            .map(|ctx| ctx.dyn_into::<web_sys::CanvasRenderingContext2d>().unwrap())
            .ok_or_else(|| panic!("Failed to get 2d context"))
    }
}

pub trait Get2DOffsets {
    fn get_2d_offsets(&self) -> (f64, f64);
}

#[cfg(feature = "MouseEvent")]
impl Get2DOffsets for web_sys::MouseEvent {
    #[inline]
    fn get_2d_offsets(&self) -> (f64, f64) {
        (self.offset_x() as f64, self.offset_y() as f64)
    }
}

impl Get2DOffsets for (f64,f64) {
    #[inline]
    fn get_2d_offsets(&self) -> (f64, f64) {
        (self.0,self.1)
    }
}

#[cfg(feature = "CanvasRenderingContext2d")]
pub trait CanvasRenderingContext2dExt {
    fn reset_path<T: Get2DOffsets>(&self, source_of_offsets: &T);
    fn add_line_stroke<T: Get2DOffsets>(&self, source_of_offsets: &T);
}

#[cfg(feature = "CanvasRenderingContext2d")]
impl CanvasRenderingContext2dExt for web_sys::CanvasRenderingContext2d {
    #[inline]
    fn reset_path<T: Get2DOffsets>(&self, source_of_offsets: &T) {
        self.begin_path();
        let (offset_x, offset_y) = source_of_offsets.get_2d_offsets();
        self.move_to(offset_x, offset_y);
    }
    #[inline]
    fn add_line_stroke<T: Get2DOffsets>(&self, source_of_offsets: &T) {
        let (offset_x, offset_y) = source_of_offsets.get_2d_offsets();
        self.line_to(offset_x, offset_y);
        self.stroke();
    }
}
