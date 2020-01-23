use std::{
    convert::{TryFrom, TryInto, AsRef},
    ops::Deref,
};

use js_sys::{Object, Reflect, Array};
use serde::de::DeserializeOwned;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;

mod pouchdb_sys;
use pouchdb_sys::PouchDB as JsPouchDB;

pub mod options;
use options::{
    all_docs::AllDocsOptions,
    changes::Changes,
    create::CreateOptions,
    fetch::FetchOptions,
    replication::Replication,
};
pub mod responses;
use responses::*;
pub mod error;
use error::Error;
pub mod document;
use document::{Document, SerializedDocument};
pub mod events;
use events::{
    SequenceID,
    changes_event_emitter::{ChangesEventEmitter, ChangeEvent},
    replication_event_emitter::{ReplicationEventEmitter},
};

pub enum PouchDBOrStringRef<'a> {
    PouchDB(&'a PouchDB),
    String(&'a str),
}

pub struct PouchDB(JsPouchDB);

impl PouchDB {
    /// Create a database
    ///
    /// This method creates a database or opens an existing one. If you use a URL like
    /// 'http://domain.com/dbname', then PouchDB will work as a client to an online CouchDB
    /// instance. Otherwise it will create a local database using whatever backend is present.
    pub fn new<T: Into<String>>(name: T) -> Self {
        let name: String = name.into();
        Self(JsPouchDB::new(name.into()))
    }
    /// Create a database with options
    pub fn new_with_options(options: CreateOptions) -> Self {
        let opts = JsValue::from_serde(&options).unwrap();
        Self(JsPouchDB::new(opts))
    }

    /// Delete a database
    ///
    /// Note that this has no impact on other replicated databases.
    pub async fn destroy(self) -> Result<DestroyResponse, Error> {
        JsFuture::from(self.0.destroy())
            .await?
            .into_serde()
            .map_err(Error::from)
    }

    /// Create/update a document
    ///
    /// Create a new document or update an existing document. If the document already
    /// exists, you must specify its revision, otherwise a conflict will occur.
    ///
    /// If you want to update an existing document even if there’s conflict, set `force`
    /// to true and let the document return the base revision, then a new
    /// conflict revision will be created.
    ///
    /// doc must be a “pure JSON object”, i.e. a collection of name/value pairs. If
    /// you try to store non-JSON data (for instance Date objects) you may see
    /// inconsistent results (might return an error from serde, to be tested).
    pub async fn put<D>(&self, doc: &D, force: bool) -> Result<ChangeResponse, Error>
    where
        D: Document + ?Sized,
    {
        let js_doc = document::serialize(doc)?;

        JsFuture::from(if force {
            let options = js_sys::Object::new();
            Reflect::set(&options, &JsValue::from_str("force"), &JsValue::TRUE)?;
            self.0.put_with_options(js_doc, options.into())
        } else {
            self.0.put(js_doc)
        })
        .await?
        .into_serde()
        .map_err(Error::from)
    }

    /// Create a document
    ///
    /// Create a new document and let PouchDB auto-generate an `id` for it.
    ///
    /// You should prefer [put] to [post], because when you [post],
    /// you are missing an opportunity to use [allDocs] to sort documents
    /// by `id` (because your `id`s are random). For more info, read the PouchDB
    /// pro tips.
    pub async fn post<D>(&self, doc: &D) -> Result<ChangeResponse, Error>
    where
        D: Document + ?Sized,
    {
        JsFuture::from(self.0.post(document::serialize(doc)?))
            .await?
            .into_serde()
            .map_err(Error::from)
    }

    /// Fetch a document
    ///
    /// Retrieves a document, specified by `doc_id`.
    pub async fn fetch(
        &self,
        doc_id: String,
        options: &FetchOptions,
    ) -> Result<SerializedDocument, Error>
    {
        JsFuture::from(
            self.0
                .get_with_options(doc_id.into(), JsValue::from_serde(options)?),
        )
        .await?
        .try_into()
        .map_err(Error::from)
    }

    /// Delete a document
    ///
    /// The document *must* return a revision!
    pub async fn remove<D>(&self, doc: &D) -> Result<ChangeResponse, Error>
    where
        D: Document + ?Sized,
    {
        let value = Object::new();

        Reflect::set(
            &value,
            &JsValue::from_str("_id"),
            &JsValue::from_str(&doc.id()),
        )?;
        Reflect::set(
            &value,
            &JsValue::from_str("_rev"),
            &doc.rev().expect("Document does not have a revision").0,
        )?;
        Reflect::set(&value, &JsValue::from_str("_deleted"), &JsValue::TRUE)?;

        JsFuture::from(self.0.put(value.into()))
            .await?
            .into_serde()
            .map_err(Error::from)
    }

    /// Create/update a batch of documents
    // pub fn bulk_docs<I: IntoIterator<Item = Box<dyn Serialize>>>(&self, docs: I) -> Result<ChangeResponse, Error> {
    pub async fn bulk_docs<I: IntoIterator<Item = Box<dyn Document>>>(
        &self,
        docs: I,
    ) -> Result<ChangeResponse, Error> {
        let array = js_sys::Array::new();
        for doc in docs {
            let object = document::serialize(doc.deref())?;
            array.push(&object);
        }
        JsFuture::from(self.0.bulk_docs(array.into()))
            .await?
            .into_serde()
            .map_err(Error::from)
    }

    /// Fetch multiple documents, indexed and sorted by the id. Deleted documents are only included
    /// if options.keys is specified.
    /// Entries in the result vector are None when the key was not found (when options.keys is supplied).
    pub async fn all_docs(
        &self,
        options: &AllDocsOptions,
    ) -> Result<Vec<Option<SerializedDocument>>, Error> {
        let options = JsValue::from_serde(options)?;
        Reflect::set(&options, &JsValue::from_str("binary"), &JsValue::TRUE)?; // we don't want to support base64

        let response = JsFuture::from(self.0.all_docs_with_options(options)).await?;
        let rows: js_sys::Array = Reflect::get(&response, &JsValue::from_str("rows"))?.into();
        Ok(rows
            .iter()
            .map(|row| {
                if !Reflect::has(&row, &JsValue::from_str("error")).unwrap_or(true) {
                    if let Some(value) = Reflect::get(&row, &JsValue::from_str("value"))
                        .ok()
                        .filter(|value| !value.is_undefined())
                    {
                        if let Ok(deleted) = Reflect::get(&value, &JsValue::from_str("deleted")) {
                            if deleted.is_truthy() {
                                if let Some(rev) = Reflect::get(&value, &JsValue::from_str("rev"))
                                    .ok()
                                    .filter(|value| !value.is_undefined())
                                    .map(|rev| rev.into())
                                {
                                    if let Some(id) = Reflect::get(&row, &JsValue::from_str("id"))
                                        .ok()
                                        .filter(|value| !value.is_undefined())
                                    {
                                        if let Some(id) = id.as_string() {
                                            return Some(SerializedDocument::new_deleted(&id, rev));
                                        }
                                    }
                                }
                            }
                        } else if let Some(doc) = Reflect::get(&row, &JsValue::from_str("doc"))
                            .ok()
                            .filter(|value| !value.is_undefined())
                        {
                            return SerializedDocument::try_from(doc).ok();
                        }
                    }
                }
                None
            })
            .collect())
    }

    /// A list of changes made to documents in the database, in the order they were made. It
    /// returns a struct with the function `cancel`, which you call if you don’t want to listen
    /// to new changes anymore.
    /// 
    /// It is an [EventEmitter] and will emit a `change` event on each document change, a
    /// `complete` event when all the changes have been processed, and an `error` event when an
    /// error occurs. Calling `cancel` will unsubscribe all event listeners automatically.
    /// 
    /// Change events
    /// - `change` (`info`) - This event fires when a change has been found. `info` will contain
    ///     details about the change, such as whether it was deleted and what the new `_rev` is.
    ///     `info.doc` will contain the doc if you set `include_docs` to true.
    /// - `complete` (`info`) - This event fires when all changes have been read. In live changes,
    ///     only cancelling the changes should trigger this event. `info.results` will contain
    ///     the list of changes.
    /// - `error` (`err`) - This event is fired when the changes feed is stopped due to an
    ///     unrecoverable failure.
    pub fn changes(&self, options: &Changes) -> Result<ChangesEventEmitter, Error> {
        let js_options = JsValue::from_serde(options)?;
        if let Some(query_params) = &options.query_params {
            if let (Some(js_options), Some(query_params)) = (
                Object::try_from(&js_options),
                Object::try_from(&query_params),
            ) {
                Object::assign(js_options, query_params);
            }
            if let Some(since) = &options.since {
                Reflect::set(&js_options, &JsValue::from_str("since"), &since.0)?;
            }
        }
        Reflect::set(&js_options, &JsValue::from_str("live"), &JsValue::TRUE)?;
        Ok(ChangesEventEmitter::new(self.0.changes(js_options)))
    }

    /// If you use [changes_oneshot] instead of [changes], it will be treated as a
    /// single-shot request, which asynchronously returns a list of the changes and the `last_seq`.
    pub async fn changes_oneshot(&self, options: &Changes) -> Result<(Vec<ChangeEvent>, SequenceID), Error> {
        let js_options = JsValue::from_serde(options)?;
        if let Some(query_params) = &options.query_params {
            if let (Some(js_options), Some(query_params)) = (
                Object::try_from(&js_options),
                Object::try_from(&query_params),
            ) {
                Object::assign(js_options, query_params);
            }
            if let Some(since) = &options.since {
                Reflect::set(&js_options, &JsValue::from_str("since"), &since.0)?;
            }
        }
        let info = JsFuture::from(self.0.changes_oneshot(js_options)).await?;
        if let Some(results) = Reflect::get(&info, &JsValue::from_str("results")).ok().filter(|results| Array::is_array(&results)) {
            if let Some(last_seq) = Reflect::get(&info, &JsValue::from_str("last_seq")).ok().filter(|last_seq| !last_seq.is_undefined()) {
                Array::from(&results).iter().map(|result| ChangeEvent::new(&result).map_err(|err| err.into())).collect::<Result<Vec<ChangeEvent>, Error>>().map(|results| (results, SequenceID(last_seq)))
            } else {
                Err(JsValue::from_str("Failed reading last_seq!").into())
            }
        } else {
            Err(JsValue::from_str("Failed reading results array!").into())
        }
    }

    /// Replicate data from `source` to `target`. Both the `source` and `target` can be a
    /// PouchDB instance or a string representing a CouchDB database URL or the name of a
    /// local PouchDB database. This call will track future changes and also replicate
    /// them automatically.
    /// 
    /// If `retry` == true will attempt to retry replications in the case of failure (due
    /// to being offline), using a backoff algorithm that retries at longer and longer
    /// intervals until a connection is re-established, with a maximum delay of 10 minutes.
    /// 
    /// This method returns an object with the method
    /// [ReplicationEventEmitter::cancel], which you call if you want to cancel live
    /// replication.
    /// 
    /// Replication is an event emitter like [changes] and emits the `complete`, `active`,
    /// `paused`, `change`, `denied` and `error` events.
    /// 
    /// Note that replication is supported for both local and remote databases. So you
    /// can replicate from local to local or from remote to remote.
    ///
    /// However, if you replicate from remote to remote, then the changes will flow
    /// through PouchDB. If you want to trigger a server-initiated replication, please
    /// use regular ajax to POST to the CouchDB `_replicate` endpoint, as described in
    /// the CouchDB docs.
    pub fn replicate(source: PouchDBOrStringRef, target: PouchDBOrStringRef, options: &Replication, retry: bool) -> Result<ReplicationEventEmitter, Error> {
        let js_options = JsValue::from_serde(options)?;
        if let Some(query_params) = &options.query_params {
            if let (Some(js_options), Some(query_params)) = (
                Object::try_from(&js_options),
                Object::try_from(&query_params),
            ) {
                Object::assign(js_options, query_params);
            }
            if let Some(since) = &options.since {
                Reflect::set(&js_options, &JsValue::from_str("since"), &since.0)?;
            }
        }
        Reflect::set(&js_options, &JsValue::from_str("live"), &JsValue::TRUE)?;
        if retry {
            Reflect::set(&js_options, &JsValue::from_str("retry"), &JsValue::TRUE)?;
        }

        // these are needed to keep the references alive
        let source_string;
        let target_string;

        let source = match source {
            PouchDBOrStringRef::PouchDB(db) => <JsPouchDB as AsRef<wasm_bindgen::JsValue>>::as_ref(&db.0),
            PouchDBOrStringRef::String(s) => {
                source_string = JsValue::from_str(s);
                &source_string
            },
        };
        let target = match target {
            PouchDBOrStringRef::PouchDB(db) => db.0.as_ref(),
            PouchDBOrStringRef::String(s) => {
                target_string = JsValue::from_str(s);
                &target_string
            },
        };

        Ok(ReplicationEventEmitter::new(JsPouchDB::replicate_with_options(source, target, js_options)))
    }

    pub async fn replicate_oneshot<'a>(_source: PouchDBOrStringRef<'a>, _target: PouchDBOrStringRef<'a>, _options: &Replication) -> Result<(), Error> {
        Ok(())
    }
}

impl std::fmt::Debug for PouchDB {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        if let Some(name) = Reflect::get(&self.0, &JsValue::from_str("name")).ok().and_then(|name| name.as_string()) {
            write!(f, "PouchDB {}", name)
        } else {
            write!(f, "PouchDB with unknown name")
        }
    }
}