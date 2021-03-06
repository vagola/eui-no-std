use crate::{string_to_eui, Eui48, Eui64, StringToEuiError};
use core::fmt;
use serde::de::Visitor;
use serde::de::{Error, Unexpected};
use serde::{Deserialize, Deserializer};

struct Eui48Visitor;
struct Eui64Visitor;

impl<'de> Visitor<'de> for Eui48Visitor {
    type Value = Eui48;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "12 byte string with only hexadecimal characters or \
             17 byte string with hexadecimal characters and separator after every second character"
        )
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        if v.len() != 12 && v.len() != 17 {
            return Err(Error::invalid_length(v.len(), &self));
        }

        let mut result = [0; 6];

        match string_to_eui(v, &mut result[..]) {
            Err(StringToEuiError::InvalidLength { length }) => {
                return Err(Error::invalid_length(length, &self));
            }
            Err(StringToEuiError::InvalidChar { char }) => {
                return Err(Error::invalid_value(Unexpected::Char(char), &self));
            }
            Err(StringToEuiError::InvalidSeparatorPlace) => {
                return Err(Error::custom(
                    "Separator must be placed after every second character",
                ))
            }
            Err(StringToEuiError::OnlyOneSeparatorTypeExpected) => {
                return Err(Error::custom("Only one type of separator should be used"));
            }
            Ok(()) => return Ok(Eui48(result)),
        }
    }
}

impl<'de> Visitor<'de> for Eui64Visitor {
    type Value = Eui64;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "16 byte string with only hexadecimal characters or \
             23 byte string with hexadecimal characters and separator after every second character"
        )
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        if v.len() != 16 && v.len() != 23 {
            return Err(Error::invalid_length(v.len(), &self));
        }

        let mut result = [0; 8];

        match string_to_eui(v, &mut result[..]) {
            Err(StringToEuiError::InvalidLength { length }) => {
                return Err(Error::invalid_length(length, &self));
            }
            Err(StringToEuiError::InvalidChar { char }) => {
                return Err(Error::invalid_value(Unexpected::Char(char), &self));
            }
            Err(StringToEuiError::InvalidSeparatorPlace) => {
                return Err(Error::custom(
                    "Separator must be placed after every second character",
                ))
            }
            Err(StringToEuiError::OnlyOneSeparatorTypeExpected) => {
                return Err(Error::custom("Only one type of separator should be used"));
            }
            Ok(()) => return Ok(Eui64(result)),
        }
    }
}

impl<'de> Deserialize<'de> for Eui48 {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D>::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(Eui48Visitor)
    }
}

impl<'de> Deserialize<'de> for Eui64 {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D>::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(Eui64Visitor)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Eui48, Eui64};
    use serde_test::{assert_de_tokens, assert_de_tokens_error, Token};

    #[test]
    fn test_eui48_deserialize_lowercase() {
        assert_de_tokens(
            &Eui48::from(85204980412143),
            &[Token::String("4d7e54972eef")],
        );
    }

    #[test]
    fn test_eui48_deserialize_uppercase() {
        assert_de_tokens(
            &Eui48::from(85204980412143),
            &[Token::String("4D7E54972EEF")],
        );
    }

    #[test]
    fn test_eui64_deserialize_lowercase() {
        assert_de_tokens(
            &Eui64::from(5583992946972634863),
            &[Token::String("4d7e540000972eef")],
        );
    }

    #[test]
    fn test_eui64_deserialize_uppercase() {
        assert_de_tokens(
            &Eui64::from(5583992946972634863),
            &[Token::String("4D7E540000972EEF")],
        );
    }

    #[test]
    fn test_eui48_deserialize_invalid_length() {
        assert_de_tokens_error::<Eui48>(
            &[Token::Str("4d7e54972e")],
            "invalid length 10, expected 12 byte string with only hexadecimal characters or \
             17 byte string with hexadecimal characters and separator after every second character",
        );

        assert_de_tokens_error::<Eui48>(
            &[Token::Str("4d7e54972eefef4d")],
            "invalid length 16, expected 12 byte string with only hexadecimal characters or \
             17 byte string with hexadecimal characters and separator after every second character",
        );

        assert_de_tokens_error::<Eui48>(
            &[Token::Str("4d7e54972eefef4da")],
            "invalid length 17, expected 12 byte string with only hexadecimal characters or \
             17 byte string with hexadecimal characters and separator after every second character",
        );
    }

    #[test]
    fn test_eui64_deserialize_invalid_length() {
        assert_de_tokens_error::<Eui64>(
            &[Token::Str("4d7e54972eaa")],
            "invalid length 12, expected 16 byte string with only hexadecimal characters or \
             23 byte string with hexadecimal characters and separator after every second character",
        );

        assert_de_tokens_error::<Eui64>(
            &[Token::Str("4d7e54972eefef4ddd")],
            "invalid length 18, expected 16 byte string with only hexadecimal characters or \
             23 byte string with hexadecimal characters and separator after every second character",
        );
    }

    #[test]
    fn test_eui48_deserialize_invalid_character() {
        assert_de_tokens_error::<Eui48>(
            &[Token::Str("ad7e54972esa")],
            "invalid value: character `s`, expected 12 byte string with only hexadecimal characters or \
            17 byte string with hexadecimal characters and separator after every second character",
        );
    }

    #[test]
    fn test_eui64_deserialize_invalid_character() {
        assert_de_tokens_error::<Eui64>(
            &[Token::Str("ad7e54972ea721sa")],
            "invalid value: character `s`, expected 16 byte string with only hexadecimal characters or \
             23 byte string with hexadecimal characters and separator after every second character",
        );
    }

    #[test]
    fn test_eui48_deserialize_with_separator_lowercase() {
        assert_de_tokens(
            &Eui48::from(85204980412143),
            &[Token::String("4d:7e:54:97:2e:ef")],
        );

        assert_de_tokens(
            &Eui48::from(85204980412143),
            &[Token::String("4d-7e-54-97-2e-ef")],
        );
    }

    #[test]
    fn test_eui48_deserialize_with_separator_uppercase() {
        assert_de_tokens(
            &Eui48::from(85204980412143),
            &[Token::String("4D:7E:54:97:2E:EF")],
        );

        assert_de_tokens(
            &Eui48::from(85204980412143),
            &[Token::String("4D-7E-54-97-2E-EF")],
        );
    }

    #[test]
    fn test_eui64_deserialize_with_separator_lowercase() {
        assert_de_tokens(
            &Eui64::from(5583992946972634863),
            &[Token::String("4d:7e:54:00:00:97:2e:ef")],
        );

        assert_de_tokens(
            &Eui64::from(5583992946972634863),
            &[Token::String("4d-7e-54-00-00-97-2e-ef")],
        );
    }

    #[test]
    fn test_eui64_deserialize_with_separator_uppercase() {
        assert_de_tokens(
            &Eui64::from(5583992946972634863),
            &[Token::String("4D:7E:54:00:00:97:2E:EF")],
        );

        assert_de_tokens(
            &Eui64::from(5583992946972634863),
            &[Token::String("4D-7E-54-00-00-97-2E-EF")],
        );
    }

    #[test]
    fn test_eui48_deserialize_invalid_separator_position() {
        assert_de_tokens_error::<Eui48>(
            &[Token::Str(":4d7e:54:97:2e:ef")],
            "Separator must be placed after every second character",
        );

        assert_de_tokens_error::<Eui48>(
            &[Token::Str("4d:7e:54:97:2eef:")],
            "Separator must be placed after every second character",
        );

        assert_de_tokens_error::<Eui48>(
            &[Token::Str("4d::7e54:97:2e:ef")],
            "Separator must be placed after every second character",
        );
    }

    #[test]
    fn test_eui64_deserialize_invalid_separator_position() {
        assert_de_tokens_error::<Eui64>(
            &[Token::Str(":4d7e:54:00:00:97:2e:ef")],
            "Separator must be placed after every second character",
        );

        assert_de_tokens_error::<Eui64>(
            &[Token::Str("4d:7e:54:00:00:97:2eef:")],
            "Separator must be placed after every second character",
        );

        assert_de_tokens_error::<Eui64>(
            &[Token::Str("4d::7e54:00:00:97:2e:ef")],
            "Separator must be placed after every second character",
        );
    }

    #[test]
    fn test_eui48_deserialize_different_separators() {
        assert_de_tokens_error::<Eui48>(
            &[Token::Str("4d:7e:54-97:2e:ef")],
            "Only one type of separator should be used",
        );
    }

    #[test]
    fn test_eui64_deserialize_different_separators() {
        assert_de_tokens_error::<Eui64>(
            &[Token::Str("4d:7e-54:00:00:97:2e-ef")],
            "Only one type of separator should be used",
        );
    }
}
