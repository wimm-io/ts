use std::sync::OnceLock;

use native_db::{Models, ToKey, native_db};
use native_model::{Model, native_model};
use serde::{Deserialize, Serialize};

use crate::app::Todo;

static MODELS: OnceLock<Models> = OnceLock::new();

pub fn get_models() -> &'static Models {
    MODELS.get_or_init(|| {
        let mut models = Models::new();
        models
            .define::<Todo>()
            .expect("failed to define `Todo` model");
        models
    })
}

pub mod v1 {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
    #[native_model(id = 1, version = 1)]
    #[native_db]
    pub struct Todo {
        #[primary_key]
        pub(crate) id: String,
        pub(crate) title: String,
        pub(crate) note: Option<String>,
        pub(crate) completed: bool,
    }
}
