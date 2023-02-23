use core::{
    future::Future,
    marker::PhantomData,
    pin::Pin,
    task::{Context, Poll},
};

use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

#[cfg(feature = "ImageBitmap")]
pub struct CreateImageBitmapWithHtmlCanvasElementFuture<'a> {
    future: JsFuture,
    phantom: PhantomData<&'a ()>,
}

#[cfg(feature = "ImageBitmap")]
impl<'a> Future for CreateImageBitmapWithHtmlCanvasElementFuture<'a> {
    type Output = Result<web_sys::ImageBitmap, JsValue>;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.get_mut().future)
            .poll(cx)
            .map(|poll_res| poll_res?.dyn_into::<web_sys::ImageBitmap>())
    }
}

#[cfg(feature = "Window")]
pub trait WindowExt {
    // desugared async associated trait fn based on:
    // https://rust-lang.github.io/rfcs/3185-static-async-fn-in-trait.html
    #[doc(hidden)]
    #[cfg(feature = "ImageBitmap")]
    type CreateImageBitmapWithHtmlCanvasElement<'me>: Future<
        Output = Result<web_sys::ImageBitmap, JsValue>,
    >
    where
        Self: 'me;

    fn inner_width_as_u32(&self) -> u32;
    fn inner_height_as_u32(&self) -> u32;

    #[cfg(feature = "ImageBitmap")]
    fn create_image_bitmap_with_html_canvas_element_async(
        &self,
        canvas: &web_sys::HtmlCanvasElement,
    ) -> Result<Self::CreateImageBitmapWithHtmlCanvasElement<'_>, JsValue>;
}

#[cfg(feature = "Window")]
impl WindowExt for web_sys::Window {
    #[cfg(feature = "ImageBitmap")]
    type CreateImageBitmapWithHtmlCanvasElement<'me> =
        CreateImageBitmapWithHtmlCanvasElementFuture<'me>;

    #[inline]
    fn inner_width_as_u32(&self) -> u32 {
        self.inner_width().unwrap().as_f64().unwrap() as u32
    }
    #[inline]
    fn inner_height_as_u32(&self) -> u32 {
        self.inner_height().unwrap().as_f64().unwrap() as u32
    }

#[cfg(feature = "ImageBitmap")]
    fn create_image_bitmap_with_html_canvas_element_async(
        &self,
        canvas: &web_sys::HtmlCanvasElement,
    ) -> Result<Self::CreateImageBitmapWithHtmlCanvasElement<'_>, JsValue> {
        let future = JsFuture::from(self.create_image_bitmap_with_html_canvas_element(canvas)?);
        Ok(CreateImageBitmapWithHtmlCanvasElementFuture {
            future,
            phantom: PhantomData,
        })
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
            .map(|ctx| JsCast::dyn_into::<web_sys::CanvasRenderingContext2d>(ctx).unwrap())
            .ok_or_else(|| panic!("Failed to get 2d context"))
    }
}

pub trait DocumentExt {
    fn disable_context_menu(&self);
}

#[cfg(all(feature = "Document", feature = "HtmlElement"))]
impl DocumentExt for web_sys::Document {
    #[inline]
    fn disable_context_menu(&self) {
        let handler = ::wasm_bindgen::closure::Closure::<dyn ::core::ops::FnMut(_)>::new::<_>(
            |event: web_sys::Event| {
                event.prevent_default();
            },
        );
        self.body()
            .unwrap()
            .set_oncontextmenu(Some(::wasm_bindgen::JsCast::unchecked_ref(
                handler.as_ref(),
            )));
        ::wasm_bindgen::closure::Closure::forget(handler);
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

impl Get2DOffsets for (f64, f64) {
    #[inline]
    fn get_2d_offsets(&self) -> (f64, f64) {
        (self.0, self.1)
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
