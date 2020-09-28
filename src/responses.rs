use crate::document::Revision;
use js_sys::Reflect;
use serde::Deserialize;
use std::convert::TryFrom;
use wasm_bindgen::JsValue;

#[derive(Deserialize, Debug)]
pub struct DestroyResponse {
    pub ok: bool,
}

#[derive(Debug)]
pub struct ChangeResponse {
    pub ok: bool,
    pub id: String,
    pub rev: Revision,
}

impl TryFrom<JsValue> for ChangeResponse {
    type Error = crate::error::Error;
    fn try_from(value: JsValue) -> Result<Self, Self::Error> {
        let id = Reflect::get(&value, &JsValue::from_str("id"))?;
        let ok = Reflect::get(&value, &JsValue::from_str("ok"))?.is_truthy();
        let rev = Reflect::get(&value, &JsValue::from_str("rev"))?;

        if let Some(id) = id.as_string() {
            if !rev.is_undefined() {
                return Ok(Self {
                    ok,
                    id,
                    rev: Revision(rev),
                });
            }
        }
        Err(crate::error::Error::Js(JsValue::from_str(
            "Response did not contain the required elements.",
        )))
    }
}
