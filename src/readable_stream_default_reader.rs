use core::{future::Future, task::Poll};
use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

pub struct ReadFuture(JsFuture);

impl Future for ReadFuture {
    type Output = Result<Read, JsValue>;

    #[inline]
    fn poll(
        self: core::pin::Pin<&mut Self>,
        cx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Self::Output> {
        let this: &mut ReadFuture = unsafe { self.get_unchecked_mut() };
        let ReadFuture(future) = this;
        let result: Poll<Result<JsValue, JsValue>> = core::pin::Pin::new(future).poll(cx);
        match result {
            core::task::Poll::Ready(Ok(value)) => {
                let value = value.dyn_into::<js_sys::Object>().unwrap();
                let read: Read = if js_sys::Reflect::get(&value, &"done".into())
                    .unwrap()
                    .as_bool()
                    .unwrap()
                {
                    Read::Done
                } else {
                    let value = js_sys::Reflect::get(&value, &"value".into())
                        .unwrap()
                        .dyn_into::<Uint8Array>()
                        .unwrap();
                    Read::NotDone(value)
                };
                core::task::Poll::Ready(Ok(read))
            }
            core::task::Poll::Ready(Err(err)) => core::task::Poll::Ready(Err(err)),
            core::task::Poll::Pending => core::task::Poll::Pending,
        }
    }
}

pub enum Read {
    Done,
    NotDone(Uint8Array),
}

pub trait ReadableStreamDefaultReaderExt {
    fn read_as_obj(&self) -> ReadFuture;
}

impl ReadableStreamDefaultReaderExt for web_sys::ReadableStreamDefaultReader {
    #[inline]
    fn read_as_obj(&self) -> ReadFuture {
        ReadFuture(JsFuture::from(self.read()))
    }
}
