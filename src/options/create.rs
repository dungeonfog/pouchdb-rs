use serde::Serialize;

#[derive(Serialize, Copy, Clone)]
pub enum Adapter {
    #[serde(rename = "idb")]
    IDB,
    #[serde(rename = "leveldb")]
    LevelDB,
    #[serde(rename = "http")]
    HTTP,
}

#[derive(Serialize, Clone, Default)]
pub struct CreateOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adapter: Option<Adapter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revs_limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deterministic_revs: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth: Option<Auth>,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub skip_setup: bool,
}

impl CreateOptions {
    pub fn name<T: Into<String>>(self, name: T) -> Self {
        Self {
            name: Some(name.into()),
            ..self
        }
    }
    pub fn adapter<T: Into<Adapter>>(self, adapter: T) -> Self {
        Self {
            adapter: Some(adapter.into()),
            ..self
        }
    }
    pub fn revs_limit<T: Into<i64>>(self, revs_limit: T) -> Self {
        Self {
            revs_limit: Some(revs_limit.into()),
            ..self
        }
    }
    pub fn deterministic_revs(self, deterministic_revs: bool) -> Self {
        Self {
            deterministic_revs: Some(deterministic_revs),
            ..self
        }
    }
    pub fn auth(self, auth: Auth) -> Self {
        Self {
            auth: Some(auth),
            ..self
        }
    }
    pub fn skip_setup(self, skip_setup: bool) -> Self {
        Self { skip_setup, ..self }
    }
}

#[derive(Serialize, Clone)]
pub struct Auth {
    pub username: String,
    pub password: String,
}
