use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum SimpleAddressError {
    CountryNotSupported(String),
    RenderError(mustache::Error),
}

impl Error for SimpleAddressError {}

impl Display for SimpleAddressError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SimpleAddressError::CountryNotSupported(country) => write!(f, "{} is not yet supported. Please open an issue upstream at https://github.com/Naurt-Ltd-Public/simple-delivery-address", country),
            SimpleAddressError::RenderError(error) => write!(f, "Error when rendering template: {}",error),
        }
    }
}
impl From<mustache::Error> for SimpleAddressError {
    fn from(value: mustache::Error) -> Self {
        Self::RenderError(value)
    }
}
