use serde::Serialize;
use wasm_bindgen::JsValue;

use super::{changes::Timeout, selector::Selector};
use crate::events::SequenceID;

#[derive(Serialize, Default, Debug)]
pub struct Replication {
    /// Reference a filter function from a design document to selectively get updates.
    /// To use a view function, pass `_view` here and provide a reference to the view
    /// function in [view].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
    /// Only show changes for docs with these ids.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub doc_ids: Vec<String>,
    /// Object containing properties that are passed to the filter function, e.g.
    /// `{"foo:"bar"}`, where `"bar"` will be available in the filter function as
    /// `params.query.foo`. To access the params, define your filter function like
    /// `function (doc, params) {/* ... */}`.
    #[serde(skip_serializing)]
    pub query_params: Option<JsValue>,
    /// Specify a view function (e.g. `"design_doc_name/view_name"` or `"view_name"` as
    /// shorthand for `"view_name/view_name"`) to act as a filter. Documents counted as
    /// “passed” for a view filter if a map function emits at least one record for them.
    /// Note: [filter] must be set to `"_view"` for this option to work.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view: Option<String>,
    /// Filter using a query/pouchdb-find selector. Note: Selectors are not supported
    /// in CouchDB 1.x. Cannot be used in combination with the filter option.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector: Option<Selector>,
    /// Start the results from the change immediately after the given sequence
    /// number. You can also pass `"now"` if you want only new changes (when [live] is true).
    #[serde(skip_serializing)]
    pub since: Option<SequenceID>,
    /// Configure the heartbeat supported by CouchDB which keeps the change connection alive.
    #[serde(skip_serializing_if = "Timeout::is_default")]
    pub heartbeat: Timeout,
    /// Request timeout (in milliseconds).
    #[serde(skip_serializing_if = "Timeout::is_default")]
    pub timeout: Timeout,
    /// Number of change feed items to process at a time. Defaults to 100. This affects
    /// the number of docs and attachments held in memory and the number sent at a time to
    /// the target server. You may need to adjust downward if targeting devices with low
    /// amounts of memory (e.g. phones) or if the documents and/or attachments are large
    /// in size or if there are many conflicted revisions. If your documents are small in
    /// size, then increasing this number will probably speed replication up.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_size: Option<u32>,
    /// Number of batches to process at a time. Defaults to 10. This (along with
    /// [batch_size]) controls how many docs are kept in memory at a time, so the
    /// maximum docs in memory at once would equal `batch_size × batches_limit`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batches_limit: Option<u32>,
    // some options are skipped, because they're not useful right now.
}
