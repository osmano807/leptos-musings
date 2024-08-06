//! This module contains macros for handling signal results with suspense and error handling.
//!
//! The macros in this module provide a convenient way to work with types that can be converted
//! into `SignalResult`, allowing for easy handling of loading states, successful results, and
//! errors in Leptos views.
//!
//! # Macros
//!
//! - [`signal_result_view_with_suspense!`]: Creates a view with suspense for handling types
//!   that can be converted into `SignalResult`.
//! - [`signal_result_view!`]: Creates a view for handling types that can be converted into
//!   `SignalResult` without suspense.
//!
//! # Examples
//!
//! ```rust
//! use crate::signal_result_view_with_suspense;
//!
//! #[component]
//! fn MyComponent() -> impl IntoView {
//!     let data = create_resource(/* ... */);
//!
//!     view! {
//!         {signal_result_view_with_suspense!(
//!             |data|
//!             view! {
//!                 <div>"Data loaded: " {data}</div>
//!             }
//!         )}
//!     }
//! }
//! ```
//!
//! This example demonstrates the use of `signal_result_view_with_suspense!` to handle
//! a resource that may be loading, contain an error, or have successfully loaded data.
//! The `data` parameter is a type that can be converted into `SignalResult`.
use super::*;

#[macro_export]
/// Creates a view with suspense for handling types that can be converted into `SignalResult`.
///
/// This macro simplifies the process of creating a view that handles loading states,
/// successful results, and errors for types that implement `Into<SignalResult>`. It wraps the content
/// in a `SuspenseSkeleton` component and uses the `signal_result_view!` macro
/// to handle different states.
///
/// # Arguments
///
/// * `|$($param:ident),+|` - A comma-separated list of parameters that will be passed to the view.
/// * `$ok_view:expr` - The view to be rendered when all input types are successfully converted to `SignalResult::Ok`.
///
/// # Returns
///
/// A view that handles loading, success, and error states using `SuspenseSkeleton`,
/// `ErrorReporter`, and `Skeleton` components.
///
/// # Example
///
/// ```rust
/// signal_result_view_with_suspense!(|data1, data2|
///     view! {
///         <div>"Data loaded: " {data1} ", " {data2}</div>
///     }
/// )
/// ```
macro_rules! signal_result_view_with_suspense {
    (|$($param:ident),+| $ok_view:expr) => {{
        view! {
            <SuspenseSkeleton>
                {move || $crate::signal_result_view!(
                    |$($param),+| $ok_view,
                    |errors| view! { <ErrorReporter errors /> },
                    view! { <Skeleton /> }
                )}
            </SuspenseSkeleton>
        }
    }};
}
pub use signal_result_view_with_suspense;

#[macro_export]
/// Creates a view that handles different states of types that can be converted into `SignalResult`.
///
/// This macro simplifies the process of creating a view that handles successful results,
/// errors, and loading states for types that implement `Into<SignalResult>`. It combines multiple
/// inputs and matches on the combined result to render the appropriate view.
///
/// # Arguments
///
/// * `|$first:ident $(,$rest:ident)*|` - A pattern matching one or more identifiers representing types that implement `Into<SignalResult>`.
/// * `$ok_view:expr` - The view to be rendered when all input types are successfully converted to `SignalResult::Ok`.
/// * `$error_view:expr` - A closure that takes a `Vec<AppError>` and returns a view for the error state.
/// * `$loading_view:expr` - The view to be rendered when any input type is converted to `SignalResult::Loading`.
///
/// # Returns
///
/// An `EitherOf3` enum that implements `ChooseView`, which ultimately renders one of three possible views: success, error, or loading.
///
/// # Example
///
/// ```rust
/// signal_result_view!(
///     |data1, data2|
///     view! { <div>"Data loaded: " {data1} ", " {data2}</div> },
///     |errors| view! { <ErrorComponent errors={errors} /> },
///     view! { <LoadingSpinner /> }
/// )
/// ```
macro_rules! signal_result_view {
    (|$first:ident $(,$rest:ident)*| $ok_view:expr, $error_view:expr, $loading_view:expr) => {{
        let validate = $crate::helpers::signal_result::SignalResult::from($first)
            $(.combine($crate::helpers::signal_result::SignalResult::from($rest)))*;

        match validate {
            $crate::helpers::signal_result::SignalResult::Ok(::frunk::hlist_pat!($first $(,$rest)*)) => {
                ::leptos::either::EitherOf3::A($ok_view)
            },
            $crate::helpers::signal_result::SignalResult::Err(errors) => {
                ::leptos::either::EitherOf3::B($error_view(errors))
            },
            $crate::helpers::signal_result::SignalResult::Loading => {
                ::leptos::either::EitherOf3::C($loading_view)
            }
        }
    }};
}

pub use signal_result_view;
