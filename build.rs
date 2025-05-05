use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use syn::{Expr, Lit, Type, parse_quote};

struct Attribute {
    id: String,
    name: String,
    spec_type: String,
    rust_type: TokenStream,
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

fn kind_to_type(ident: &str, kind: &str) -> TokenStream {
    match kind {
        "nodata" => quote! { NoData },
        "data8" => quote! { Data8 },
        "data16" => quote! { Data16 },
        "data24" => quote! { Data24 },
        "data32" => quote! { Data32 },
        "data40" => quote! { Data40 },
        "data48" => quote! { Data48 },
        "data56" => quote! { Data56 },
        "data64" => quote! { Data64 },
        "bool" => quote! { Bool },
        "map8" => quote! { Bitmap8 },
        "map16" => quote! { Bitmap16 },
        "map24" => quote! { Bitmap24 },
        "map32" => quote! { Bitmap32 },
        "map40" => quote! { Bitmap40 },
        "map48" => quote! { Bitmap48 },
        "map56" => quote! { Bitmap56 },
        "map64" => quote! { Bitmap64 },
        "uint8" => quote! { U8 },
        "uint16" => quote! { U16 },
        "uint24" => quote! { U24 },
        "uint32" => quote! { U32 },
        "uint40" => quote! { U40 },
        "uint48" => quote! { U48 },
        "uint56" => quote! { U56 },
        "uint64" => quote! { U64 },
        "int8" => quote! { I8 },
        "int16" => quote! { I16 },
        "int24" => quote! { I24 },
        "int32" => quote! { I32 },
        "int40" => quote! { I40 },
        "int48" => quote! { I48 },
        "int56" => quote! { I56 },
        "int64" => quote! { I64 },
        "enum8" => {
            let ident = format_ident!("{ident}");
            quote! { Enum8::<#ident> }
        }
        "enum16" => {
            let ident = format_ident!("{ident}");
            quote! { Enum16::<#ident> }
        }
        // "semi"      => quote! {  }}
        "single" => quote! { F32 },
        "double" => quote! { F64 },
        "octstr" => quote! { OctetString::<'static> },
        "string" => quote! { CharacterString::<'static> },
        "octstr16" => quote! { LongOctetString::<'static> },
        "string16" => quote! { LongCharacterString::<'static> },
        // "ASCII"     => quote! {  }}
        // "array"     => quote! { Array }}
        // "struct"    => quote! { Structure }}
        // "set"       => quote! {  }}
        // "bag"       => quote! {  }}
        "ToD" => quote! { TimeOfDay },
        "date" => quote! { Date },
        "UTC" => quote! { UtcTime },
        "clusterId" => quote! { ClusterId },
        "attribId" => quote! { AttributeId },
        "bacOID" => quote! { BacnetOid },
        "EUI64" => quote! { IeeeAddress },
        "key128" => quote! { SecurityKey },
        "unk" => quote! { Unknown },
        other => quote! { #other },
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
                let rust_type = kind_to_type(parts[2], &spec_type);
                let attr = Attribute {
                    id: parts[1].to_string(),
                    name: parts[2].to_string(),
                    spec_type,
                    rust_type,
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

fn parse_bound(attr: &Attribute, bound: &str) -> TokenStream {
    match (attr.spec_type.as_str(), bound) {
        (_, "-") => quote! { AttributeRange::Ignore },
        ("enum8" | "enum16", _) => quote! { AttributeRange::Ignore },
        ("octstr" | "string" | "octstr16" | "string16", v) => {
            let v: Expr = syn::parse_str(v).unwrap();
            quote! { AttributeRange::Size(#v) }
        }
        (_, v) => {
            let rust_type = &attr.rust_type;
            let v: Expr = syn::parse_str(v).unwrap();
            quote! { AttributeRange::Value(#rust_type(#v)) }
        }
    }
}

fn generate_attribute_code(attr: &Attribute) -> TokenStream {
    let id: Lit = syn::parse_str(&attr.id).unwrap();
    let name = &attr.name;
    let rust_type = &attr.rust_type;
    let default = match attr.default.as_str() {
        "-" => quote! { None },
        "None" => {
            quote! { Some(#rust_type(None)) }
        }
        def => {
            let default: Expr = syn::parse_str(def).unwrap();
            match attr.spec_type.as_str() {
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
    let min = parse_bound(attr, &attr.min);
    let max = parse_bound(attr, &attr.max);
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
                mod_content.extend(generate_attribute_code(attr));
            }

            for cluster in &clusters {
                let mut inner_mod_content = TokenStream::new();
                let mod_name = format_ident!("{}", cluster.name);
                for enu in &cluster.enums {
                    inner_mod_content.extend(generate_enum8(enu));
                }
                for attr in &cluster.attributes {
                    inner_mod_content.extend(generate_attribute_code(attr));
                }
                inner_mod_content.extend(generate_cluster_struct(cluster));
                mod_content.extend(quote! {
                    pub mod #mod_name {
                        use crate::types::*;
                        #inner_mod_content
                    }
                });
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
