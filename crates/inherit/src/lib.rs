extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;
#[macro_use]
extern crate quote;
use proc_macro::{TokenStream as StdStream, TokenTree as StdTree};
use proc_macro2::{Delimiter, Group, Ident, Span, TokenStream, TokenTree};
#[proc_macro_attribute]
pub fn inherit(attr: StdStream, input: StdStream) -> StdStream {
    let mut out: TokenStream = input.into();
    for item in attr.into_iter() {
        match item {
            StdTree::Ident(ident) => {
                let name = format!("{}", ident);
                match name.as_str() {
                    "Node" => out = node(out),
                    "Expression" => out = expr(out),
                    "Statement" => out = stmt(out),
                    "Function" => out = func(out),
                    "Pattern" => out = pat(out),
                    "Declaration" => out = decl(out),
                    "ImportSpecifier" => out = mod_spec(out, true),
                    // "ExportSpecifier" => out = mod_spec(out, false),
                    _ => panic!("Unknown inherit target"),
                }
            }
            _ => (),
        }
    }
    out.into()
}

fn node(input: TokenStream) -> TokenStream {
    let (name, stream) = inherit_node(input);
    impl_node(stream, &name, &name)
}

fn expr(input: TokenStream) -> TokenStream {
    let (name, stream) = inherit_node(input);
    let kind = format!("{}Expression", name);
    impl_node(stream, &name, &kind)
}

fn stmt(input: TokenStream) -> TokenStream {
    let (name, stream) = inherit_node(input);
    let kind = format!("{}Statement", name);
    impl_node(stream, &name, &kind)
}

fn func(input: TokenStream) -> TokenStream {
    add_props(
        input,
        "pub id: Option<Identifier>,
    pub params: Vec<Pattern>,
    pub generator: bool,",
    )
}

fn pat(input: TokenStream) -> TokenStream {
    let (name, stream) = inherit_node(input);
    let kind = format!("{}Pattern", name);
    impl_node(stream, &name, &kind)
}

fn decl(input: TokenStream) -> TokenStream {
    let (name, stream) = inherit_node(input);
    let kind = format!("{}Declaration", name);
    impl_node(stream, &name, &kind)
}

fn mod_spec(input: TokenStream, import: bool) -> TokenStream {
    let (name, stream) = inherit_node(input);
    let prop = if import {
        "pub local: Identifier,"
    } else {
        "pub exported: Identifier,"
    };
    let updated = add_props(stream, prop);
    let kind = format!("{}Specifier", name);
    impl_node(updated, &name, &kind)
}

fn inherit_node(input: TokenStream) -> (String, TokenStream) {
    let mut at_name = false;
    let mut name = String::new();
    input.clone().into_iter().for_each(|t| match &t {
        proc_macro2::TokenTree::Ident(ref ident) => {
            if at_name {
                name = format!("{}", ident);
            }
            at_name = format!("{}", ident).as_str() == "struct";
        }
        _ => (),
    });
    let updated = add_props(input, "pub loc: SourceLocation,");
    (name, updated)
}

fn add_props(input: TokenStream, props: &str) -> TokenStream {
    input
        .into_iter()
        .map(|t| {
            match &t {
                proc_macro2::TokenTree::Group(ref grp) => {
                    if grp.delimiter() == Delimiter::Brace {
                        let mut extended = grp.stream();
                        let extension: StdStream = props.parse().unwrap();
                        let extension: TokenStream = extension.into();
                        extended.extend(extension);
                        return TokenTree::Group(Group::new(Delimiter::Brace, extended));
                    }
                }
                _ => (),
            }
            t
        }).collect()
}

fn impl_node(input: TokenStream, name: &str, kind: &str) -> TokenStream {
    let name = Ident::new(name, Span::call_site());
    let kind = Ident::new(kind, Span::call_site());
    quote! {
        #input
        impl Node for #name {
            fn loc(&self) -> SourceLocation {
                self.loc.clone()
            }

            fn kind(&self) -> NodeKind {
                NodeKind::#kind
            }
        }
    }
}
