pub use wasm_extra_macros::add_event_listener_with_callback;
pub use wasm_extra_macros::add_event_listener_with_fn_once_callback;

pub mod prelude;
#[cfg(feature = "ReadableStream")]
pub mod readable_stream;
#[cfg(feature = "ReadableStreamDefaultReader")]
pub mod readable_stream_default_reader;
