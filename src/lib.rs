use serde::{de::DeserializeOwned, Serialize};
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;

mod pouchdb_sys;
use pouchdb_sys::PouchDB as JsPouchDB;

mod options;
use options::{create::CreateOptions, fetch::FetchOptions};
mod responses;
use responses::*;
mod error;
use error::Error;

pub struct PouchDB {
    inner: JsPouchDB,
}

impl PouchDB {
    pub fn new<T: Into<String>>(name: T) -> Self {
        let name: String = name.into();
        let inner = JsPouchDB::new(name.into());
        Self { inner }
    }
    pub fn with_options(options: CreateOptions) -> Self {
        let opts = JsValue::from_serde(&options).unwrap();
        let inner = JsPouchDB::new(opts);
        Self { inner }
    }

    pub async fn destroy(self) -> Result<DestroyResponse, Error> {
        JsFuture::from(self.inner.destroy())
            .await
            .map(|val| val.into_serde().unwrap())
            .map_err(Error::from)
    }

    pub async fn put<T>(&self, doc: &T) -> Result<ChangeResponse, Error>
    where
        T: Serialize,
    {
        JsFuture::from(self.inner.put(JsValue::from_serde(doc)?))
            .await
            .map(|val| val.into_serde().unwrap())
            .map_err(Error::from)
    }

    pub async fn post<T>(&self, doc: &T) -> Result<ChangeResponse, Error>
    where
        T: Serialize,
    {
        JsFuture::from(self.inner.post(JsValue::from_serde(doc)?))
            .await
            .map(|val| val.into_serde().unwrap())
            .map_err(Error::from)
    }

    pub async fn fetch<T>(&self, doc_id: String, options: &FetchOptions) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        JsFuture::from(self.inner.get(doc_id.into(), JsValue::from_serde(options)?))
            .await
            .map(|val| val.into_serde().unwrap())
            .map_err(Error::from)
    }

    pub async fn remove<T>(&self, doc: &T) -> Result<ChangeResponse, Error>
    where
        T: Serialize,
    {
        JsFuture::from(self.inner.remove(JsValue::from_serde(doc)?))
            .await
            .map(|val| val.into_serde().unwrap())
            .map_err(Error::from)
    }
}
