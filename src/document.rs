use js_sys::{JsString, Reflect, Object};
use serde::Deserialize;
use serde_json::error::Result as SerdeResult;
use std::{collections::HashMap, convert::TryFrom};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::Blob;

#[derive(Debug, Clone, PartialEq)]
pub struct Revision(pub(crate) JsValue);

impl AsRef<JsValue> for Revision {
    fn as_ref(&self) -> &JsValue {
        &self.0
    }
}

impl From<JsValue> for Revision {
    fn from(value: JsValue) -> Self {
        Revision(value)
    }
}

/// A document stored in the database. Everything serialized will be stored.
///
/// Do *not* include the `id` and `rev` parameters in the json!
/// This is handled internally.
pub trait Document {
    /// The unique identifier for this document. Can be any string.
    fn id(&self) -> String;
    /// The opaque revision id of this document.
    fn rev(&self) -> Option<&Revision>;
    /// Serialize the document into a JSON string.
    fn json(&self) -> String;
    /// Optionally, you can add binary attachments here.
    fn attachments(&self) -> HashMap<String, Blob> {
        HashMap::new()
    }
    /// Return true and [PouchDB::put] this document (or pass to [PouchDB::bulk_docs]) to delete it.
    fn deleted(&self) -> bool {
        false
    }
}

pub(crate) fn serialize<D>(doc: &D) -> Result<JsValue, JsValue>
where
    D: Document + ?Sized,
{
    let object = if doc.deleted() {
        let object = js_sys::Object::new();
        Reflect::set(&object, &JsValue::from_str("_deleted"), &JsValue::TRUE)?;
        object.into()
    } else {
        let attachments = doc.attachments();
        let doc = js_sys::JSON::parse(&doc.json())?;
        if !attachments.is_empty() {
            let root = Object::new();
            for (name, blob) in &attachments {
                let attachment = Object::new();
                Reflect::set(&attachment, &JsValue::from_str("content_type"), &JsValue::from_str(&blob.type_()))?;
                Reflect::set(&attachment, &JsValue::from_str("data"), &blob)?;
                Reflect::set(&root, &JsValue::from_str(&name), &attachment)?;
            }
            Reflect::set(&doc, &JsValue::from_str("_attachments"), &root)?;
        }
        doc
    };
    Reflect::set(
        &object,
        &JsValue::from_str("_id"),
        &JsValue::from_str(&doc.id()),
    )?;
    if let Some(rev) = doc.rev() {
        Reflect::set(&object, &JsValue::from_str("_rev"), &rev.0)?;
    }

    Ok(object.into())
}

#[derive(Debug, Clone)]
pub struct SerializedDocument {
    pub id: String,
    pub rev: Option<Revision>,
    pub conflicts: Vec<Revision>,
    pub attachments: HashMap<String, web_sys::Blob>,
    pub deleted: bool,
    data: JsValue,
}

impl SerializedDocument {
    pub fn deserialize<T>(self) -> (String, Option<Revision>, SerdeResult<T>, HashMap<String, web_sys::Blob>)
    where
        T: for<'a> Deserialize<'a>,
    {
        (self.id, self.rev, self.data.into_serde(), self.attachments)
    }

    pub(crate) fn new_deleted(id: &str, rev: JsValue) -> Self {
        Self {
            id: id.to_owned(),
            rev: Some(Revision(rev)),
            conflicts: Vec::new(),
            attachments: HashMap::new(),
            deleted: true,
            data: JsValue::NULL,
        }
    }
}

impl TryFrom<JsValue> for SerializedDocument {
    type Error = JsValue;

    fn try_from(data: JsValue) -> Result<Self, Self::Error> {
        let id: JsString = Reflect::get(&data, &JsValue::from_str("_id"))?.dyn_into()?;
        let id = id
            .as_string()
            .ok_or_else(|| JsValue::from_str("Document id is not a string."))?;
        let rev = Reflect::get(&data, &JsValue::from_str("_rev"))
            .ok()
            .map(|rev| Revision(rev));
        let conflicts = Reflect::get(&data, &JsValue::from_str("_conflicts"))
            .map(|conflicts| {
                <js_sys::Array as std::convert::From<JsValue>>::from(conflicts)
                    .iter()
                    .map(|conflict| Revision(conflict.into()))
                    .collect()
            })
            .unwrap_or_else(|_| Vec::new());
        let attachments = Reflect::get(&data, &JsValue::from_str("_attachments"))
            .and_then(|attachments| {
                Ok(Reflect::own_keys(&attachments)?
                    .iter()
                    .filter_map(|name| {
                        if let Ok(object) = Reflect::get(&attachments, &name) {
                            if let Some(name) = name.as_string() {
                                if let Ok(data) = Reflect::get(&object, &JsValue::from_str("data"))
                                {
                                    return Some((name, data.into()));
                                }
                            }
                        }
                        None
                    })
                    .collect())
            })
            .unwrap_or_else(|_| HashMap::new());

        Ok(SerializedDocument {
            id,
            rev,
            conflicts,
            attachments,
            data,
            deleted: false,
        })
    }
}
