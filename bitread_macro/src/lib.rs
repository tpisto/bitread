extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, parse_str, Data, DeriveInput, Expr, Field, Fields, Lit, Meta, NestedMeta,
};

#[proc_macro_derive(BitRead, attributes(bitrw))]
pub fn bit_read_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // Extract struct-level attributes
    let (struct_endian, struct_bit_order) = parse_struct_attributes(&input.attrs);

    let name = input.ident.clone();
    let fields = extract_fields_from_input(input).unwrap_or_default();

    let read_fields: Vec<_> = fields.iter().map(|f| generate_field_read_code(f)).collect();
    let field_idents: Vec<_> = fields.iter().map(|f| &f.ident).collect();

    let bit_order_match = match struct_bit_order.as_str() {
        "lsb" => quote! { bitvec::order::Lsb0 },
        _ => quote! { bitvec::order::Msb0 },
    };

    let gen = quote! {
        impl BitRead for #name {
            fn read_from(data: &[u8]) -> Result<Self, ReadError> {
                let bitvec_bits = data.view_bits::<#bit_order_match>();
                // println!("bitvec_bits: {:?}", bitvec_bits);
                let mut offset = 0;
                #( #read_fields )*
                Ok(#name { #( #field_idents: #field_idents ),* })
            }
        }
    };

    gen.into()
}

fn extract_fields_from_input(input: DeriveInput) -> Option<Vec<Field>> {
    if let Data::Struct(data_struct) = input.data {
        match data_struct.fields {
            Fields::Named(fields) => Some(fields.named.into_iter().collect()),
            Fields::Unnamed(fields) => Some(fields.unnamed.into_iter().collect()),
            Fields::Unit => None,
        }
    } else {
        None
    }
}

fn generate_field_read_code(field: &Field) -> proc_macro2::TokenStream {
    let field_name = &field.ident;
    let field_type = &field.ty;
    let (bits, map_fn_closure, skip, default_expr) = parse_attributes(&field.attrs);

    if skip {
        if let Some(default) = default_expr {
            return quote! {
                let #field_name: #field_type = #default;
            };
        } else {
            return quote! {
                let #field_name = Default::default();
            };
        }
    }

    let read_value = quote! {
        let #field_name = read_bits!(bitvec_bits, offset, #bits, #field_type);
        offset += #bits;
    };

    match map_fn_closure {
        Some(closure_expr) => {
            let input_type = extract_input_type_from_closure(&closure_expr);
            quote! {
                let #field_name = {
                    let intermediate_value: #input_type = read_bits!(bitvec_bits, offset, #bits, #input_type);
                    let transform_fn: fn(#input_type) -> #field_type = #closure_expr;
                    transform_fn(intermediate_value)
                };
                offset += #bits;
            }
        }
        None => read_value,
    }
}

fn parse_attributes(attrs: &[syn::Attribute]) -> (usize, Option<Expr>, bool, Option<Expr>) {
    let mut bits = 0;
    let mut map_fn_closure = None;
    let mut skip = false;
    let mut default_expr = None;

    for attr in attrs {
        if attr.path.is_ident("bitrw") {
            if let Ok(Meta::List(list)) = attr.parse_meta() {
                for nested in list.nested.iter() {
                    match nested {
                        NestedMeta::Meta(Meta::NameValue(name_value)) => {
                            if name_value.path.is_ident("bits") {
                                if let Lit::Int(lit_int) = &name_value.lit {
                                    bits = lit_int.base10_parse::<usize>().unwrap_or(0);
                                }
                            } else if name_value.path.is_ident("map") {
                                if let Lit::Str(lit_str) = &name_value.lit {
                                    map_fn_closure = parse_str::<Expr>(&lit_str.value()).ok();
                                }
                            } else if name_value.path.is_ident("default") {
                                if let Lit::Str(lit_str) = &name_value.lit {
                                    default_expr = parse_str::<Expr>(&lit_str.value()).ok();
                                }
                            }
                        }
                        NestedMeta::Meta(Meta::Path(path)) => {
                            if path.is_ident("skip") {
                                skip = true;
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    (bits, map_fn_closure, skip, default_expr)
}

fn extract_input_type_from_closure(closure_expr: &Expr) -> Box<syn::Type> {
    if let Expr::Closure(expr_closure) = closure_expr {
        if let Some(pat) = expr_closure.inputs.iter().next() {
            if let syn::Pat::Type(pat_type) = pat {
                return pat_type.ty.clone();
            }
        }
    }
    Box::new(parse_str::<syn::Type>("()").unwrap()) // Default to unit type if not found
}

fn parse_struct_attributes(attrs: &[syn::Attribute]) -> (String, String) {
    let mut endian = String::from("little");
    let mut bit_order = String::from("lsb");

    for attr in attrs {
        if attr.path.is_ident("bitrw") {
            if let Ok(Meta::List(list)) = attr.parse_meta() {
                for nested in list.nested.iter() {
                    match nested {
                        NestedMeta::Meta(Meta::NameValue(name_value)) => {
                            if name_value.path.is_ident("endian") {
                                if let Lit::Str(lit_str) = &name_value.lit {
                                    endian = lit_str.value();
                                }
                            }
                            if name_value.path.is_ident("bit_order") {
                                if let Lit::Str(lit_str) = &name_value.lit {
                                    bit_order = lit_str.value();
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    (endian, bit_order)
}
