use super::{EventEmitter, EventListener, EventName, SequenceID};
use crate::document::SerializedDocument;
use js_sys::{Function, Reflect};
use wasm_bindgen::JsValue;

#[derive(Debug)]
pub struct ChangeEvent {
    pub doc_write_failures: u32,
    pub docs_read: u32,
    pub docs_written: u32,
    pub errors: Vec<String>,
    pub last_seq: SequenceID,
    pub ok: bool,
    pub start_time: std::time::Instant,
    pub docs: Vec<SerializedDocument>,
}

impl ChangeEvent {
    fn new(_value: &JsValue) -> Result<Self, crate::Error> {
        // value.into_serde().map_err(|err| err.into())
        unimplemented!()
    }
}

/// Returned by [PouchDB::replicate]
pub struct ReplicationEventEmitter(EventEmitter);

impl ReplicationEventEmitter {
    pub(crate) fn new(value: JsValue) -> Self {
        Self(EventEmitter::new(value))
    }

    fn as_js(&self) -> &JsValue {
        self.0.as_js()
    }

    /// Call if you want to cancel live replication.
    pub fn cancel(self) {
        if let Ok(cancel) = Reflect::get(self.as_js(), &JsValue::from_str("cancel")) {
            if cancel.is_function() {
                Function::from(cancel).call0(self.as_js()).ok();
            }
        }
    }

    /// This event fires when the replication has written a new document. The parameter
    /// will contain details about the change.
    pub fn add_change_listener(
        &self,
        listener: impl Fn(ChangeEvent) + 'static,
    ) -> Result<EventListener, JsValue> {
        self.0
            .add_listener(&EventName::string("change"), move |info| {
                if let Ok(event) = ChangeEvent::new(&info) {
                    listener(event);
                }
            })
    }

    /// This event fires when replication is completed or cancelled. In a live
    /// replication, only cancelling the replication should trigger this event. The
    /// parameter will contain details about the replication.
    pub fn add_complete_listener(
        &self,
        listener: impl Fn() + 'static, // TODO: FnOnce
    ) -> Result<EventListener, JsValue> {
        self.0
            .add_listener(&EventName::string("complete"), move |_info| {
                listener();
            })
    }

    /// This event fires when the replication is paused, either because a live
    /// replication is waiting for changes, or replication has temporarily failed,
    /// with an error passed as the parameter, and is attempting to resume.
    pub fn add_paused_listener(
        &self,
        listener: impl Fn(JsValue) + 'static,
    ) -> Result<EventListener, JsValue> {
        self.0
            .add_listener(&EventName::string("paused"), move |err| {
                listener(err);
            })
    }

    /// This event fires when the replication starts actively processing changes;
    /// e.g. when it recovers from an error or new changes are available.
    pub fn add_active_listener(
        &self,
        listener: impl Fn() + 'static,
    ) -> Result<EventListener, JsValue> {
        self.0.add_listener(&EventName::string("active"), move |_| {
            listener();
        })
    }

    /// This event fires if a document failed to replicate due to validation or
    /// authorization errors.
    pub fn add_denied_listener(
        &self,
        listener: impl Fn(JsValue) + 'static, // TODO: FnOnce
    ) -> Result<EventListener, JsValue> {
        self.0
            .add_listener(&EventName::string("denied"), move |err| {
                listener(err);
            })
    }

    /// This event is fired when the replication is stopped due to an unrecoverable
    /// failure. If `retry` is false, this will also fire when the user goes offline
    /// or another network error occurs (so you can handle retries yourself, if you
    /// want).
    pub fn add_error_listener(
        &self,
        listener: impl Fn(JsValue) + 'static, // TODO: FnOnce
    ) -> Result<EventListener, JsValue> {
        self.0
            .add_listener(&EventName::string("error"), move |err| {
                listener(err);
            })
    }
}
