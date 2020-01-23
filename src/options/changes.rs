use serde::{Serialize, Serializer};
use wasm_bindgen::JsValue;

use super::selector::Selector;
use crate::events::SequenceID;

#[derive(PartialEq, Eq, Debug)]
pub enum Timeout {
    None,
    Default,
    Duration(std::time::Duration),
}

impl Timeout {
    pub(crate) fn is_default(&self) -> bool {
        self == &Self::Default
    }
}

impl Default for Timeout {
    fn default() -> Self {
        Self::Default
    }
}

impl Serialize for Timeout {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_bool(false),
            Self::Default => panic!("Tried to serialize default value"),
            Self::Duration(duration) => serializer.serialize_f64(duration.as_secs_f64() * 1000.0),
        }
    }
}

/// All options default to false unless otherwise specified.
#[derive(Serialize, Default, Debug)]
pub struct Changes {
    /// Include the associated document with each change.
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub include_docs: bool,
    /// Include conflicts.
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub include_conflicts: bool,
    /// Include attachments.
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub include_attachments: bool,
    /// Reverse the order of the output documents.
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub descending: bool,
    /// Start the results from the change immediately after the given sequence
    /// number. You can also pass `"now"` if you want only new changes (when [live] is true).
    #[serde(skip_serializing)]
    pub since: Option<SequenceID>,
    /// Limit the number of results to this number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Request timeout (in milliseconds).
    #[serde(skip_serializing_if = "Timeout::is_default")]
    pub timeout: Timeout,
    /// For http adapter only, time in milliseconds for server to give a heartbeat to
    /// keep long connections open. Defaults to 10000 (10 seconds).
    #[serde(skip_serializing_if = "Timeout::is_default")]
    pub heartbeat: Timeout,
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
    /// Only available for http databases, this configures how many changes to fetch
    /// at a time. Increasing this can reduce the number of requests made. Default is 25.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_size: Option<u32>,
    // some options are skipped, because they're not useful right now.
}
