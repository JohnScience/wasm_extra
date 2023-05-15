use wasm_bindgen::prelude::*;
use web_sys::ReadableStream;

#[wasm_bindgen]
#[cfg(all(
    feature = "ReadableStreamDefaultReader",
    feature = "ReadableStreamDefaultController"
))]
extern "C" {
    #[wasm_bindgen(js_name = ReadableStream, extends = ReadableStream)]
    type ReadableStreamClass;

    #[wasm_bindgen(constructor, js_class = "ReadableStream")]
    fn new(source: &JsValue) -> ReadableStreamClass;
}

pub trait ReadableStreamExt {
    #[cfg(all(
        feature = "ReadableStreamDefaultReader",
        feature = "ReadableStreamDefaultController"
    ))]
    fn from_u8_vec(vec: Vec<u8>) -> Result<web_sys::ReadableStream, JsValue>;
}

impl ReadableStreamExt for web_sys::ReadableStream {
    #[cfg(all(
        feature = "ReadableStreamDefaultReader",
        feature = "ReadableStreamDefaultController"
    ))]
    fn from_u8_vec(vec: Vec<u8>) -> Result<web_sys::ReadableStream, JsValue> {
        let source = js_sys::Object::new();
        {
            let f = Closure::<dyn Fn(JsValue)>::new(move |controller: JsValue| {
                let controller = controller
                    .dyn_into::<web_sys::ReadableStreamDefaultController>()
                    .unwrap();
                let arr = js_sys::Uint8Array::from(vec.as_slice());
                controller.enqueue_with_chunk(&arr).unwrap();
                controller.close().unwrap();
            });
            js_sys::Reflect::set(
                &source,
                &JsValue::from_str("pull"),
                f.as_ref().unchecked_ref(),
            )
            .unwrap();

            f.forget();
        }
        Ok(ReadableStreamClass::new(&source).into())
    }
}
