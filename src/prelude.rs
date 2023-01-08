#[cfg(feature = "Window")]
pub trait WindowExtra {
    fn inner_width_as_u32(&self) -> u32;
    fn inner_height_as_u32(&self) -> u32;
}

#[cfg(feature = "Window")]
impl WindowExtra for web_sys::Window {
    fn inner_width_as_u32(&self) -> u32 {
        self.inner_width().unwrap().as_f64().unwrap() as u32
    }
    fn inner_height_as_u32(&self) -> u32 {
        self.inner_height().unwrap().as_f64().unwrap() as u32
    }
}

#[cfg(feature = "HtmlCanvasElement")]
pub trait HtmlCanvasElementExtra {
    #[cfg(feature = "CanvasRenderingContext2d")]
    fn get_context_2d(&self) -> Result<web_sys::CanvasRenderingContext2d, JsValue>;
}

// impl HtmlCanvasElementExtra 