use serde::Deserialize;

#[derive(Deserialize)]
pub struct DestroyResponse {
    pub ok: bool,
}

#[derive(Deserialize)]
pub struct ChangeResponse {
    pub ok: bool,
    pub id: String,
    pub rev: String,
}
