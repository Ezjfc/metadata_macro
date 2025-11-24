//! #[metadata] attribute for both structs and fields.

use syn::{parse::{Parse, ParseStream}, punctuated::Punctuated, token::{Colon, Eq, Pub, Struct}, Attribute, Ident, Token, Type, Visibility};

pub fn proc_struct_level_attribute(attr: &Attribute) -> Result<Vec<MetadataStruct>, syn::Error> {
    let parser = <ArgsList<MetadataStruct>>::parse_terminated;
    Ok(attr.parse_args_with(parser)?.into_iter().collect())
}

type ArgsList<T> = Punctuated::<T, Token![,]>;

#[derive(Debug)]
pub struct MetadataStruct {
    pub prefix: StructLevelAttributeArgPrefix,
    pub name: Ident,
    pub metadata_type: Option<MetadataType>,
    pub alias: Option<MetadataAlias>,
}

#[derive(Debug)]
enum StructLevelAttributeArgPrefix {
    NoPrefix,
    StructKeyword(Struct),
    Visibility(Visibility, Struct),
}

#[derive(Debug)]
pub struct MetadataType {
    pub colon: Colon,
    pub metadata_type: Type,
}

#[derive(Debug)]
pub struct MetadataAlias {
    pub eq: Eq,
    pub alias: Ident,
}

impl Parse for MetadataStruct {
    fn parse(input: ParseStream<'_>) -> Result<Self, syn::Error> {
        Ok(Self {
            prefix: {
                use StructLevelAttributeArgPrefix as Prefix;
                if input.peek(Pub) {
                    Prefix::Visibility(input.parse()?, input.parse()?)
                } else if input.peek(Struct) {
                    Prefix::StructKeyword(input.parse()?)
                } else {
                    Prefix::NoPrefix
                }
            },
            name: input.parse()?,
            metadata_type: {
                if input.peek(Token![:]) {
                    Some(input.parse()?)
                } else {
                    None
                }
            },
            alias: {
                if input.peek(Token![=]) {
                    Some(input.parse()?)
                } else {
                    None
                }
            },
        })
    }
}

impl Parse for MetadataType {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        Ok(Self {
            colon: input.parse()?,
            metadata_type: input.parse()?,
        })
    }
}

impl Parse for MetadataAlias {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        Ok(Self {
            eq: input.parse()?,
            alias: input.parse()?,
        })
    }
}
