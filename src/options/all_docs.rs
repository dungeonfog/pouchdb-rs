use serde::Serialize;

/// All options default to false unless otherwise specified.
///
/// Notes: For pagination, [limit] and [skip] are also available, but the same performance
/// concerns as in CouchDB apply. Use the [startkey/endkey pattern](http://docs.couchdb.org/en/latest/couchapp/views/pagination.html) instead.
#[derive(Serialize)]
pub struct AllDocsOptions {
    /// Include the document itself in each row in the doc field.
    /// Otherwise by default you only get the id and rev properties.
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub include_docs: bool,
    /// Include conflict information in the _conflicts field of a doc.
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub conflicts: bool,
    /// Include attachment data.
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub attachments: bool,
    /// Get documents with IDs in a certain range.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub startkey: Option<String>,
    /// Get documents with IDs in a certain range.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endkey: Option<String>,
    /// Include documents having an ID equal to the given [endkey]. Default: true
    #[serde(skip_serializing_if = "Clone::clone")]
    pub inclusive_end: bool,
    /// Maximum number of documents to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Number of docs to skip before returning (warning: poor performance on IndexedDB/LevelDB!).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<u32>,
    /// Reverse the order of the output documents. Note that the order of [startkey] and [endkey]
    /// is reversed when [descending] `== true`.
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub descending: bool,
    /// Only return documents with IDs matching this string key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// Array of string keys to fetch in a single shot.
    ///
    /// * Neither [startkey] nor [endkey] can be specified with this option.
    /// * The rows are returned in the same order as the supplied keys array.
    /// * The row for a deleted document will have the revision ID of the deletion, and the deleted flag set to true.
    /// * The row for a nonexistent document will just contain a `None` value.
    /// * For details, see the [CouchDB query options documentation](https://docs.couchdb.org/en/stable/api/ddoc/views.html#db-design-design-doc-view-view-name).
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub keys: Vec<String>,
    /// Include an update_seq value indicating which sequence id of the underlying database the view reflects.
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub update_seq: bool,
}

impl Default for AllDocsOptions {
    fn default() -> Self {
        Self {
            include_docs: false,
            conflicts: false,
            attachments: false,
            startkey: None,
            endkey: None,
            inclusive_end: true,
            limit: None,
            skip: None,
            descending: false,
            key: None,
            keys: Vec::new(),
            update_seq: false,
        }
    }
}
