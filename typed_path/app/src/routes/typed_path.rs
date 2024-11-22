use std::{any::type_name, fmt};

use http::Uri;
use serde::Serialize;

/// A type safe path
///
/// This is used to generate type safe paths for Leptos and in app routing
///
pub trait TypedPath: std::fmt::Display {
    /// The path with optional captures such as `/users/:id`.
    const PATH: &'static str;

    fn raw_path() -> &'static str {
        Self::PATH
    }

    fn to_uri(&self) -> Uri {
        // * unwrap is safe because the path is static and known at compile time
        self.to_string().parse().unwrap()
    }

    fn with_query_params<T>(self, params: T) -> WithQueryParams<Self, T>
    where
        T: Serialize,
        Self: Sized,
    {
        WithQueryParams { path: self, params }
    }
}

/// A [`TypedPath`] with query params.
///
/// See [`TypedPath::with_query_params`] for more details.
#[derive(Debug, Clone, Copy)]
pub struct WithQueryParams<P, T> {
    path: P,
    params: T,
}

impl<P, T> fmt::Display for WithQueryParams<P, T>
where
    P: TypedPath,
    T: Serialize,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = self.path.to_string();
        if !out.contains('?') {
            out.push('?');
        }
        let mut urlencoder = form_urlencoded::Serializer::new(&mut out);
        self.params
            .serialize(serde_html_form::ser::Serializer::new(&mut urlencoder))
            .unwrap_or_else(|err| {
                panic!(
                    "failed to URL encode value of type `{}`: {}",
                    type_name::<T>(),
                    err
                )
            });
        f.write_str(&out)?;

        Ok(())
    }
}

impl<P, T> TypedPath for WithQueryParams<P, T>
where
    P: TypedPath,
    T: Serialize,
{
    const PATH: &'static str = P::PATH;
}
