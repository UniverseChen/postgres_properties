use proc_macro2::Ident;
use syn::{
    Meta::{List, NameValue},
    NestedMeta::Meta,
};

extern crate proc_macro;

use {
    self::proc_macro::TokenStream,
    proc_macro2,
    quote::*,
    syn::{parse_macro_input, DeriveInput},
};

/// postgres table's properties, with schema, name, owner, tablespace, partitioned_table and comments;
#[proc_macro_derive(Properties, attributes(general))]
pub fn derive(item: TokenStream) -> TokenStream {
    // parse token tree
    let input = parse_macro_input!(item as DeriveInput);

    let struct_name = &input.ident;

    let (schema, table, owner, tablespace, partitioned_table, comments) = parse_table_attr(&input);

    let implemented_show = quote! {
            impl PgTableProperties for #struct_name {
                fn schema() -> String {
                    #schema.to_string()
                }
                fn schema_sql() -> String {
                    format!(r#""{}""#, #schema)
                }
                fn table() -> String {
                    #table.to_string()
                }
                fn table_sql() -> String {
                    format!(r#""{}""#, #table)
                }
                fn owner() -> String{
                    #owner.to_string()
                }
                fn owner_sql() -> String {
                    format!(r#""{}""#, #owner)
                }
                fn tablespace() -> String {
                    #tablespace.to_string()
                }
                fn tablespace_sql() -> String {
                    format!(r#""{}""#, #tablespace)
                }
                fn partitioned_table() -> bool {
                    #partitioned_table
                }
                fn comments() -> String {
                    #comments.to_string()
                }
                fn schema_table_sql() -> String {
                    format!(r#""{}"."{}""#, #schema, #table)
                }
                fn schema_table_field_sql(field: &str) -> String {
                    format!(r#""{}"."{}"."{}""#, #schema, #table, field)
                }
            }
    };

    implemented_show.into()
}

fn get_lit_str<'a>(
    attr_name: Option<&Ident>,
    lit: &'a syn::Lit,
) -> ::std::result::Result<&'a syn::LitStr, ()> {
    if let syn::Lit::Str(ref lit) = *lit {
        Ok(lit)
    } else {
        if let Some(val) = attr_name {
            panic!("expected pg_mapper {:?} attribute to be a string", val);
        } else {
            panic!("expected pg_mapper attribute to be a string");
        }
        #[allow(unreachable_code)]
        Err(())
    }
}

fn get_mapper_meta_items(attr: &syn::Attribute) -> Option<Vec<syn::NestedMeta>> {
    if attr.path.segments.len() == 1 && attr.path.segments[0].ident == "general" {
        match attr.parse_meta() {
            Ok(List(ref meta)) => Some(meta.nested.iter().cloned().collect()),
            _ => {
                panic!("declare table name: #[pg_mapper(table = \"foo\")]");
            }
        }
    } else {
        None
    }
}

fn parse_table_attr(ast: &DeriveInput) -> (String, String, String, String, bool, String) {
    let mut schema = None;
    let mut table = None;
    let mut owner = None;
    let mut tablespace = None;
    let mut partitioned_table = None;
    let mut comments = None;

    for meta_items in ast.attrs.iter().filter_map(get_mapper_meta_items) {
        for meta_item in meta_items {
            match meta_item {
                Meta(NameValue(ref m)) if m.path.is_ident("schema") => {
                    if let Ok(s) = get_lit_str(m.path.get_ident(), &m.lit) {
                        schema = Some(s.value());
                    }
                }
                Meta(NameValue(ref m)) if m.path.is_ident("table") => {
                    if let Ok(s) = get_lit_str(m.path.get_ident(), &m.lit) {
                        table = Some(s.value());
                    }
                }
                Meta(NameValue(ref m)) if m.path.is_ident("owner") => {
                    if let Ok(s) = get_lit_str(m.path.get_ident(), &m.lit) {
                        owner = Some(s.value());
                    }
                }
                Meta(NameValue(ref m)) if m.path.is_ident("tablespace") => {
                    if let Ok(s) = get_lit_str(m.path.get_ident(), &m.lit) {
                        tablespace = Some(s.value());
                    }
                }
                Meta(NameValue(ref m)) if m.path.is_ident("partitioned_table") => {
                    if let Ok(s) = get_lit_str(m.path.get_ident(), &m.lit) {
                        if s.value() == "true".to_owned() {
                            partitioned_table = Some(true);
                        } else if s.value() == "false".to_owned() {
                            partitioned_table = Some(false);
                        } else {
                            panic!("declare general: #[general(partitioned_table= \"true\")] or #[general(partitioned_table= \"false\")]");
                        }
                    }
                }
                Meta(NameValue(ref m)) if m.path.is_ident("comments") => {
                    if let Ok(s) = get_lit_str(m.path.get_ident(), &m.lit) {
                        comments = Some(s.value());
                    }
                }
                Meta(_) => {
                    panic!("unknown general container attribute",)
                }
                _ => {
                    panic!("unexpected literal in general container attribute");
                }
            }
        }
    }

    (
        schema.expect("declare general: #[general(schema = \"schema_name\")]"),
        table.expect("declare general: #[general(table = \"table_name\")]"),
        owner.expect("declare general: #[general(owner = \"owner_name\")]"),
        tablespace.expect("declare general: #[general(tablespace = \"tablespace_name\")]"),
        partitioned_table.expect("declare general: #[general(partitioned_table = \"false\")]"),
        comments.expect("declare general: #[general(comments = \"comments\")]"),
    )
}
