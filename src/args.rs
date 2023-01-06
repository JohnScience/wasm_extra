use proc_macro2::{Ident, TokenStream};
use syn::{parse::{Parse, ParseStream, ParseBuffer}, token::Comma, LitStr, braced, ExprClosure};
use quote::quote;

pub(crate) struct Args {
    pub(crate) target: Ident,
    pub(crate) event: LitStr,
    pub(crate) closure_prologue: TokenStream,
    pub(crate) closure: ExprClosure,
}

impl Parse for Args {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let target = input.parse::<Ident>()?;
        input.parse::<Comma>()?;
        let event = input.parse::<LitStr>()?;
        input.parse::<Comma>()?;
        let closure_prologue = {
            let contents: ParseBuffer;
            braced!(contents in input);
            contents.parse::<TokenStream>()
        }?;
        input.parse::<Comma>()?;
        let closure = input.parse::<ExprClosure>()?;
        
        Ok(Self { target, event, closure_prologue, closure })
    }
}

impl Args {
    pub(crate) fn handle(self) -> TokenStream {
        let Self { target, event, closure_prologue, closure } = self;
        quote! {
            {
                #closure_prologue
                let handler = ::wasm_bindgen::closure::Closure::<dyn ::core::ops::FnMut(_)>::new::<_>(
                    #closure
                );
                #target.add_event_listener_with_callback(
                    #event,
                    handler.as_ref().unchecked_ref()
                ).unwrap();
                ::wasm_bindgen::closure::Closure::forget(handler);
            }
        }
    }
}