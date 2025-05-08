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
    range: String,
    access: String,
    default: String,
    mandatory: String,
}

#[derive(Clone)]
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

fn kind_to_cast(litv: &LitInt, kind: &str) -> TokenStream {
    let val: i128 = litv
        .base10_parse()
        .expect("Could not parse literal integer in bound");
    match kind {
        "int8" => {
            if val > i8::MAX as i128 {
                quote! { as u8 as i8 }
            } else {
                quote! {}
            }
        }
        "int16" => {
            if val > i16::MAX as i128 {
                quote! { as u16 as i16 }
            } else {
                quote! {}
            }
        }
        "int24" => {
            if val > ((1u128 << 23) - 1) as i128 {
                quote! { as u24 as i24 }
            } else {
                quote! {}
            }
        }
        "int32" => {
            if val > i32::MAX as i128 {
                quote! { as u32 as i32 }
            } else {
                quote! {}
            }
        }
        "int40" => {
            if val > ((1u128 << 39) - 1) as i128 {
                quote! { as u40 as i40 }
            } else {
                quote! {}
            }
        }
        "int48" => {
            if val > ((1u128 << 47) - 1) as i128 {
                quote! { as u48 as i48 }
            } else {
                quote! {}
            }
        }
        "int56" => {
            if val > ((1u128 << 55) - 1) as i128 {
                quote! { as u56 as i56 }
            } else {
                quote! {}
            }
        }
        "int64" => {
            if val > i64::MAX as i128 {
                quote! { as u64 as i64 }
            } else {
                quote! {}
            }
        }
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
            if parts.len() == 8 {
                let spec_type = parts[3].to_string();
                let (rust_type, rust_type_doc) = kind_to_type(parts[2], &spec_type);
                let range = match (parts[3], parts[4]) {
                    ("enum8", "0x00,0xff") => "full-non",
                    ("enum16", "0x0000,0xffff") => "full-non",
                    (_, x) => x,
                }
                .to_string();
                let attr = Attribute {
                    id: parts[1].to_string(),
                    name: parts[2].to_string(),
                    spec_type,
                    rust_type,
                    rust_type_doc,
                    range,
                    access: parts[5].to_string(),
                    default: parts[6].to_string(),
                    mandatory: parts[7].to_string(),
                };
                if let Some(cluster) = current_cluster.as_mut() {
                    cluster.attributes.push(attr);
                } else {
                    global_attributes.push(attr);
                }
            } else {
                println!("cargo:warning=definition '{}' should have 8 fields", line);
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

fn parse_bound(attr: &Attribute, cluster: Option<&Cluster>) -> TokenStream {
    match (attr.spec_type.as_str(), attr.range.as_str()) {
        // (_, "-") => quote! { AttributeRange::Ignore },
        (_, "value") => quote! { AttributeRange::Value },
        (_, "full-non" | "-") => quote! { AttributeRange::FullWithNone },
        (_, "full") => quote! { AttributeRange::Full },
        ("octstr" | "string" | "octstr16" | "string16", _) => {
            let s: Expr = syn::parse_str(&attr.range).unwrap();
            quote! { AttributeRange::Size(#s) }
        }
        _ => {
            let (min, max) = attr.range.split_once(',').unwrap_or_else(|| {
                panic!(
                    "expected min,max bound but found no ',' delimiter, attribute {}",
                    attr.name
                )
            });

            if let (Ok(lit_min), Ok(lit_max)) =
                (syn::parse_str::<LitInt>(min), syn::parse_str::<LitInt>(max))
            {
                let min_cast = kind_to_cast(&lit_min, &attr.spec_type);
                let max_cast = kind_to_cast(&lit_max, &attr.spec_type);
                let rust_type = &attr.rust_type;
                quote! { AttributeRange::InclusiveRange(#rust_type(#lit_min #min_cast), #rust_type(#lit_max #max_cast)) }
            } else if let Some(cluster) = cluster {
                if let (Some(min_attr), Some(max_attr)) = (
                    cluster.attributes.iter().find(|x| x.name == min),
                    cluster.attributes.iter().find(|x| x.name == max),
                ) {
                    let min_id: Lit = syn::parse_str(&min_attr.id).unwrap();
                    let max_id: Lit = syn::parse_str(&max_attr.id).unwrap();
                    quote! { AttributeRange::InclusiveRangeReference(#min_id, #max_id) }
                } else {
                    panic!("Failed to find attributes ({},{}) for bound", min, max);
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
    let range = parse_bound(attr, cluster);
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
            readable: #readable,
            writable: #writable,
            reportable: #reportable,
            scene: #scene,
            mandatory: #mandatory,
            default: #default,
            range: #range,
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

    let mut variants = enum8.variants.clone();
    if variants.iter().find(|x| x.value == "0xff").is_none() {
        variants.push(EnumVariant {
            value: "0xff".to_string(),
            name: "None".to_string(),
        })
    }

    let enum_variants = variants
        .iter()
        .map(|x| {
            let name = format_ident!("{}", x.name.to_case(Case::UpperCamel));
            let value: Expr = syn::parse_str(&x.value).unwrap();
            quote! {
                #name = #value,
            }
        })
        .collect::<Vec<_>>();

    let from_value_arms = variants
        .iter()
        .map(|x| {
            let name = format_ident!("{}", x.name.to_case(Case::UpperCamel));
            let value: Expr = syn::parse_str(&x.value).unwrap();
            quote! {
                #value => Ok(Self::#name),
            }
        })
        .collect::<Vec<_>>();

    let non_value = variants
        .iter()
        .find(|x| x.value == "0xff")
        .map(|x| {
            let name = format_ident!("{}", x.name);
            quote! { Self::#name }
        })
        .unwrap();

    quote! {
        #[repr(#repr_type)]
        #[derive(PartialEq, Debug, Copy, Clone)]
        pub enum #ident {
            #(#enum_variants)*
        }

        impl crate::types::ZclEnum for #ident {
            const NON_VALUE: Self = #non_value;
        }

        impl #ident {
            pub const fn try_from_value(value: #repr_type) -> Result<Self, ()> {
                match value {
                    #(#from_value_arms)*
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
                attr_table += "| id | name | type | range | access | default | mandatory |\n";
                attr_table += "|----|------|------|-------|--------|---------|-----------|\n";

                for enu in &cluster.enums {
                    inner_mod_content.extend(generate_enum8(enu));
                }
                for attr in &cluster.attributes {
                    inner_mod_content.extend(generate_attribute_code(attr, Some(cluster)));
                    attr_table += &format!(
                        "| {} | [{}]({mod_name}::{}) | {} | {} | {} | {} | {} |\n",
                        attr.id,
                        attr.name,
                        attr.name.to_case(Case::UpperSnake),
                        attr.rust_type_doc,
                        attr.range,
                        attr.access,
                        attr.default,
                        if attr.mandatory == "M" { "✅" } else { "❌" },
                    );
                }
                inner_mod_content.extend(generate_cluster_struct(cluster));

                let cluster_desc = format!(
                    "Holds types and constants related to the [`{}`](self::{}_CLUSTER) cluster.",
                    cluster.name,
                    cluster.name.to_case(Case::UpperSnake),
                );
                mod_content.extend(quote! {
                    #[doc = #cluster_desc]
                    #[doc = ""]
                    #[doc = "Attribute list:"]
                    #[doc = ""]
                    #[doc = #attr_table]
                    pub mod #mod_name {
                        #[allow(unused)]
                        use crate::types::*;
                        #[allow(unused)]
                        use super::*;
                        #inner_mod_content
                    }
                });
                mod_content.extend(generate_cluster(cluster));
            }

            let wrapped_mod = quote! {
                pub mod #mod_name {
                    #[allow(unused)]
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
    let out_path = PathBuf::from(std::env::var("OUT_DIR").expect("OUT_DIR not set"));
    fs::write(
        out_path.join("generated.rs"),
        prettyplease::unparse(&tokens),
    )
    .unwrap();
}
