use proc_macro2::{Ident, TokenStream};
use syn::{parse::{Parse, ParseStream, ParseBuffer}, token::{Comma, And}, LitStr, braced, ExprClosure};
use quote::{quote, ToTokens};

pub(crate) enum EventTarget {
    SinglyReferred(Ident),
    DoublyReferred(Ident),
}

impl EventTarget {
    pub(crate) fn target_obj_ident(&self) -> EventTargetObjIdent {
        match self {
            Self::SinglyReferred(ident) => EventTargetObjIdent::SinglyReferred(ident),
            Self::DoublyReferred(_) => EventTargetObjIdent::DoublyReferred,
        }
    }
    pub(crate) fn preprologue(&self) -> Option<ClosurePreprologue> {
        match self {
            Self::SinglyReferred(_) => None,
            Self::DoublyReferred(ident) => Some(ClosurePreprologue { doubly_referred_target: ident }),
        }
    }
}

pub(crate) struct ClosurePreprologue<'a> {
    pub(crate) doubly_referred_target: &'a Ident,
}


impl<'a> ToTokens for ClosurePreprologue<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { doubly_referred_target } = self;
        tokens.extend(quote! {
            let __event_target_alias = #doubly_referred_target.as_ref();
        });
    }
}

pub(crate) enum EventTargetObjIdent<'a> {
    SinglyReferred(&'a Ident),
    DoublyReferred,
}

impl<'a> ToTokens for EventTargetObjIdent<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::SinglyReferred(ident) => tokens.extend(quote! { #ident }),
            Self::DoublyReferred => tokens.extend(quote! { __event_target_alias }),
        }
    }
}

pub(crate) struct Args {
    pub(crate) target: EventTarget,
    pub(crate) event: LitStr,
    pub(crate) closure_prologue: TokenStream,
    pub(crate) closure: ExprClosure,
}

impl Parse for EventTarget {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(And) {
            input.parse::<And>()?;
            let ident = input.parse::<Ident>()?;
            Ok(Self::DoublyReferred(ident))
        } else {
            let ident = input.parse::<Ident>()?;
            Ok(Self::SinglyReferred(ident))
        }
    }
}

impl Parse for Args {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let target = input.parse::<EventTarget>()?;
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
        let target_obj_ident = target.target_obj_ident();
        let preprologue: Option<ClosurePreprologue> = target.preprologue();
        quote! {
            {
                #preprologue
                #closure_prologue
                let __handler = ::wasm_bindgen::closure::Closure::<dyn ::core::ops::FnMut(_)>::new::<_>(
                    #closure
                );
                #target_obj_ident.add_event_listener_with_callback(
                    #event,
                    __handler.as_ref().unchecked_ref()
                ).unwrap();
                ::wasm_bindgen::closure::Closure::forget(handler);
            }
        }
    }
}