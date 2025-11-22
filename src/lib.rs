//! Util macro to attach read-only metadata to structs in the parent module.

/// The main struct can be defined in whatever way desired.
/// But all fields in each metadata struct will always be public.
///
/// This macro rule receives a main struct and multiple metadata structs to recursively copy-paste
/// the same field names for each of the structs in the macro below.
#[macro_export]
macro_rules! metadata {
    ($main_struct:tt, {
        $(
            $(#[$metadata_attrs:meta])*
            $metadata_vis:vis
            struct $metadata_struct:ident: $metadata_type:ty;
        )+
    }) => {
        $(
            use put_struct;
            put_struct!($main_struct);
            metadata_single!(
                $main_struct,
                $(#[$metadata_attrs])* $metadata_vis struct $metadata_struct: $metadata_type
            );
        )+
    };
}

/// The main struct can be defined in whatever way desired.
/// But all fields in the metadata struct will always be public.
///
/// This macro receives a main struct and a metadata struct. The metadata struct will takeover all
/// fields in the main struct but replace their types to the given one:
#[macro_export]
macro_rules! metadata_single {
    (
        {
            $(#[$attrs:meta])*
            $vis:vis
            struct $name:ident {
                $(
                    $(#[$field_attrs:meta])*
                    $field_vis:vis
                    $field:ident: $type:ty
                ),*
                $(,)?
            }
        },
        $(#[$metadata_attrs:meta])*
        $metadata_vis:vis
        struct $metadata_struct:ident: $metadata_type:ty
    ) => {
        $(#[$metadata_attrs])*
        $metadata_vis
        struct $metadata_struct {
            $(
                $(#[$field_attrs])*
                $field_vis
                $field: $metadata_type
            )*
        }
    };
}

/// This rule is almost no-op except maintaining the macro hygigene of Rust.
/// See: https://stackoverflow.com/a/75530574/13787084
/// It receives the main struct and pastes it:
#[allow(unused_macros)]
macro_rules! put_struct {
    ({
        $(#[$attrs:meta])*
        $vis:vis
        struct $name:ident {
            $(
                $(#[$field_attrs:meta])*
                $field_vis:vis
                $field:ident: $type:ty
            ),*
            $(,)?
        }
    }) => {
        $(#[$attrs])*
        $vis
        struct $name {
            $(
                $(#[$field_attrs])*
                $field_vis
                $field: $type
            )*
        }
    };
}
