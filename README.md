# wasm_extra

> Extra utilities for WASM.

So far, the only thing that this crate offers is the `add_event_listener_with_callback!`
macro named after the `::web_sys::EventTarget::add_event_listener_with_callback()` function.

The macro is a bit more ergonomic to use and code with it is easier to mentally parse than
the corresponding code without it.

![side-by-side comparison of the macro and the function](https://i.imgur.com/Bjy4bcV.png)

In the expanded code, the event target appears only on line 16, whereas in the macro it's the
very first argument, which then followed by the event name, then "closure prologue", and - lastly -
the closure itself.

"Closure prologue" is a stipulative term for the code that is executed before the closure itself. It's useful for
preparing the closure's environment, for example, by cloning the variables before capturing.
