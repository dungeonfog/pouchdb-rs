use super::all_docs::AllDocsOptions;
use serde::Serialize;
#[derive(Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum StaleOption {
    /// Returns results immediately, even if they’re out-of-date.
    Ok,
    /// Returns results immediately, but kicks off a build afterwards.
    UpdateAfter,
}

/// All options default to false unless otherwise specified.
#[derive(Serialize, Default, Debug)]
pub struct QueryOptions {
    /// Query has the same options that all_docs has
    #[serde(flatten)]
    pub all_docs: AllDocsOptions,

    /// True if you want the reduce function to group results by keys, rather than returning a single result.
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub group: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Number of elements in a key to group by, assuming the keys are arrays. Defaults to the full length of the array.
    pub group_level: Option<usize>,
    /// One of [StaleOptions::Ok] or [StaleOptions::UpdateAfter]. Only applies to saved views. Can be one of:
    ///
    /// * `None` (default): Returns the latest results, waiting for the view to build if necessary.
    /// * [StaleOptions::Ok]: Returns results immediately, even if they’re out-of-date.
    /// * [StaleOptions::UpdateAfter]: Returns results immediately, but kicks off a build afterwards.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stale: Option<StaleOption>,
}
