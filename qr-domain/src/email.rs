use std::fmt::{self, Debug, Display};
use std::str::FromStr;

use serde::{
    de::{self, Deserialize, Deserializer},
    ser::{Serialize, Serializer},
};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Hash)]
pub struct EmailAddress(String);

#[derive(Debug, Error, Clone)]
#[error("incalid email address")]
pub struct FromStringError {
    _priv: (),
}

impl EmailAddress {
    pub fn from_string(s: String) -> Result<EmailAddress, FromStringError> {
        if is_valid_email_address(&s) {
            Ok(EmailAddress(s))
        } else {
            Err(FromStringError { _priv: () })
        }
    }
}

impl From<EmailAddress> for String {
    fn from(email: EmailAddress) -> String {
        email.0
    }
}

impl Display for EmailAddress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl FromStr for EmailAddress {
    type Err = FromStringError;
    fn from_str(s: &str) -> Result<EmailAddress, Self::Err> {
        let s = s.to_owned();
        EmailAddress::from_string(s)
    }
}

impl<'de> Deserialize<'de> for EmailAddress {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        EmailAddress::from_string(String::deserialize(deserializer)?).map_err(de::Error::custom)
    }
}

impl Serialize for EmailAddress {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(self)
    }
}

fn split_local_and_domain(s: &str) -> Option<(&str, &str)> {
    let mut sp = s.splitn(2, '@');
    let local_part = match sp.next() {
        None => return None,
        Some(x) if x.is_empty() => return None,
        Some(x) => x,
    };
    let domain = match sp.next() {
        None => return None,
        Some(x) if x.is_empty() => return None,
        Some(x) => x,
    };
    assert!(sp.next().is_none());
    return Some((local_part, domain));
}

fn is_valid_local(s: &str) -> bool {
    s.bytes().all(|c| {
        c == b'.' || c.is_ascii_alphanumeric() || (c.is_ascii_punctuation() && !is_specials(c))
    })
}

fn is_valid_email_address(s: &str) -> bool {
    let (local_part, domain) = match split_local_and_domain(s) {
        None => return false,
        Some(x) => x,
    };

    if !is_valid_local(local_part) {
        return false;
    }

    for label in domain.split('.') {
        if label.is_empty() || label.len() > 63 {
            return false;
        }
        if label.starts_with('-') || label.ends_with('-') {
            return false;
        }
        if !label
            .bytes()
            .all(|c| c == b'-' || c.is_ascii_alphanumeric())
        {
            return false;
        }
    }
    return true;
}

pub fn is_valid_sohosai_email_address(s: &str) -> bool {
    let (local_part, domain) = match split_local_and_domain(s) {
        None => return false,
        Some(x) => x,
    };

    if !is_valid_local(local_part) {
        return false;
    }

    if !(domain == "sohosai.com") {
        return false;
    }

    return true;
}

fn is_specials(b: u8) -> bool {
    matches!(
        b,
        b'(' | b')' | b'<' | b'>' | b'[' | b']' | b':' | b';' | b'@' | b'\\' | b',' | b'.' | b'"'
    )
}

#[cfg(test)]
mod tests {
    use super::{is_valid_sohosai_email_address, EmailAddress};
    use std::str::FromStr;

    #[test]
    fn test_address_invalid() {
        assert!(EmailAddress::from_str("").is_err());
        assert!(EmailAddress::from_str("a@a@a").is_err());
        assert!(EmailAddress::from_str("a(b)c@a.b").is_err());
        assert!(EmailAddress::from_str("ab@a-.b").is_err());
        assert!(EmailAddress::from_str("@a.b").is_err());
        assert!(EmailAddress::from_str("a@").is_err());
    }

    #[test]
    fn test_address_valid() {
        assert!(EmailAddress::from_str("a@b.c").is_ok());
        assert!(EmailAddress::from_str("a.b.c@de.fg").is_ok());
        assert!(EmailAddress::from_str("ab.c@d-e.fg").is_ok());
    }

    #[test]
    fn test_sohosai_address_invalid() {
        assert!(!is_valid_sohosai_email_address("hello@example.com"));
    }

    #[test]
    fn test_sohosai_address_valid() {
        assert!(is_valid_sohosai_email_address("hello@sohosai.com"));
    }
}
