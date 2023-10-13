#![forbid(unsafe_code)]

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, DeriveInput, Ident, Token,
};

struct Args {
    _trait: Ident,
    end_point: Ident,
}

impl Parse for Args {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let _trait = match &*input.parse::<Ident>()?.to_string().to_lowercase() {
            "get" => Ident::new("HttpFetcher", Span::call_site()),
            "post" => Ident::new("HttpSender", Span::call_site()),
            a => {
                return Err(syn::Error::new_spanned(
                    a,
                    "unsupported api method attribute",
                ))
            }
        };
        let _: Token![,] = input.parse()?;
        let end_point: Ident = input.parse()?;
        Ok(Args { _trait, end_point })
    }
}

#[proc_macro_attribute]
pub fn api(args: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as DeriveInput);
    let args = parse_macro_input!(args as Args);

    let name = item.ident.clone();
    let _trait = args._trait;
    let end_point = args.end_point;

    let expanded = quote! {
        #item

        impl dtcm_angel_utils::http::Api for #name {
            fn end_point() -> dtcm_angel_utils::http::EndPoint {
                dtcm_angel_utils::http::EndPoint::#end_point
            }
            fn url() -> String {
                Self::end_point().url()
            }
        }

        impl dtcm_angel_utils::http::#_trait for #name {}
    };

    proc_macro::TokenStream::from(expanded)
}
