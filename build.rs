use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::env;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use syn::{Expr, Lit, parse_quote};

#[derive(Debug)]
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

#[derive(Debug)]
struct EnumVariant {
    value: String,
    name: String,
}

#[derive(Debug)]
struct Enum8 {
    variants: Vec<EnumVariant>,
}

#[derive(Debug)]
struct Cluster {
    name: String,
    id: String,
    attributes: Vec<Attribute>,
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

fn parse_file(filename: &str) -> (Vec<Cluster>, Vec<Enum8>) {
    let file = File::open(filename).expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut clusters = Vec::new();
    let mut enum8s = Vec::new();

    let mut current_cluster: Option<Cluster> = None;
    let mut current_enum8: Option<Enum8> = None;

    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();

        if line.starts_with("#") || line.is_empty() {
            continue;
        } else if line.starts_with("enum8") {
            current_enum8 = Some(Enum8 {
                variants: Vec::new(),
            });
        } else if line.starts_with("cluster") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            current_cluster = Some(Cluster {
                name: parts[1].to_string(),
                id: parts[2].to_string(),
                attributes: Vec::new(),
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
                }
            }
        } else if current_enum8.is_some() {
            // TODO: insert
        } else if line == "}" {
            if let Some(cluster) = current_cluster.take() {
                clusters.push(cluster);
            }
            if let Some(enum8) = current_enum8.take() {
                enum8s.push(enum8);
            }
        }
    }

    (clusters, enum8s)
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
// let min: Expr = syn::parse_str(&attr.min).unwrap();
// let max: Expr = syn::parse_str(&attr.max).unwrap();

fn generate_attribute_code(attr: &Attribute) -> TokenStream {
    let id: Lit = syn::parse_str(&attr.id).unwrap();
    let name = &attr.name;
    let rust_type = &attr.rust_type;
    let default = match attr.default.as_str() {
        "-" => quote! { None },
        def => {
            let default: Expr = syn::parse_str(def).unwrap();
            quote! { Some(#rust_type(#default)) }
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

    quote! {
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

fn main() {
    let mut generated = TokenStream::new();
    let cluster_dir = std::fs::read_dir("clusters").expect("Failed to read clusters directory");

    for entry in cluster_dir {
        let entry = entry.expect("Failed to read dir entry");
        let path = entry.path();

        if path.extension().map(|ext| ext == "txt").unwrap_or(false) {
            let filename_stem = path.file_stem().unwrap().to_string_lossy();
            let mod_name = format_ident!("{}", filename_stem);
            let (clusters, enum8s) = parse_file(&path.to_string_lossy());

            let mut mod_content = TokenStream::new();
            for cluster in &clusters {
                for attr in &cluster.attributes {
                    mod_content.extend(generate_attribute_code(attr));
                }
                mod_content.extend(generate_cluster_struct(cluster));
            }

            for enum8 in &enum8s {
                for variant in &enum8.variants {
                    mod_content.extend(generate_enum8_variant_code(variant));
                }
                mod_content.extend(generate_enum8(enum8));
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

    let tokens = parse_quote! {
        #generated
    };

    // Write to generated.rs
    // let out_path = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not set"));
    // fs::write(
    //     out_path.join("generated.rs"),
    //     prettyplease::unparse(&tokens),
    // )
    // .unwrap();
    let out_path = PathBuf::from("/tmp/a.rs");
    fs::write(&out_path, prettyplease::unparse(&tokens)).unwrap();
    panic!("aa");

    // Watch for changes
    println!("cargo:rerun-if-changed=clusters/");
}
