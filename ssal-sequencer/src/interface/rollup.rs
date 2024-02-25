use super::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(crate = "ssal_core::serde")]
pub struct GetBlock {
    raw_tx: RawTransaction,
}

impl GetBlock {
    pub async fn handler(
        State(state): State<Database>,
        Json(payload): Json<Self>,
    ) -> Result<impl IntoResponse, Error> {
        Ok((StatusCode::OK, Json(())).into_response())
    }
}
