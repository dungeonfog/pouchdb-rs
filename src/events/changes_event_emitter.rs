use std::convert::TryFrom;
use wasm_bindgen::JsValue;
use js_sys::{Function, Reflect, Array};

use super::{EventEmitter, EventName, EventListener, SequenceID};
use crate::document::{SerializedDocument, Revision};

pub struct ChangeEvent {
    pub id: String,
    pub changes: Vec<Revision>,
    pub seq: SequenceID,
    pub deleted: bool,
    pub doc: Option<SerializedDocument>,
}

impl ChangeEvent {
    pub(crate) fn new(info: &JsValue) -> Result<Self, JsValue> {
        if let Some(id) = Reflect::get(&info, &JsValue::from_str("id")).ok().filter(|value| !value.is_undefined()).and_then(|id| id.as_string()) {
            if let Some(changes) = Reflect::get(&info, &JsValue::from_str("changes")).ok().filter(|value| Array::is_array(&value)) {
                let rev = JsValue::from_str("rev");
                let changes: Vec<Revision> = Array::from(&changes).iter().filter_map(|change| {
                    Reflect::get(&change, &rev).ok().map(|rev| Revision(rev))
                }).collect();
                if let Some(seq) = Reflect::get(&info, &JsValue::from_str("seq")).ok().map(|seq| SequenceID(seq)) {
                    if let Some(_deleted) = Reflect::get(&info, &JsValue::from_str("deleted")).ok().map(|b| b.is_truthy()) {
                        return Ok(ChangeEvent {
                            id, changes, seq,
                            deleted: true,
                            doc: None,
                        });
                    } else {
                        let doc = if let Some(doc) = Reflect::get(&info, &JsValue::from_str("doc")).ok().filter(|value| !value.is_undefined()) {
                            SerializedDocument::try_from(doc).ok()
                        } else {
                            None
                        };
                        return Ok(ChangeEvent {
                            id, changes, seq, doc,
                            deleted: false,
                        });
                    }
                }
            }
        }
        Err(JsValue::from_str("Failed parsing change event!"))
    }
}

/// Returned by [PouchDB::changes]
pub struct ChangesEventEmitter(EventEmitter);

impl ChangesEventEmitter {
    pub(crate) fn new(value: JsValue) -> Self {
        Self(EventEmitter::new(value))
    }

    fn as_js(&self) -> &JsValue {
        self.0.as_js()
    }
    /// Call this function if you don’t want to listen to new changes anymore.
    /// It will unsubscribe all event listeners automatically.
    pub fn cancel(self) {
        if let Ok(cancel) = Reflect::get(self.as_js(), &JsValue::from_str("cancel")) {
            if cancel.is_function() {
                Function::from(cancel).call0(self.as_js()).ok();
            }
        }
    }

    /// This event fires when a change has been found. The parameter will contain
    /// details about the change, such as whether it was deleted and what the new `rev` is.
    /// Will contain the doc if you set `include_docs` to true.
    pub fn add_change_listener(
        &self,
        listener: impl Fn(ChangeEvent) + 'static,
    ) -> Result<EventListener, JsValue> {
        self.0.add_listener(&EventName::string("change"), move |info| {
            if let Ok(event) = ChangeEvent::new(&info) {
                listener(event);
            }
        })
    }

    /// This event fires when all changes have been read. Only cancelling
    /// the changes should trigger this event.
    pub fn add_complete_listener(
        &self,
        listener: impl Fn() + 'static, // TODO: FnOnce
    ) -> Result<EventListener, JsValue> {
        self.0.add_listener(&EventName::string("complete"), move |_| {
            listener()
        })
    }

    /// This event is fired when the changes feed is stopped due to an
    /// unrecoverable failure.
    pub fn add_error_listener(
        &self,
        listener: impl Fn(JsValue) + 'static, // TODO: FnOnce
    ) -> Result<EventListener, JsValue> {
        self.0.add_listener(&EventName::string("error"), listener)
    }
}

impl Drop for ChangesEventEmitter {
    fn drop(&mut self) {
        self.0.remove_all_listeners(None).ok();
    }
}
