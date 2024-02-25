pub mod common;
pub mod rollup;
pub mod prelude {
    pub use ssal_core::{
        axum::{
            extract::{Query, State},
            http::StatusCode,
            response::IntoResponse,
            Json,
        },
        error::{Error, WrapError},
        rand::{self, seq::SliceRandom},
        serde::{Deserialize, Serialize},
        tracing,
        types::*,
    };
    pub use ssal_database::{Database, Lock};
}
