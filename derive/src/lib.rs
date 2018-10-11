#![allow(dead_code, unused_variables)]
#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{DeriveInput, Data};

// TODO: Handle generics, attributes and visibility correctly
// TODO: Maybe better error messages
fn impl_event_struct(input: &DeriveInput) -> TokenStream {
    let (name, vis, attrs, generics, data) = (
        &input.ident, &input.vis, &input.attrs, &input.generics, &input.data,
    );

    let data = match data {
        Data::Struct(d) => d,
        _ => unreachable!(),
    };

    let tokens = quote! {
        unsafe impl ::circini_core::Event for #name {
            fn filter_any(ev: ::circini_core::AnyEvent) -> Option<Self> {
                if ev.get_id() == ::std::any::TypeId::of::<Self>() {
                    unsafe {
                        Some(ev.downcast_into_unchecked())
                    }
                } else {
                    None
                }
            }

            fn upcast_to_any(self) -> ::circini_core::AnyEvent {
                unsafe {
                    ::circini_core::AnyEvent::new(self)
                }
            }

            fn check_any(ev: &::circini_core::AnyEvent) -> bool {
                ev.get_id() == ::std::any::TypeId::of::<Self>()
            }
        }
    };

    tokens.into()
}


// TODO: Implement event families (proc macro for enum)
fn impl_event_enum(input: &DeriveInput) -> TokenStream {
    unimplemented!()
}

// TODO: Helpful error message (`#[derive(Event)]` should never work on unions)
fn impl_event_union(input: &DeriveInput) -> TokenStream {
    unimplemented!()
}

#[proc_macro_derive(Event)]
pub fn derive_event(input: TokenStream) -> TokenStream {

    let input = parse_macro_input!(input as DeriveInput);

    let output = match input.data {
        Data::Struct(_) => impl_event_struct(&input),
        Data::Enum(_) => impl_event_enum(&input),
        Data::Union(_) => impl_event_union(&input),
    };

    output.into()
}


struct MatchEventInput;

#[proc_macro]
pub fn match_event(input: TokenStream) -> TokenStream {
    unimplemented!()
}