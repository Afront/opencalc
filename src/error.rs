#[derive(Debug)]
pub enum Error {}

/**
impl std::convert::From<addr::Error> for Error {
    fn from(err: addr::Error) -> Error {
        Error::Addr(err)
    }
}

impl std::convert::From<validators::email::EmailError> for Error {
    fn from(err: validators::email::EmailError) -> Error {
        Error::Email(err)
    }
}

impl std::convert::From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Error {
        Error::Reqwest(err)
    }
}

impl std::convert::From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::SerdeJSON(err)
    }
}
**/

pub type Result<T> = std::result::Result<T, Error>;
