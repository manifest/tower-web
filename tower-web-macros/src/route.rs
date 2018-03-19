use syn;
use quote::Tokens;

use std::fmt;

/// Represents a service route
pub struct Route {
    /// Function identifier
    pub ident: syn::Ident,

    /// Function return type
    pub ret: syn::Type,

    /// HTTP method
    pub method: Option<Method>,

    /// HTTP path
    pub path: Option<String>,
    pub path_lit: Option<syn::LitStr>,
}

#[derive(Debug)]
pub enum Method {
    Get,
}

impl Route {
    pub fn new(ident: syn::Ident, ret: syn::Type) -> Self {
        Route {
            ident,
            ret,
            method: None,
            path: None,
            path_lit: None,
        }
    }

    /// Returns `true` if the attribute is processed
    pub fn process_attr(&mut self, attr: &syn::Attribute) -> bool {
        if attr.path.segments.len() != 1 {
            return false;
        }

        match attr.path.segments[0].ident.as_ref() {
            "GET" => {
                self.set_method(Method::Get);

                let args: syn::LitStr = match syn::parse2(attr.tts.clone()) {
                    Ok(v) => v,
                    _ => unimplemented!(),
                };

                self.set_path(args.value());
                self.path_lit = Some(args);
            }
            _ => {
                return false;
            }
        }

        true
    }

    pub fn set_method(&mut self, value: Method) {
        self.method = Some(value);
    }

    pub fn set_path(&mut self, value: String) {
        self.path = Some(value);
    }
}

impl fmt::Debug for Route {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use quote::ToTokens;

        // TODO: Avoid escaping
        let ret = self.ret.clone().into_tokens().to_string();

        fmt.debug_struct("Route")
            .field("ident", &self.ident.as_ref())
            .field("ret", &ret)
            .field("method", &self.method)
            .field("path", &self.path)
            .finish()
    }
}

impl Method {
    pub fn to_tokens(&self) -> Tokens {
        use self::Method::*;

        match *self {
            Get => quote! {
                ::tower_web::codegen::http::Method::GET
            },
        }
    }
}