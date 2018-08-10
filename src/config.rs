use syn::{Attribute, Lit, Meta, NestedMeta};

#[derive(Default)]
pub(crate) struct AttrInfo {
    pub(crate) names: Vec<String>,
    pub(crate) name_vals: Vec<(String, String)>,
}

pub(crate) struct MacroConfig {
    pub(crate) inblock: bool,
    pub(crate) non_public: bool,
    pub(crate) prefix: String,
    pub(crate) impl_doc: String,
    pub(crate) doc: String,
}

#[derive(Default)]
pub(crate) struct MethodConfig {
    pub(crate) inblock: bool,
    pub(crate) non_public: bool,
    pub(crate) skip: bool,
    pub(crate) prefix: Option<String>,
    pub(crate) rename: Option<String>,
    pub(crate) name: Option<String>,
    pub(crate) doc: Option<String>,
}

impl Default for MacroConfig {
    fn default() -> Self {
        Self {
            inblock: false,
            non_public: false,
            prefix: "with_".into(),
            doc: "The chaining (fluent) equivalent of [`%f%()`].".into(),
            impl_doc: "Chaining (fluent) methods for [`%t%`].".into(),
        }
    }
}

pub(crate) fn parse_config_from_attr(attr: &Attribute) -> Result<AttrInfo, String> {
    let mut attr_info = AttrInfo::default();

    if let Some(meta) = attr.interpret_meta() {
        match meta {
            Meta::List(meta_list) => {
                for nm in &meta_list.nested {
                    match nm {
                        NestedMeta::Meta(m) => match m {
                            Meta::Word(ident) => attr_info.names.push(ident.to_string()),
                            Meta::NameValue(name_value) => match name_value.lit {
                                Lit::Str(ref ls) => attr_info.name_vals.push((name_value.ident.to_string(), ls.value())),
                                _ => Err(format!(
                                    "expected a string literal value in a name_vlue pair, found: {:?}",
                                    name_value.lit
                                ))?,
                            },
                            Meta::List(l) => Err(format!("expected a meta word or name=value, found meta list: {:?}", l))?,
                        },
                        NestedMeta::Literal(l) => Err(format!("expected a meta word or name=value, found literal: {:?}", l))?,
                    }
                }
            },
            _ => Err("expected #[fluent_impl(...)] format")?,
        }
    } else {
        Err("couldn't parse meta items, make sure all items are either name or name=value where value is a string literal")?;
    }

    Ok(attr_info)
}

macro_rules! err_if_set {
    ($ty:ident, $var:ident, $field:ident, $val:expr) => {
        if $var.$field != $ty::default().$field {
            Err(format!("{} is already set", stringify!($field)))?;
        } else {
            $var.$field = $val;
        }
    };
}

pub(crate) fn get_proc_macro_config(attr_info: AttrInfo) -> Result<MacroConfig, String> {
    let mut config = MacroConfig::default();

    for name in attr_info.names {
        match &*name {
            "inblock" => err_if_set!(MacroConfig, config, inblock, true),
            "non_public" => err_if_set!(MacroConfig, config, non_public, true),
            _ => Err(format!("invalid attribute word: {}", name))?,
        }
    }

    for (name, val) in attr_info.name_vals {
        match (&*name, val) {
            ("prefix", val) => err_if_set!(MacroConfig, config, prefix, val),
            ("impl_doc", val) => err_if_set!(MacroConfig, config, impl_doc, val),
            ("doc", val) => err_if_set!(MacroConfig, config, doc, val),
            _ => Err(format!("invalid name in a name_value pair: {}", name))?,
        }
    }

    if config.prefix.is_empty() {
        Err("invalid empty prefix attribute")?;
    }

    Ok(config)
}

pub(crate) fn get_method_config(attr_info: AttrInfo, pre_config: Option<MethodConfig>) -> Result<MethodConfig, String> {
    let mut config = match pre_config {
        Some(pre_config) => pre_config,
        None => MethodConfig::default(),
    };

    for name in attr_info.names {
        match &*name {
            "inblock" => err_if_set!(MethodConfig, config, inblock, true),
            "non_public" => err_if_set!(MethodConfig, config, non_public, true),
            "skip" => err_if_set!(MethodConfig, config, skip, true),
            _ => Err(format!("invalid attribute word: {}", name))?,
        }
    }

    for (name, val) in attr_info.name_vals {
        match (&*name, val) {
            ("prefix", val) => err_if_set!(MethodConfig, config, prefix, Some(val)),
            ("name", val) => err_if_set!(MethodConfig, config, name, Some(val)),
            ("rename", val) => err_if_set!(MethodConfig, config, rename, Some(val)),
            ("doc", val) => err_if_set!(MethodConfig, config, doc, Some(val)),
            _ => Err(format!("invalid name in a name_value pair: {}", name))?,
        }
    }
    if config.name.is_some() {
        match (&config.rename, &config.prefix) {
            (None, None) => (),
            _ => Err("rename and/or prefix attributes can't be set if name is set")?,
        }
    }

    if let Some(name) = &config.name {
        if name.is_empty() {
            Err("invalid empty name attribute")?;
        }
    }

    if let Some(rename) = &config.rename {
        if rename.is_empty() {
            Err("invalid empty rename attribute")?;
        }
    }

    if let Some(prefix) = &config.prefix {
        if prefix.is_empty() {
            Err("invalid empty prefix attribute")?;
        }
    }

    Ok(config)
}
