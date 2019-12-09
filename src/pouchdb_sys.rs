use js_sys::Promise;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "pouchdb")]
extern "C" {
    pub type PouchDB;

    #[wasm_bindgen(constructor)]
    pub fn new(name_or_opts: JsValue) -> PouchDB;

    #[wasm_bindgen(method)]
    pub fn destroy(this: &PouchDB) -> Promise;

    #[wasm_bindgen(method)]
    pub fn put(this: &PouchDB, doc: JsValue) -> Promise;

    #[wasm_bindgen(method)]
    pub fn post(this: &PouchDB, doc: JsValue) -> Promise;

    #[wasm_bindgen(method)]
    pub fn get(this: &PouchDB, docId: JsValue, options: JsValue) -> Promise;

    #[wasm_bindgen(method)]
    pub fn remove(this: &PouchDB, doc: JsValue) -> Promise;
}
