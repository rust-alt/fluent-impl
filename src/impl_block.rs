/*
    This file is a part of term-string.

    Copyright (C) 2018 Mohammad AlSaleh <CE.Mohammad.AlSaleh at gmail.com>
    https://github.com/rust-alt/term-string

    This Source Code Form is subject to the terms of the Mozilla Public
    License, v. 2.0. If a copy of the MPL was not distributed with this
    file, You can obtain one at <http://mozilla.org/MPL/2.0/>.
*/

use proc_macro2::TokenStream as TokenStream2;
use quote::ToTokens;
use syn::{ImplItem, ImplItemMethod, ItemImpl};

use config::MacroConfig;
use method as m;
use type_utils as t;

pub(crate) fn gen_fluent_from_impl_block(impl_block: &ItemImpl, macro_config: &MacroConfig) -> Result<TokenStream2, String> {
    if impl_block.trait_.is_some() {
        Err("fluent_impl does not apply to trait impl blocks")?
    }

    let mut input = TokenStream2::new();
    gen_fluent_inblock(&impl_block, macro_config)?.to_tokens(&mut input);

    let new_impl_block = gen_fluent_new_block(&impl_block, macro_config)?;
    if !new_impl_block.items.is_empty() {
        let mut block = new_impl_block;
        let bare_ty_str = t::bare_ty_str(&block.self_ty)?;
        let doc = macro_config.impl_doc.clone().replace("%t%", &bare_ty_str);
        block.attrs.push(parse_quote! { #[doc = #doc] });
        block.to_tokens(&mut input);
    }

    Ok(input)
}

fn gen_fluent_inblock(impl_block: &ItemImpl, macro_config: &MacroConfig) -> Result<ItemImpl, String> {
    let mut inblock_impl_block = impl_block.clone();
    let mut added_count = 0;
    let mut added_methods_pos: Vec<(usize, ImplItemMethod)> = Vec::with_capacity(16);

    for (pos, impl_item) in inblock_impl_block.items.iter().enumerate() {
        if let ImplItem::Method(method) = impl_item {
            let method_config = m::get_method_config(&method.attrs)?;
            if m::try_fluentable(method, macro_config, &method_config).is_ok()
                && (macro_config.inblock || method_config.inblock)
            {
                let ty = &inblock_impl_block.self_ty;
                added_methods_pos.push((
                    pos + added_count + 1,
                    m::fluent_from_fluentable(method.clone(), &macro_config, ty)?,
                ));
                added_count += 1;
            }
        }
    }

    for (pos, method) in added_methods_pos {
        inblock_impl_block.items.insert(pos, ImplItem::Method(method));
    }
    Ok(inblock_impl_block)
}

fn gen_fluent_new_block(impl_block: &ItemImpl, macro_config: &MacroConfig) -> Result<ItemImpl, String> {
    let mut new_impl_block = impl_block.clone();
    new_impl_block.items = Vec::with_capacity(16);

    if !macro_config.inblock {
        for impl_item in &impl_block.items {
            if let ImplItem::Method(method) = impl_item {
                let method_config = m::get_method_config(&method.attrs)?;
                if m::try_fluentable(method, macro_config, &method_config).is_ok() && !method_config.inblock {
                    new_impl_block.items.push(ImplItem::Method(m::fluent_from_fluentable(
                        method.clone(),
                        macro_config,
                        &new_impl_block.self_ty,
                    )?));
                }
            }
        }
    }

    Ok(new_impl_block)
}
