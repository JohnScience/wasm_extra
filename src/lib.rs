use proc_macro::TokenStream;
use syn::parse_macro_input;

mod args;

use args::Args;

#[proc_macro]
pub fn add_event_listener_with_callback(ts: TokenStream) -> TokenStream {
    let args = parse_macro_input!(ts as Args);
    args.handle().into()
}
