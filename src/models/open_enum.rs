//! Open wire enums that preserve unknown server values.
/// An unknown value goes into `Other`, so a new value of the API does not break the decode.
/// Equality and the hash use the value, so `Other("paragraph")` is equal to `Paragraph`.
macro_rules! open_enum {
    (
        $(#[$meta:meta])*
        pub enum $name:ident {
            $( $(#[$variant_meta:meta])* $variant:ident => $value:literal, )*
        }
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
        #[serde(from = "String", into = "String")]
        #[non_exhaustive]
        #[doc = concat!("Wire values for ", stringify!($name), "; unknown values are preserved.")]
        pub enum $name {
            $( $(#[$variant_meta])* #[doc = concat!("The `", $value, "` wire value.")] $variant, )*
            /// A value that this version of the SDK does not know.
            Other(String),
        }

        impl $name {
            /// Returns the value that the API uses.
            pub fn as_str(&self) -> &str {
                match self {
                    $( $name::$variant => $value, )*
                    $name::Other(value) => value,
                }
            }
        }

        impl From<String> for $name {
            fn from(value: String) -> Self {
                match value.as_str() {
                    $( $value => $name::$variant, )*
                    _ => $name::Other(value),
                }
            }
        }

        impl From<$name> for String {
            fn from(value: $name) -> Self {
                match value {
                    $name::Other(value) => value,
                    known => known.as_str().to_string(),
                }
            }
        }

        impl PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                self.as_str() == other.as_str()
            }
        }

        impl Eq for $name {}

        impl std::hash::Hash for $name {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                self.as_str().hash(state);
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.as_str())
            }
        }
    };
}

pub(crate) use open_enum;
