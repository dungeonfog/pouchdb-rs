use js_sys::Promise;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "pouchdb")]
extern "C" {
    #[wasm_bindgen(js_name = default)]
    pub type PouchDB;

    #[wasm_bindgen(constructor, js_class = default)]
    pub fn new(name_or_opts: JsValue) -> PouchDB;

    #[wasm_bindgen(method, js_class = default)]
    pub fn destroy(this: &PouchDB) -> Promise;

    #[wasm_bindgen(method, js_class = default)]
    pub fn put(this: &PouchDB, doc: JsValue) -> Promise;

    #[wasm_bindgen(method, js_class = default)]
    pub fn post(this: &PouchDB, doc: JsValue) -> Promise;

    #[wasm_bindgen(method, js_class = default)]
    pub fn get(this: &PouchDB, docId: JsValue, options: JsValue) -> Promise;

    #[wasm_bindgen(method, js_class = default)]
    pub fn remove(this: &PouchDB, doc: JsValue) -> Promise;
}
