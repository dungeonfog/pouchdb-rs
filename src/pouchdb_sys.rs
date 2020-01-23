use js_sys::{Function, Promise};
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

    #[wasm_bindgen(method, js_class = default, js_name = put)]
    pub fn put_with_options(this: &PouchDB, doc: JsValue, options: JsValue) -> Promise;

    #[wasm_bindgen(method, js_class = default)]
    pub fn post(this: &PouchDB, doc: JsValue) -> Promise;

    #[wasm_bindgen(method, js_class = default, js_name = post)]
    pub fn post_with_options(this: &PouchDB, doc: JsValue, options: JsValue) -> Promise;

    #[wasm_bindgen(method, js_class = default)]
    pub fn get(this: &PouchDB, docId: JsValue) -> Promise;

    #[wasm_bindgen(method, js_class = default, js_name = get)]
    pub fn get_with_options(this: &PouchDB, docId: JsValue, options: JsValue) -> Promise;

    #[wasm_bindgen(method, js_class = default, js_name = remove)]
    pub fn remove_doc(this: &PouchDB, doc: JsValue) -> Promise;

    #[wasm_bindgen(method, js_class = default, js_name = remove)]
    pub fn remove_doc_with_options(this: &PouchDB, doc: JsValue, options: JsValue) -> Promise;

    #[wasm_bindgen(method, js_class = default, js_name = remove)]
    pub fn remove_id(this: &PouchDB, doc_id: JsValue, doc_rev: JsValue) -> Promise;

    #[wasm_bindgen(method, js_class = default, js_name = remove)]
    pub fn remove_id_with_options(
        this: &PouchDB,
        doc_id: JsValue,
        doc_rev: JsValue,
        options: JsValue,
    ) -> Promise;

    #[wasm_bindgen(method, js_class = default, js_name = bulkDocs)]
    pub fn bulk_docs(this: &PouchDB, docs: JsValue) -> Promise;

    #[wasm_bindgen(method, js_class = default, js_name = bulkDocs)]
    pub fn bulk_docs_with_options(this: &PouchDB, docs: JsValue, options: JsValue) -> Promise;

    #[wasm_bindgen(method, js_class = default, js_name = allDocs)]
    pub fn all_docs(this: &PouchDB) -> Promise;

    #[wasm_bindgen(method, js_class = default, js_name = allDocs)]
    pub fn all_docs_with_options(this: &PouchDB, options: JsValue) -> Promise;

    #[wasm_bindgen(method, js_class = default)]
    pub fn changes(this: &PouchDB, options: JsValue) -> JsValue;

    #[wasm_bindgen(method, js_class = default, js_name = changes)]
    pub fn changes_oneshot(this: &PouchDB, options: JsValue) -> Promise;

    #[wasm_bindgen(js_class = default)]
    pub fn replicate(source: JsValue, target: JsValue) -> JsValue;

    #[wasm_bindgen(js_class = default, js_name = replicate)]
    pub fn replicate_with_options(source: JsValue, target: JsValue, options: JsValue) -> JsValue;

    #[wasm_bindgen(js_class = default)]
    pub fn sync(src: JsValue, target: JsValue) -> JsValue;

    #[wasm_bindgen(js_class = default, js_name = sync)]
    pub fn sync_with_options(src: JsValue, target: JsValue, options: JsValue) -> JsValue;

    #[wasm_bindgen(method, js_class = default, js_name = putAttachment)]
    pub fn put_attachment(
        this: &PouchDB,
        attachment_id: JsValue,
        attachment: JsValue,
        _type: JsValue,
    ) -> Promise;

    #[wasm_bindgen(method, js_class = default, js_name = putAttachment)]
    pub fn put_attachment_with_rev(
        this: &PouchDB,
        attachment_id: JsValue,
        rev: JsValue,
        attachment: JsValue,
        _type: JsValue,
    ) -> Promise;

    #[wasm_bindgen(method, js_class = default, js_name = getAttachment)]
    pub fn get_attachment(this: &PouchDB, attachment_id: JsValue) -> Promise;

    #[wasm_bindgen(method, js_class = default, js_name = getAttachment)]
    pub fn get_attachment_with_options(
        this: &PouchDB,
        attachment_id: JsValue,
        options: JsValue,
    ) -> Promise;

    #[wasm_bindgen(method, js_class = default, js_name = removeAttachment)]
    pub fn remove_attachment(this: &PouchDB, attachment_id: JsValue, rev: JsValue) -> Promise;

    #[wasm_bindgen(method, js_class = default, js_name = createIndex)]
    pub fn create_index(this: &PouchDB, index: JsValue) -> Promise;

    #[wasm_bindgen(method, js_class = default)]
    pub fn find(this: &PouchDB, request: JsValue) -> Promise;

    #[wasm_bindgen(method, js_class = default)]
    pub fn explain(this: &PouchDB, request: JsValue) -> Promise;

    #[wasm_bindgen(method, js_class = default, js_name = getIndexes)]
    pub fn get_indexes(this: &PouchDB) -> Promise;

    #[wasm_bindgen(method, js_class = default, js_name = deleteIndex)]
    pub fn delete_index(this: &PouchDB, index: JsValue) -> Promise;

    #[wasm_bindgen(method, js_class = default)]
    pub fn query(this: &PouchDB, fun: Function) -> Promise;

    #[wasm_bindgen(method, js_class = default, js_name = query)]
    pub fn query_with_options(this: &PouchDB, fun: Function, options: JsValue) -> Promise;

    #[wasm_bindgen(method, js_class = default, js_name = viewCleanup)]
    pub fn view_Cleanup(this: &PouchDB) -> Promise;

    #[wasm_bindgen(method, js_class = default)]
    pub fn info(this: &PouchDB) -> Promise;

    #[wasm_bindgen(method, js_class = default)]
    pub fn compact(this: &PouchDB) -> Promise;

    #[wasm_bindgen(method, js_class = default, js_name = compact)]
    pub fn compact_with_options(this: &PouchDB, options: JsValue) -> Promise;

    #[wasm_bindgen(method, js_class = default, js_name = bulkGet)]
    pub fn bulk_get(this: &PouchDB, options: JsValue) -> Promise;

    #[wasm_bindgen(method, js_class = default)]
    pub fn close(this: &PouchDB) -> Promise;
}
