use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use syn::{Expr, Lit, LitInt, Type, parse_quote};

struct Attribute {
    id: String,
    name: String,
    spec_type: String,
    rust_type: TokenStream,
    rust_type_doc: String,
    min: String,
    max: String,
    access: String,
    default: String,
    mandatory: String,
}

struct EnumVariant {
    value: String,
    name: String,
}

struct Enum {
    repr_type: Type,
    name: String,
    variants: Vec<EnumVariant>,
}

struct Cluster {
    name: String,
    id: String,
    attributes: Vec<Attribute>,
    enums: Vec<Enum>,
}

fn kind_to_type(ident: &str, kind: &str) -> (TokenStream, String) {
    macro_rules! simple {
        ($x:tt) => {
            (
                quote! { $x },
                concat!("[`", stringify!($x), "`]").to_string(),
            )
        };
    }
    match kind {
        "nodata" => simple! { NoData },
        "data8" => simple! { Data8 },
        "data16" => simple! { Data16 },
        "data24" => simple! { Data24 },
        "data32" => simple! { Data32 },
        "data40" => simple! { Data40 },
        "data48" => simple! { Data48 },
        "data56" => simple! { Data56 },
        "data64" => simple! { Data64 },
        "bool" => simple! { Bool },
        "map8" => simple! { Bitmap8 },
        "map16" => simple! { Bitmap16 },
        "map24" => simple! { Bitmap24 },
        "map32" => simple! { Bitmap32 },
        "map40" => simple! { Bitmap40 },
        "map48" => simple! { Bitmap48 },
        "map56" => simple! { Bitmap56 },
        "map64" => simple! { Bitmap64 },
        "uint8" => simple! { U8 },
        "uint16" => simple! { U16 },
        "uint24" => simple! { U24 },
        "uint32" => simple! { U32 },
        "uint40" => simple! { U40 },
        "uint48" => simple! { U48 },
        "uint56" => simple! { U56 },
        "uint64" => simple! { U64 },
        "int8" => simple! { I8 },
        "int16" => simple! { I16 },
        "int24" => simple! { I24 },
        "int32" => simple! { I32 },
        "int40" => simple! { I40 },
        "int48" => simple! { I48 },
        "int56" => simple! { I56 },
        "int64" => simple! { I64 },
        x if x.starts_with("enum8:") => {
            let chosen_enum = format_ident!("{}", x.strip_prefix("enum8:").unwrap());
            (
                quote! { Enum8::<#chosen_enum> },
                format!("[`Enum8`]::<[`{chosen_enum}`]>"),
            )
        }
        x if x.starts_with("enum16:") => {
            let chosen_enum = format_ident!("{}", x.strip_prefix("enum16:").unwrap());
            (
                quote! { Enum16::<#chosen_enum> },
                format!("[`Enum16`]::<[`{chosen_enum}`]>"),
            )
        }
        "enum8" => {
            let ident = format_ident!("{ident}");
            (
                quote! { Enum8::<#ident> },
                format!("[`Enum8`]::<[`{ident}`]>"),
            )
        }
        "enum16" => {
            let ident = format_ident!("{ident}");
            (
                quote! { Enum16::<#ident> },
                format!("[`Enum16`]::<[`{ident}`]>"),
            )
        }
        // "semi"      => quote! {  }}
        "single" => simple! { F32 },
        "double" => simple! { F64 },
        "octstr" => (
            quote! { OctetString::<'static> },
            "[`OctetString`]".to_string(),
        ),
        "string" => (
            quote! { CharacterString::<'static> },
            "[`CharacterString`]".to_string(),
        ),
        "octstr16" => (
            quote! { LongOctetString::<'static> },
            "[`LongOctetString`]".to_string(),
        ),
        "string16" => (
            quote! { LongCharacterString::<'static> },
            "[`LongCharacterString`]".to_string(),
        ),
        // "ASCII"     => quote! {  }}
        // "array"     => quote! { Array }}
        // "struct"    => quote! { Structure }}
        // "set"       => quote! {  }}
        // "bag"       => quote! {  }}
        "ToD" => simple! { TimeOfDay },
        "date" => simple! { Date },
        "UTC" => simple! { UtcTime },
        "clusterId" => simple! { ClusterId },
        "attribId" => simple! { AttributeId },
        "bacOID" => simple! { BacnetOid },
        "EUI64" => simple! { IeeeAddress },
        "key128" => simple! { SecurityKey },
        "unk" => simple! { Unknown },
        other => (quote! { #other }, other.to_string()),
    }
}

fn kind_to_cast(kind: &str) -> TokenStream {
    match kind {
        "int8" => quote! { as u8  as i8 },
        "int16" => quote! { as u16 as i16 },
        "int24" => quote! { as u24 as i24 },
        "int32" => quote! { as u32 as i32 },
        "int40" => quote! { as u40 as i40 },
        "int48" => quote! { as u48 as i48 },
        "int56" => quote! { as u56 as i56 },
        "int64" => quote! { as u64 as i64 },
        _ => quote! {},
    }
}

fn parse_file(filename: &str) -> (Vec<Attribute>, Vec<Cluster>, Vec<Enum>) {
    let file = File::open(filename).expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut global_attributes = Vec::new();
    let mut clusters = Vec::new();
    let mut global_enums = Vec::new();

    let mut current_cluster: Option<Cluster> = None;
    let mut current_enum: Option<Enum> = None;

    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();

        if line.starts_with("#") || line.is_empty() {
            continue;
        } else if line.starts_with("enum") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            current_enum = Some(Enum {
                repr_type: syn::parse_str(&format!("u{}", parts[0].strip_prefix("enum").unwrap()))
                    .unwrap(),
                name: parts[1].to_string(),
                variants: Vec::new(),
            });
        } else if line.starts_with("cluster") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            current_cluster = Some(Cluster {
                name: parts[1].to_string(),
                id: parts[2].to_string(),
                attributes: Vec::new(),
                enums: Vec::new(),
            });
        } else if line.starts_with("attr") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 9 {
                let spec_type = parts[3].to_string();
                let (rust_type, rust_type_doc) = kind_to_type(parts[2], &spec_type);
                let attr = Attribute {
                    id: parts[1].to_string(),
                    name: parts[2].to_string(),
                    spec_type,
                    rust_type,
                    rust_type_doc,
                    min: parts[4].to_string(),
                    max: parts[5].to_string(),
                    access: parts[6].to_string(),
                    default: parts[7].to_string(),
                    mandatory: parts[8].to_string(),
                };
                if let Some(cluster) = current_cluster.as_mut() {
                    cluster.attributes.push(attr);
                } else {
                    global_attributes.push(attr);
                }
            }
        } else if line == "}" {
            if let Some(en) = current_enum.take() {
                if let Some(cluster) = current_cluster.as_mut() {
                    cluster.enums.push(en);
                } else {
                    global_enums.push(en);
                }
            } else if let Some(cluster) = current_cluster.take() {
                clusters.push(cluster);
            }
        } else if current_enum.is_some() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if let Some(en) = current_enum.as_mut() {
                en.variants.push(EnumVariant {
                    value: parts[0].to_string(),
                    name: parts[1].to_string(),
                });
            }
        }
    }

    (global_attributes, clusters, global_enums)
}

fn parse_bound(attr: &Attribute, bound: &str, cluster: Option<&Cluster>) -> TokenStream {
    match (attr.spec_type.as_str(), bound) {
        (_, "-") => quote! { AttributeRange::Ignore },
        ("enum8" | "enum16", _) => quote! { AttributeRange::Ignore },
        ("octstr" | "string" | "octstr16" | "string16", v) => {
            let v: Expr = syn::parse_str(v).unwrap();
            quote! { AttributeRange::Size(#v) }
        }
        (_, v) => {
            if let Ok(v) = syn::parse_str::<LitInt>(v) {
                let cast = kind_to_cast(&attr.spec_type);
                let rust_type = &attr.rust_type;
                quote! { AttributeRange::Value(#rust_type(#v #cast)) }
            } else if let Some(cluster) = cluster {
                if let Some(attr) = cluster.attributes.iter().find(|x| x.name == v) {
                    let id: Lit = syn::parse_str(&attr.id).unwrap();
                    quote! { AttributeRange::Attribute(#id) }
                } else {
                    panic!("Failed finding attribute");
                }
            } else {
                panic!("Failed parsing bound");
            }
        }
    }
}

fn generate_attribute_code(attr: &Attribute, cluster: Option<&Cluster>) -> TokenStream {
    let id: Lit = syn::parse_str(&attr.id).unwrap();
    let name = &attr.name;
    let rust_type = &attr.rust_type;
    let default = match attr.default.as_str() {
        "-" => quote! { None },
        "non" => {
            quote! { Some(#rust_type(#rust_type::NON_VALUE.unwrap())) }
        }
        def => {
            let default: Expr = syn::parse_str(def).unwrap();
            match attr.spec_type.as_str() {
                x if x.starts_with("enum8:") => {
                    let chosen_enum = format_ident!("{}", x.strip_prefix("enum8:").unwrap());
                    quote! { Some(#rust_type(#chosen_enum::from_value(#default))) }
                }
                x if x.starts_with("enum16:") => {
                    let chosen_enum = format_ident!("{}", x.strip_prefix("enum16:").unwrap());
                    quote! { Some(#rust_type(#chosen_enum::from_value(#default))) }
                }
                "enum8" | "enum16" => {
                    let ident = format_ident!("{}", attr.name);
                    quote! { Some(#rust_type(#ident::from_value(#default))) }
                }
                "octstr" | "string" | "octstr16" | "string16" | "bool" => {
                    quote! { Some(#rust_type(Some(#default))) }
                }
                _ => quote! { Some(#rust_type(#default)) },
            }
        }
    };
    let min = parse_bound(attr, &attr.min, cluster);
    let max = parse_bound(attr, &attr.max, cluster);
    let mandatory = attr.mandatory == "M";

    // Parse access flags
    let readable = attr.access.contains('R');
    let writable = attr.access.contains('W');
    let reportable = attr.access.contains('P');
    let scene = attr.access.contains('S');

    let name_ident = format_ident!("{}", name.to_case(Case::UpperSnake));

    let attr_def = quote! {
        pub const #name_ident: Attribute<'static, #rust_type> = Attribute {
            code: #id,
            name: #name,
            side: AttributeSide::Server,
            writable: #writable,
            readable: #readable,
            reportable: #reportable,
            scene: #scene,
            mandatory: #mandatory,
            default: #default,
            min: #min,
            max: #max,
        };
    };

    let attr_def_str =
        prettyplease::unparse(&syn::parse_file(attr_def.to_string().as_str()).unwrap());
    quote! {
        #[doc = "```rust"]
        #[doc = #attr_def_str]
        #[doc = "```"]
        #attr_def
    }
}

fn generate_cluster(cluster: &Cluster) -> TokenStream {
    let name = &cluster.name;
    let cluster_name = format_ident!("{}_CLUSTER", cluster.name.to_case(Case::UpperSnake));
    let id: Lit = syn::parse_str(&cluster.id).unwrap();
    let mod_name = format_ident!("{}", cluster.name.to_case(Case::Snake));

    let fields = cluster.attributes.iter().map(|attr| {
        let field_name = format_ident!("{}", attr.name.to_case(Case::Snake));
        let val_name = format_ident!("{}", attr.name.to_case(Case::UpperSnake));
        quote! {
            #field_name: self::#mod_name::#val_name,
        }
    });
    let struct_name = format_ident!("{}Attrs", cluster.name.to_case(Case::UpperCamel));
    let cluster_def = quote! {
        pub const #cluster_name:crate::Cluster<'static,self::#mod_name::#struct_name> = crate::Cluster {
            code: #id,
            name: #name,
            meta: self::#mod_name::#struct_name {
                #(#fields)*
            },
        };
    };
    let cluster_def_str =
        prettyplease::unparse(&syn::parse_file(cluster_def.to_string().as_str()).unwrap());
    quote! {
        #[doc = "```rust"]
        #[doc = #cluster_def_str]
        #[doc = "```"]
        #cluster_def
    }
}

fn generate_cluster_struct(cluster: &Cluster) -> TokenStream {
    let struct_name = format_ident!("{}Attrs", cluster.name.to_case(Case::UpperCamel));
    let fields = cluster.attributes.iter().map(|attr| {
        let field_name = format_ident!("{}", attr.name.to_case(Case::Snake));
        let ty = &attr.rust_type;
        quote! {
            pub #field_name: Attribute<'static, #ty>,
        }
    });

    let attrs_array = cluster.attributes.iter().map(|attr| {
        let name = &attr.name;
        let ty_str = attr.rust_type.to_string();
        quote! {
            (#name, #ty_str)
        }
    });

    let n_attrs = cluster.attributes.len();
    quote! {
        pub struct #struct_name {
            #(#fields)*
        }

        impl #struct_name {
            pub fn attrs(&self) -> [(&'static str, &'static str); #n_attrs] {
                [
                    #(#attrs_array),*
                ]
            }
        }
    }
}

fn generate_enum8(enum8: &Enum) -> TokenStream {
    let ident = format_ident!("{}", enum8.name.to_case(Case::UpperCamel));
    let repr_type = &enum8.repr_type;
    let variants = enum8
        .variants
        .iter()
        .map(|EnumVariant { value, name }| {
            let name = format_ident!("{}", name.to_case(Case::UpperCamel));
            let value: Expr = syn::parse_str(value).unwrap();
            quote! {
                #name = #value,
            }
        })
        .collect::<Vec<_>>();

    let from_value_arms = enum8
        .variants
        .iter()
        .map(|EnumVariant { value, name }| {
            let name = format_ident!("{}", name.to_case(Case::UpperCamel));
            let value: Expr = syn::parse_str(value).unwrap();
            quote! {
                #value => Ok(Self::#name),
            }
        })
        .collect::<Vec<_>>();

    quote! {
        #[repr(#repr_type)]
        #[derive(PartialEq, Debug, Copy, Clone)]
        pub enum #ident {
            #(#variants)*
            None = #repr_type::MAX,
        }

        impl crate::types::ZclEnum for #ident {
            const NON_VALUE: Self = Self::None;
        }

        impl #ident {
            pub const fn try_from_value(value: #repr_type) -> Result<Self, ()> {
                match value {
                    #(#from_value_arms)*
                    #repr_type::MAX => Ok(Self::None),
                    _ => Err(())
                }
            }

            pub const fn from_value(value: #repr_type) -> Self {
                match Self::try_from_value(value) {
                    Ok(x) => x,
                    Err(_) => panic!("Failed to convert value to enum"),
                }
            }
        }
    }
}

fn main() {
    let mut generated = TokenStream::new();
    let cluster_dir = std::fs::read_dir("clusters").expect("Failed to read clusters directory");

    for entry in cluster_dir {
        let entry = entry.expect("Failed to read dir entry");
        let path = entry.path();

        if path.extension().map(|ext| ext == "txt").unwrap_or(false) {
            let filename_stem = path.file_stem().unwrap().to_string_lossy();
            let mod_name = format_ident!("{}", filename_stem);
            println!("cargo:rerun-if-changed={}", path.to_str().unwrap());
            let (global_attributes, clusters, enum8s) = parse_file(&path.to_string_lossy());

            let mut mod_content = TokenStream::new();
            for enum8 in &enum8s {
                mod_content.extend(generate_enum8(enum8));
            }

            for attr in &global_attributes {
                mod_content.extend(generate_attribute_code(attr, None));
            }

            for cluster in &clusters {
                let mut inner_mod_content = TokenStream::new();
                let mod_name = format_ident!("{}", cluster.name.to_case(Case::Snake));
                let mut attr_table = "".to_string();
                attr_table += "| id | name | type | min | max | access | default | mandatory |\n";
                attr_table += "|----|------|------|-----|-----|--------|---------|-----------|\n";

                for enu in &cluster.enums {
                    inner_mod_content.extend(generate_enum8(enu));
                }
                for attr in &cluster.attributes {
                    inner_mod_content.extend(generate_attribute_code(attr, Some(cluster)));
                    attr_table += &format!(
                        "| {} | [{}]({mod_name}::{}) | {} | {} | {} | {} | {} | {} |\n",
                        attr.id,
                        attr.name,
                        attr.name.to_case(Case::UpperSnake),
                        attr.rust_type_doc,
                        attr.min,
                        attr.max,
                        attr.access,
                        attr.default,
                        if attr.mandatory == "M" { "✅" } else { "❌" },
                    );
                }
                inner_mod_content.extend(generate_cluster_struct(cluster));

                mod_content.extend(quote! {
                    #[doc = "Holds types and constants related to the {} cluster."]
                    #[doc = ""]
                    #[doc = "Attribute list:"]
                    #[doc = ""]
                    #[doc = #attr_table]
                    pub mod #mod_name {
                        use crate::types::*;
                        use super::*;
                        #inner_mod_content
                    }
                });
                mod_content.extend(generate_cluster(cluster));
            }

            let wrapped_mod = quote! {
                pub mod #mod_name {
                    use crate::types::*;
                    #mod_content
                }
            };

            generated.extend(wrapped_mod);
        }
    }

    // let out_path = PathBuf::from("/tmp/a.rs");
    // fs::write(&out_path, generated.to_string()).unwrap();
    // panic!("aa");

    let tokens = parse_quote! {
        #generated
    };

    // Write to generated.rs
    let out_path = PathBuf::from(std::env::var("OUT_DIR").expect("OUT_DIR not set"));
    fs::write(
        out_path.join("generated.rs"),
        prettyplease::unparse(&tokens),
    )
    .unwrap();
    // let out_path = PathBuf::from("/tmp/a.rs");
    // fs::write(&out_path, prettyplease::unparse(&tokens)).unwrap();
}
