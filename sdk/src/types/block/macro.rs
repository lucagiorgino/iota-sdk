// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/// TODO
#[macro_export]
macro_rules! impl_id {
    ($vis:vis $name:ident, $length:literal, $doc:literal) => {
        #[doc = $doc]
        #[derive(
            Clone,
            Copy,
            Eq,
            Hash,
            PartialEq,
            Ord,
            PartialOrd,
            derive_more::From,
            derive_more::AsRef,
            packable::Packable,
        )]
        #[as_ref(forward)]
        $vis struct $name([u8; $name::LENGTH]);

        impl $name {
            #[doc = concat!("The length of a [`", stringify!($ty),"`].")]
            $vis const LENGTH: usize = $length;

            #[doc = concat!("Creates a new [`", stringify!($ty),"`].")]
            $vis fn new(bytes: [u8; $name::LENGTH]) -> Self {
                Self::from(bytes)
            }

            #[doc = concat!("Creates a null [`", stringify!($ty),"`].")]
            pub fn null() -> Self {
                Self::from([0u8; $name::LENGTH])
            }

            #[doc = concat!("Checks if the [`", stringify!($ty),"`] is null.")]
            pub fn is_null(&self) -> bool {
                self.0.iter().all(|&b| b == 0)
            }
        }

        impl core::str::FromStr for $name {
            type Err = $crate::types::block::Error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Ok($name::new(prefix_hex::decode(s).map_err($crate::types::block::Error::Hex)?))
            }
        }

        impl core::fmt::Display for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{}", prefix_hex::encode(self.0))
            }
        }

        impl core::fmt::Debug for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{}({})", stringify!($name), self)
            }
        }

        impl core::ops::Deref for $name {
            type Target = [u8; $name::LENGTH];

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };
}
pub(crate) use impl_id;

/// Helper macro to serialize types to string via serde.
#[macro_export]
#[cfg(feature = "serde")]
macro_rules! string_serde_impl {
    ($type:ty) => {
        impl serde::Serialize for $type {
            fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
                use alloc::string::ToString;

                s.serialize_str(&self.to_string())
            }
        }

        impl<'de> serde::Deserialize<'de> for $type {
            fn deserialize<D>(deserializer: D) -> Result<$type, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct StringVisitor;

                impl<'de> serde::de::Visitor<'de> for StringVisitor {
                    type Value = $type;

                    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        formatter.write_str("a string representing the value")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        let value = core::str::FromStr::from_str(v).map_err(serde::de::Error::custom)?;
                        Ok(value)
                    }
                }

                deserializer.deserialize_str(StringVisitor)
            }
        }
    };
}
pub(crate) use string_serde_impl;

/// A convenience macro to work around the fact the `[bitflags]` crate does not yet support iterating over the
/// individual flags. This macro essentially creates the `[bitflags]` and puts the individual flags into an associated
/// constant `pub const ALL_FLAGS: &'static []`.
#[macro_export]
macro_rules! create_bitflags {
    ($(#[$meta:meta])* $vis:vis $Name:ident, $TagType:ty, [$(($FlagName:ident, $TypeName:ident),)+]) => {
        bitflags! {
            $(#[$meta])*
            #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
            $vis struct $Name: $TagType {
                $(
                    // TODO: Regression in v2, may be fixed soon <https://github.com/bitflags/bitflags/issues/320>.
                    // #[doc = concat!("Signals the presence of a [`", stringify!($TypeName), "`].")]
                    const $FlagName = 1 << $TypeName::KIND;
                )*
            }
        }

        impl $Name {
            #[allow(dead_code)]
            /// Returns a slice of all possible base flags.
            $vis const ALL_FLAGS: &'static [$Name] = &[$($Name::$FlagName),*];
        }
    };
}
pub(crate) use create_bitflags;
