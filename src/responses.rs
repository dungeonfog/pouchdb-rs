use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct DestroyResponse {
    pub ok: bool,
}

#[derive(Deserialize, Debug)]
pub struct ChangeResponse {
    pub ok: bool,
    pub id: String,
    pub rev: String,
}
