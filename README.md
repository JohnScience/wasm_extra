# wasm_extra

> Extra utilities for WASM.

So far, the only thing that this crate offers is the `add_event_listener_with_callback!`
macro named after the `::web_sys::EventTarget::add_event_listener_with_callback()` function.

The macro is a bit more ergonomic to use and its usage is easier to understand than
the corresponding code without it.

![side-by-side comparison of the macro and the function](https://i.imgur.com/Bjy4bcV.png)
