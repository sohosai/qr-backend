use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct Bearer {
    pub token: String,
}

#[derive(Debug, Error, Clone)]
#[error("invalid bearer authentication header value")]
pub struct FromStrError {
    _priv: (),
}

impl FromStr for Bearer {
    type Err = FromStrError;
    fn from_str(s: &str) -> Result<Bearer, Self::Err> {
        let token = match s.trim().strip_prefix(("Bearer")) {
            Some(x) => x,
            None => return Err(FromStrError { _priv: () }),
        };
        Ok(Bearer {
            token: token.trim().to_owned(),
        })
    }
}
