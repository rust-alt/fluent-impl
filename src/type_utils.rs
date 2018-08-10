/*
    This file is a part of term-string.

    Copyright (C) 2018 Mohammad AlSaleh <CE.Mohammad.AlSaleh at gmail.com>
    https://github.com/rust-alt/term-string

    This Source Code Form is subject to the terms of the Mozilla Public
    License, v. 2.0. If a copy of the MPL was not distributed with this
    file, You can obtain one at <http://mozilla.org/MPL/2.0/>.
*/

use syn::Type;

pub(crate) fn bare_ty_str(ty: &Type) -> Result<String, String> {
    (quote! { #ty })
        .to_string()
        .split(char::is_whitespace)
        .nth(0)
        .map(|s| s.into())
        .ok_or_else(|| "impossible".into())
}
