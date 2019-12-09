use serde::Serialize;

#[derive(Serialize)]
#[serde(untagged)]
pub enum OpenRevs {
    #[serde(serialize_with = "OpenRevs::serialize_all")]
    All,
    Revs(Vec<String>),
    #[serde(skip_serializing)]
    Default,
}
impl OpenRevs {
    pub fn is_default(&self) -> bool {
        if let OpenRevs::Default = self {
            true
        } else {
            false
        }
    }
    fn serialize_all<S: serde::Serializer>(s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str("all")
    }
}
impl Default for OpenRevs {
    fn default() -> OpenRevs {
        OpenRevs::Default
    }
}
impl<T: Into<String>> From<Vec<T>> for OpenRevs {
    fn from(vec: Vec<T>) -> OpenRevs {
        OpenRevs::Revs(vec.into_iter().map(Into::into).collect())
    }
}

#[derive(Serialize, Default)]
pub struct FetchOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rev: Option<String>,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub revs: bool,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub revs_info: bool,
    #[serde(skip_serializing_if = "OpenRevs::is_default")]
    pub open_revs: OpenRevs,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflicts: Option<Vec<String>>,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub attachments: bool,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub binary: bool,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub latest: bool,
}

impl FetchOptions {
    pub fn rev<T: Into<String>>(self, rev: T) -> Self {
        Self {
            rev: Some(rev.into()),
            ..self
        }
    }
    pub fn revs(self, revs: bool) -> Self {
        Self { revs, ..self }
    }
    pub fn revs_info(self, revs_info: bool) -> Self {
        Self { revs_info, ..self }
    }
    pub fn open_revs<T: Into<OpenRevs>>(self, open_revs: T) -> Self {
        Self {
            open_revs: open_revs.into(),
            ..self
        }
    }
    pub fn conflicts<T: Into<Vec<String>>>(self, conflicts: T) -> Self {
        Self {
            conflicts: Some(conflicts.into()),
            ..self
        }
    }
    pub fn attachments(self, attachments: bool) -> Self {
        Self {
            attachments,
            ..self
        }
    }
    pub fn binary(self, binary: bool) -> Self {
        Self { binary, ..self }
    }
    pub fn latest(self, latest: bool) -> Self {
        Self { latest, ..self }
    }
}

#[cfg(test)]
mod tests {
    use super::OpenRevs;

    #[test]
    fn openrevs_serializes_correctly() {
        let all = OpenRevs::All;
        let specific = OpenRevs::from(vec!["bla", "blub", "three"]);

        let all_string = serde_json::to_string(&all).unwrap();
        assert_eq!(all_string, r#""all""#);

        let specific_string = serde_json::to_string(&specific).unwrap();
        assert_eq!(specific_string, r#"["bla","blub","three"]"#);
    }
}
