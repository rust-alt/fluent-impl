/*
    This file is a part of term-string.

    Copyright (C) 2018 Mohammad AlSaleh <CE.Mohammad.AlSaleh at gmail.com>
    https://github.com/rust-alt/term-string

    This Source Code Form is subject to the terms of the Mozilla Public
    License, v. 2.0. If a copy of the MPL was not distributed with this
    file, You can obtain one at <http://mozilla.org/MPL/2.0/>.
*/

use proc_macro2::Span;
use syn::{
    punctuated::Punctuated, token::Comma, Attribute, Expr, FnArg, GenericParam, Generics, Ident, ImplItemMethod, Pat,
    ReturnType, Type, Visibility,
};

use config::{self, MacroConfig, MethodConfig};
use type_utils as t;

pub(crate) fn get_method_config(attrs: &[Attribute]) -> Result<MethodConfig, String> {
    let mut method_config = MethodConfig::default();

    for opts in attrs.iter().filter(|a| a.path == parse_quote! { fluent_impl_opts }) {
        let attr_info = config::parse_config_from_attr(&opts)?;
        method_config = config::get_method_config(attr_info, Some(method_config))?;
    }

    Ok(method_config)
}

pub(crate) fn try_fluentable(
    method: &ImplItemMethod,
    macro_config: &MacroConfig,
    method_config: &MethodConfig,
) -> Result<(), String> {
    let err_msg = "fluent_impl only applies to `&mut self` methods and no return value";

    // Check if method returns anything
    if method.sig.decl.output != ReturnType::Default {
        Err(err_msg)?
    }

    // Check if first arg is `&mut self`
    if let Some(first_arg) = method.sig.decl.inputs.first() {
        match first_arg.into_value() {
            FnArg::SelfRef(arg) => {
                if arg.mutability.is_none() {
                    Err(err_msg)?
                }
            },
            _ => Err(err_msg)?,
        }
    } else {
        Err(err_msg)?
    }

    match method.vis {
        Visibility::Public(_) => (),
        _ => if !macro_config.non_public && !method_config.non_public {
            Err("generating a chaining method from this non-public method was not enabled")?;
        },
    }

    if method_config.skip {
        Err("skip opt enabled")?;
    }

    Ok(())
}

pub(crate) fn fluent_from_fluentable(
    method: ImplItemMethod,
    macro_config: &MacroConfig,
    ty: &Type,
) -> Result<ImplItemMethod, String> {
    let mut fluent_method = method;
    let b_ident = fluent_method.sig.ident.clone();
    let doc = fluent_doc(&fluent_method, macro_config)?;
    let doc = doc.replace("%f%", &fluent_method.sig.ident.to_string());
    let doc = doc.replace("%t%", &t::bare_ty_str(ty)?);

    fluent_method.sig.ident = Ident::new(&fluent_ident(&fluent_method, macro_config)?, Span::call_site());
    // Remove original doc and add ours
    fluent_method.attrs.retain(|a| a.path != parse_quote!{ doc });
    fluent_method.attrs.push(parse_quote! { #[doc = #doc] });

    // Always Some
    match fluent_method.sig.decl.inputs.iter_mut().nth(0) {
        Some(first_arg) => *first_arg = parse_quote! { mut self },
        None => unreachable!(),
    };

    fluent_method.sig.decl.output = parse_quote! { -> Self };
    simplify_fn_args(&mut fluent_method.sig.decl.inputs);
    let call_args = get_call_args(&fluent_method.sig.decl.inputs);
    let generic_params = get_generic_params(&fluent_method.sig.decl.generics);
    fluent_method.block = parse_quote! { { self.#b_ident::<#generic_params>(#call_args); self } };

    Ok(fluent_method)
}

fn get_generic_params(generics: &Generics) -> Punctuated<Ident, Comma> {
    let mut ret = Punctuated::new();
    for param in &generics.params {
        match param {
            GenericParam::Lifetime(l) => {
                ret.push_value(l.lifetime.ident.clone());
                ret.push_punct(Comma::new(Span::call_site()));
            },
            GenericParam::Const(p) => {
                ret.push_value(p.ident.clone());
                ret.push_punct(Comma::new(Span::call_site()));
            },
            GenericParam::Type(t) => {
                ret.push_value(t.ident.clone());
                ret.push_punct(Comma::new(Span::call_site()));
            },
        }
    }
    ret
}

// Replace non-ident arg params with idents.
// Check tests/run-pass/pattern_args.rs where without this
// we will get errors.
fn simplify_fn_args(inputs: &mut Punctuated<FnArg, Comma>) {
    for param in inputs.iter_mut().enumerate() {
        if let (idx, FnArg::Captured(cap)) = param {
            match cap.pat {
                Pat::Ident(_) => (),
                _ => {
                    let ident = Ident::new(&format!("arg{}", idx), Span::call_site());
                    cap.pat = parse_quote! { #ident };
                },
            }
        }
    }
}

fn get_call_args(inputs: &Punctuated<FnArg, Comma>) -> Punctuated<Expr, Comma> {
    let mut ret = Punctuated::new();
    for param in inputs {
        match param {
            FnArg::SelfRef(_) | FnArg::SelfValue(_) => (),
            FnArg::Captured(cap) => {
                let pat = &cap.pat;
                let expr: Expr = parse_quote! { #pat };
                ret.push_value(expr);
                ret.push_punct(Comma::new(Span::call_site()));
            },
            FnArg::Inferred(pat) => {
                let expr: Expr = parse_quote! { #pat };
                ret.push_value(expr);
                ret.push_punct(Comma::new(Span::call_site()));
            },
            FnArg::Ignored(ty) => {
                let expr: Expr = parse_quote! { #ty };
                ret.push_value(expr);
                ret.push_punct(Comma::new(Span::call_site()));
            },
        }
    }
    ret
}

fn fluent_doc(method: &ImplItemMethod, macro_config: &MacroConfig) -> Result<String, String> {
    let method_config = get_method_config(&method.attrs)?;
    let mut doc = if let Some(doc) = method_config.doc {
        doc
    } else {
        macro_config.doc.clone()
    };
    doc += "\n\n [`%f%`]: %t%::%f%";
    doc += "\n [`%f%()`]: %t%::%f%";
    Ok(doc)
}

fn fluent_ident(method: &ImplItemMethod, macro_config: &MacroConfig) -> Result<String, String> {
    let method_config = get_method_config(&method.attrs)?;

    if let Some(name) = method_config.name {
        return Ok(name);
    }

    let b_ident = if let Some(rename) = method_config.rename {
        rename
    } else {
        method.sig.ident.to_string()
    };

    let prefix = if let Some(prefix) = method_config.prefix {
        prefix
    } else {
        macro_config.prefix.clone()
    };

    let ident_str = prefix + &b_ident;
    Ok(ident_str)
}
