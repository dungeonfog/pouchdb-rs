use serde_json::Error as SerdeError;
use std::fmt::Debug;
use wasm_bindgen::JsValue;

#[derive(Debug)]
pub enum Error {
    Js(JsValue),
    Serde(SerdeError),
}

impl From<JsValue> for Error {
    fn from(v: JsValue) -> Error {
        Error::Js(v)
    }
}

impl From<SerdeError> for Error {
    fn from(err: SerdeError) -> Error {
        Error::Serde(err)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Js(_) => None,
            Self::Serde(err) => err.source(),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Js(err) => err.fmt(f),
            Self::Serde(err) => <SerdeError as std::fmt::Display>::fmt(err, f),
        }
    }
}
