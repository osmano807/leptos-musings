mod helpers;
pub mod typed_path;

use app_macros::TypedPath;
use derive_new::new;
use leptos::Params;
use leptos_router::params::Params;
use serde::Deserialize;

#[derive(TypedPath, Deserialize, new)]
#[typed_path("/")]
pub struct Home;

#[derive(TypedPath, Deserialize, new)]
#[typed_path("/help")]
pub struct Help;

#[derive(Params, PartialEq, Clone, Debug, TypedPath, Deserialize, new)]
#[typed_path("/some/:parameter")]
pub struct SomeParameterPath {
    pub parameter: String,
}