/// Declares a new NVAPI handle type.
///
/// Handles are opaque pointers, so this creates a new struct that is a transparent
/// wrapper around a `*const c_void`. It also implements `Default` for the handle,
/// which initializes it to a null pointer.
#[macro_export]
macro_rules! nv_declare_handle {
    (
        $(#[$meta:meta])*
        $name:ident
    ) => {
        $(#[$meta])*
        #[derive(Copy, Clone, Debug)]
        #[allow(dead_code)]
        #[repr(transparent)]
        pub struct $name(*const ::std::os::raw::c_void);

        impl Default for $name {
            fn default() -> Self {
                $name(::std::ptr::null())
            }
        }
    };
}


/// Implements `Deref` and `DerefMut` for a struct, allowing it to "inherit"
/// from a field.
///
/// This is useful for creating versioned structs where a newer version of a
/// struct contains an older version as a field.
#[macro_export]
macro_rules! nvinherit {
    (
        $v2:ident($id:ident: $v1:ty)
    ) => {
        impl ::std::ops::Deref for $v2 {
            type Target = $v1;

            fn deref(&self) -> &Self::Target {
                &self.$id
            }
        }

        impl ::std::ops::DerefMut for $v2 {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.$id
            }
        }
    };
}

/// Creates a C-compatible struct.
///
/// This macro adds `#[repr(C)]` and `#[derive(Copy, Clone, Debug)]` to the
/// struct definition, and also provides a `zeroed()` constructor that returns
/// a struct instance with all fields initialized to zero.
#[macro_export]
macro_rules! nvstruct {
    (
        $(#[$meta:meta])*
        pub struct $name:ident {
            $($tt:tt)*
        }
    ) => {
        $(#[$meta])*
        #[repr(C)]
        #[derive(Copy, Clone, Debug)]
        pub struct $name {
            $($tt)*
        }

        impl $name {
            pub fn zeroed() -> Self {
                unsafe { ::std::mem::zeroed() }
            }
        }
    };
}

/// Creates a C-style enum with a corresponding type-safe Rust enum.
///
/// It defines a type alias for the C-style enum (e.g., `pub type MyEnum = c_int;`)
/// and constants for each variant. It then creates a Rust `enum` with the same
/// variants, and provides methods for converting between the raw integer value
/// and the Rust enum, as well as an iterator over the enum's values.
#[macro_export]
macro_rules! nvenum {
    (
        $(#[$meta:meta])*
        pub enum $enum:ident / $enum_name:ident {
            $(
                $(#[$metai:meta])*
                $symbol:ident / $name:ident = $value:expr,
            )*
        }
    ) => {
        $(#[$meta])*
        pub type $enum = ::std::os::raw::c_int;
        $(
            $(#[$metai])*
            pub const $symbol: $enum = $value as _;
        )*

        $(#[$meta])*
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        #[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
        #[repr(i32)]
        pub enum $enum_name {
            $(
                $(#[$metai])*
                $name = $symbol as _,
            )*
        }

        impl $enum_name {
            pub fn from_raw(raw: $enum) -> ::std::result::Result<Self, crate::ArgumentRangeError> {
                match raw {
                    $(
                        $symbol
                    )|* => Ok(unsafe { ::std::mem::transmute(raw) }),
                    _ => Err(Default::default()),
                }
            }

            pub fn raw(&self) -> $enum {
                *self as _
            }

            pub fn values() -> impl Iterator<Item=Self> {
                [
                    $(
                        $enum_name::$name
                    ),*
                ].into_iter()
            }
        }

        impl Into<$enum> for $enum_name {
            fn into(self) -> $enum {
                self as _
            }
        }
    };
}


/// Creates a bitflags enum using the `bitflags` crate.
///
/// This macro defines a `u32` type alias for the bitflags, and then uses the
/// `bitflags!` macro to generate a struct that represents a set of bit flags.
/// It also implements an `Iterator` to iterate over the set flags.
#[macro_export]
macro_rules! nvbits {
    (
        $(#[$meta:meta])*
        pub enum $enum:ident / $enum_name:ident {
            $(
                $(#[$($metai:tt)*])*
                $symbol:ident / $name:ident = $value:expr,
            )*
        }
    ) => {
        $(#[$meta])*
        pub type $enum = u32;
        $(
            $(#[$($metai)*])*
            pub const $symbol: $enum = $value as _;
        )*

        bitflags::bitflags! {
            $(#[$meta])*
            #[derive(Default)]
            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub struct $enum_name: $enum {
            $(
                $(#[$($metai)*])*
                const $name = $value;
            )*
            }
        }

        impl Iterator for $enum_name {
            type Item = Self;

            fn next(&mut self) -> Option<Self::Item> {
                $(
                    if self.contains($enum_name::$name) {
                        self.remove($enum_name::$name);
                        Some($enum_name::$name)
                    } else
                 )*
                { None }
            }
        }
    };
}


/// Implements the `Display` trait for an enum, allowing it to be formatted as a string.
///
/// This macro has two forms:
/// - `nvenum_display!(MyEnum => _)`: Implements `Display` by forwarding to the `Debug` implementation.
/// - `nvenum_display!(MyEnum => { Name = "Value", ... })`: Implements `Display` with a custom format string for each variant.
macro_rules! nvenum_display {
    ($enum:ident => _) => {
        impl ::std::fmt::Display for $enum {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                ::std::fmt::Debug::fmt(self, f)
            }
        }
    };
    ($enum:ident => {
        $(
            $name:tt = $value:tt,
        )*
    }) => {
        impl ::std::fmt::Display for $enum {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                match *self {
                $(
                    nvenum_display!(@q $enum $name) => nvenum_display!(@expr self f $value),
                    //$enum::$name => nvenum_display!(@expr self f $value),
                )*
                }
            }
        }
    };
    (@q $enum:ident _) => {
        _
    };
    (@q $enum:ident $name:ident) => {
        $enum::$name
    };
    (@expr $this:tt $fmt:ident _) => {
        ::std::fmt::Debug::fmt($this, $fmt)
    };
    (@expr $this:tt $fmt:ident $expr:expr) => {
        write!($fmt, "{}", $expr)
    };
}


/// Declares and calls an NVAPI function.
///
/// This macro simplifies the process of calling NVAPI functions. It takes a
/// function signature, looks up the function pointer at runtime using
/// `nvapi::query_interface`, caches the pointer in a static atomic variable,
/// and then calls the function.
///
/// It also has a form for defining a function pointer type alongside the function.
#[macro_export]
macro_rules! nvapi_fn {
    (
        $(#[$meta:meta])*
        pub unsafe fn $fn:ident($($arg:ident: $arg_ty:ty),*) -> $ret:ty;
    ) => {
        $(#[$meta])*
        pub unsafe fn $fn($($arg: $arg_ty),*) -> $ret {
            static CACHE: ::std::sync::atomic::AtomicUsize = ::std::sync::atomic::AtomicUsize::new(0);

            match crate::nvapi::query_interface(crate::nvid::Api::$fn.id(), &CACHE) {
                Ok(ptr) => ::std::mem::transmute::<_, extern "C" fn($($arg: $arg_ty),*) -> $ret>(ptr)($($arg),*),
                Err(e) => e.raw(),
            }
        }
    };
    (
        pub type $name:ident = extern "C" fn($($arg:ident: $arg_ty:ty),*) -> $ret:ty;

        $(#[$meta:meta])*
        pub unsafe fn $fn:ident;
    ) => {
        pub type $name = extern "C" fn($($arg: $arg_ty),*) -> $ret;

        nvapi_fn! {
            $(#[$meta])*
            pub unsafe fn $fn($($arg: $arg_ty),*) -> $ret;
        }
    };
}


/// Creates a version number for an NVAPI struct.
///
/// NVAPI structs are often versioned. This macro packs the struct's size and
/// a version number into a `u32` value, which is then passed to NVAPI functions.
/// It also includes a compile-time test to assert that the struct size is correct.
// No `const fn` yet :(
#[macro_export]
macro_rules! nvversion {
    ($name:ident($struct:ident = $sz:expr, $ver:expr)) => {
        pub const $name: u32 = ($sz) as u32 | ($ver as u32) << 16;
        /*pub fn $name() -> u32 {
            MAKE_NVAPI_VERSION::<$struct>($ver)
        }*/

        mod $name {
            #[test]
            fn $name() {
                assert_eq!(crate::types::GET_NVAPI_SIZE(super::$name), ::std::mem::size_of::<super::$struct>());
            }
        }
    };
    ($name:ident = $target:ident) => {
        pub const $name: u32 = $target;
        /*pub fn $name() -> u32 {
            $target()
        }*/
    };
}

