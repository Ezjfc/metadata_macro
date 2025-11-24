use proc_macro::TokenStream;
use quote::quote;
use syn::{Attribute, DeriveInput, Error, Result, Type};

mod attr;

/// ```
/// use structfield_metadata_proc_macro::Metadata;
///
/// #[derive(Metadata)]
/// #[metadata(pub(crate) struct a = Thing)]
/// struct YourStruct {
///     #[metadata(c = "d")]
///     field_a: bool,
///     field_b: usize,
/// }
/// ```
#[proc_macro_derive(Metadata, attributes(metadata))]
pub fn metadata_derive(item: TokenStream) -> TokenStream {
    proc(item).unwrap_or_else(Error::into_compile_error).into()
}

fn proc(item: TokenStream) -> Result<proc_macro2::TokenStream> {
    let item = syn::parse::<DeriveInput>(item)?;
    let attrs = item.attrs.iter().filter(filter_attr);
    let structs: Vec<_> = attrs
        .flat_map(attr::proc_struct_level_attribute)
        .collect();
    dbg!(structs);

    todo!("b")
}

fn filter_attr(attr: &&Attribute) -> bool {
    attr.path().is_ident("metadata")
}

struct MetadataStruct {
    // Since the `#[metadata]` helper attribute will handle specific cases for [Option], it is
    // necessary to know the type instead:
}

/// Since the procedural macro is only responsible for implementing [core::Default] to the
/// generated metadata structs, this function wraps the main struct with [crate::metadata],
/// who handles the generation of metadata structs.
fn wrap_in_macro(main_struct: TokenStream, metadata_structs: Vec<MetadataStruct>) -> proc_macro2::TokenStream {
    todo!("c");
    // let output = quote! {
    //     use crate::metadata;
    //     metadata!({
    //     }, {
    //     });
    // };
    // output
}
