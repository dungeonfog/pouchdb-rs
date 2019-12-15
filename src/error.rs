use serde_json::Error as SerdeError;
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
