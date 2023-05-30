use thiserror::Error;

pub use super::users::UserId;

#[allow(dead_code)]
#[inline]
pub fn random_base62(n: usize) -> u64 {
    random_base62_rng(&mut rand::thread_rng(), n)
}

pub fn random_base62_rng<R: rand::RngCore>(rng: &mut R, n: usize) -> u64 {
    use rand::Rng;
    assert!(n > 0 && n <= 11);
    rng.gen_range(MULTIPLES[n - 1]..MULTIPLES[n])
}

const MULTIPLES: [u64; 12] = [
    1,
    62,
    62 * 62,
    62 * 62 * 62,
    62 * 62 * 62 * 62,
    62 * 62 * 62 * 62 * 62,
    62 * 62 * 62 * 62 * 62 * 62,
    62 * 62 * 62 * 62 * 62 * 62 * 62,
    62 * 62 * 62 * 62 * 62 * 62 * 62 * 62,
    62 * 62 * 62 * 62 * 62 * 62 * 62 * 62 * 62,
    62 * 62 * 62 * 62 * 62 * 62 * 62 * 62 * 62 * 62,
    u64::MAX,
];

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Base62Id(pub u64);

/// An error decoding a number from base62.
#[derive(Error, Debug)]
pub enum DecodingError {
    /// Encountered a non-base62 character in a base62 string
    #[error("Invalid character {0:?} in base62 encoding")]
    InvalidBase62(char),
    /// Encountered integer overflow when decoding a base62 id.
    #[error("Base62 decoding overflowed")]
    Overflow,
}

macro_rules! from_base62id {
    ($($struct:ty, $con:expr;)+) => {
        $(
            impl From<Base62Id> for $struct {
                fn from(id: Base62Id) -> $struct {
                    $con(id.0)
                }
            }
            impl From<$struct> for Base62Id {
                fn from(id: $struct) -> Base62Id {
                    Base62Id(id.0)
                }
            }
        )+
    };
}

macro_rules! impl_base62_display {
    ($struct:ty) => {
        impl std::fmt::Display for $struct {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(&base62_impl::to_base62(self.0))
            }
        }
    };
}

macro_rules! impl_new_method {
    ($struct:ty) => {
        impl $struct {
            pub fn new() -> Self {
                Self(0)
            }
        }
    };
}
/*
macro_rules! impl_base62_sql_type {
    ($struct:ty) => {
        impl sqlx::Type<sqlx::Sqlite> for $struct {
            fn type_info() -> sqlx::sqlite::SqliteTypeInfo {
                <i64 as sqlx::Type<sqlx::Sqlite>>::type_info()
            }
        }
    };
}
 */

impl_base62_display!(Base62Id);

macro_rules! base62_id_impl {
    ($struct:ty, $cons:expr) => {
        from_base62id!($struct, $cons;);
        impl_base62_display!($struct);
        impl_new_method!($struct);
    }
}

base62_id_impl!(UserId, UserId);

pub mod base62_impl {
    use serde::de::{self, Deserializer, Visitor};
    use serde::ser::Serializer;
    use serde::{Deserialize, Serialize};

    use super::{Base62Id, DecodingError};

    impl<'de> Deserialize<'de> for Base62Id {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            struct Base62Visitor;

            impl<'de> Visitor<'de> for Base62Visitor {
                type Value = Base62Id;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str("a base62 string id")
                }

                fn visit_str<E>(self, string: &str) -> Result<Base62Id, E>
                where
                    E: de::Error,
                {
                    parse_base62(string).map(Base62Id).map_err(E::custom)
                }
            }

            deserializer.deserialize_str(Base62Visitor)
        }
    }

    impl Serialize for Base62Id {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.serialize_str(&to_base62(self.0))
        }
    }

    const BASE62_CHARS: [u8; 62] =
        *b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

    pub fn to_base62(mut num: u64) -> String {
        let length = (num as f64).log(62.0).ceil() as usize;
        let mut output = String::with_capacity(length);

        while num > 0 {
            // Could be done more efficiently, but requires byte
            // manipulation of strings & Vec<u8> -> String conversion
            output.insert(0, BASE62_CHARS[(num % 62) as usize] as char);
            num /= 62;
        }
        output
    }

    pub fn parse_base62(string: &str) -> Result<u64, DecodingError> {
        let mut num: u64 = 0;
        for c in string.chars() {
            let next_digit;
            if c.is_ascii_digit() {
                next_digit = (c as u8 - b'0') as u64;
            } else if c.is_ascii_uppercase() {
                next_digit = 10 + (c as u8 - b'A') as u64;
            } else if c.is_ascii_lowercase() {
                next_digit = 36 + (c as u8 - b'a') as u64;
            } else {
                return Err(DecodingError::InvalidBase62(c));
            }

            // We don't want this panicking or wrapping on integer overflow
            if let Some(n) = num.checked_mul(62).and_then(|n| n.checked_add(next_digit)) {
                num = n;
            } else {
                return Err(DecodingError::Overflow);
            }
        }
        Ok(num)
    }
}
