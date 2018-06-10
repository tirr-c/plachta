macro_rules! make_variant_slice {
    (
        $(#[ $attr:meta ])*
        pub enum $enum_name:ident [$slice_name:ident, $name_slice_name:ident] {
            $($variant:ident,)*
        }
    ) => {
        $(#[ $attr ])*
        pub enum $enum_name {
            $($variant,)*
        }

        pub const $slice_name: &'static [$enum_name] = &[
            $($enum_name::$variant,)*
        ];

        pub const $name_slice_name: &'static [&'static str] = &[
            $(stringify!($variant),)*
        ];
    };
}
