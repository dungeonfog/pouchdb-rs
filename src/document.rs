use js_sys::{JsString, Reflect, Object, Promise, Array, Uint8Array, JSON, WebAssembly};
use serde::{Serialize, Deserialize};
use serde_json::error::Result as SerdeResult;
use std::{collections::HashMap, convert::TryFrom};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Blob, BlobPropertyBag};

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
    /// Serialize the document into a JSON-like object.
    fn serialize(&self) -> Result<JsValue, JsValue>;
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
        let doc = doc.serialize()?;
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
    pub data: JsValue,
}

impl SerializedDocument {
    pub fn deserialize<T>(self) -> (String, Option<Revision>, SerdeResult<T>, HashMap<String, web_sys::Blob>)
    where
        T: for<'a> Deserialize<'a>,
    {
        (self.id, self.rev, self.data.into_serde(), self.attachments)
    }
    pub async fn into_serialized(self) -> Result<SerializedDocumentData, crate::error::Error> {
        let array_buffer_str = JsValue::from_str("arrayBuffer");
        let promises = Array::new();
        for (_, blob) in self.attachments.iter() {
            promises.push(&Reflect::get(blob.as_ref(), &array_buffer_str)?);
        }
        let arraybuffers: Array = JsFuture::from(Promise::all(promises.as_ref())).await?.unchecked_into();
        let json = JSON::stringify(&self.data)?.as_string().unwrap();
        let data: serde_json::Value = serde_json::from_str(&json).unwrap();

        Ok(SerializedDocumentData {
            id: self.id,
            attachments: self.attachments.into_iter().zip(arraybuffers.iter()).map(|((name, blob), arraybuffer)| {
                (name, (blob.type_(), Uint8Array::new(arraybuffer.as_ref()).to_vec()))
            }).collect::<HashMap<String, (String, Vec<u8>)>>(),
            data,
        })
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

/// Do *not* use for existing documents! Does not store a rev.
#[derive(Serialize, Deserialize)]
pub struct SerializedDocumentData {
    pub id: String,
    pub attachments: HashMap<String, (String, Vec<u8>)>,
    pub data: serde_json::Value,
}

impl Document for SerializedDocumentData {
    fn id(&self) -> String {
        self.id.clone()
    }
    fn rev(&self) -> Option<&Revision> {
        None
    }
    fn serialize(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.data).map_err(|err| JsValue::from_str(&format!("{}", err)))
    }
    fn attachments(&self) -> HashMap<String, Blob> {
        let memory_buffer = wasm_bindgen::memory().dyn_into::<WebAssembly::Memory>().unwrap().buffer();
        self.attachments.iter().filter_map(|(name, (mime_type, binary))| {
            let binary_location = binary.as_ptr() as u32;
            let buffer = js_sys::Uint8Array::new(&memory_buffer).subarray(binary_location, binary_location + binary.len() as u32);
            let mut options = BlobPropertyBag::new();
            options.type_(&mime_type);
            Blob::new_with_u8_array_sequence_and_options(&js_sys::Array::of1(buffer.as_ref()).into(), &options).ok().map(|blob| (name.clone(), blob))
        }).collect()
    }
}