//! This module provides the `SignalResult` type, which is designed to handle asynchronous
//! operations in Leptos applications, particularly when working with Signals, Resources, and Memos.

use crate::AppError;
pub(crate) use frunk::hlist;
pub(crate) use frunk::hlist_pat;
use frunk::prelude::*;
use frunk::{HCons, HNil};
use leptos::prelude::*;

pub mod macros;
pub use macros::signal_result_view;
pub use macros::signal_result_view_with_suspense;

/// `SignalResult` is a type that represents the state of asynchronous operations in Leptos.
///
/// It is particularly useful for handling the return values of `get()` methods on Leptos
/// primitives such as Signals, Resources, and Memos.
///
/// # States
///
/// - `Loading`: The operation is still in progress.
/// - `Ok(T)`: The operation completed successfully with a value of type `T`.
/// - `Err(Vec<AppError>)`: The operation failed with one or more errors.
///
/// # Type Parameters
///
/// - `T`: A heterogeneous list (`HList`) representing the successful result(s) of the operation(s).
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// #![feature(assert_matches)]
/// use std::assert_matches::assert_matches;
/// use app::helpers::signal_result::SignalResult;
/// use app::errors::AppError;
/// use frunk::{hlist, hlist_pat};
/// use leptos::prelude::*;
/// # tokio_test::block_on(async move {
/// # tokio::task::LocalSet::new().run_until(async move {
///
/// let resource = Resource::new(|| (), |_| async { Ok::<_, AppError>(42) });
/// let result = SignalResult::from_option_result(resource.get());
///
/// match result {
///     SignalResult::Ok(hlist_pat!(value)) => println!("Value: {}", value),
///     SignalResult::Err(ref errors) => println!("Errors: {:?}", errors),
///     SignalResult::Loading => println!("Still loading..."),
/// }
///
/// assert_matches!(result, SignalResult::Ok(hlist![42]));
/// # });
/// # });
/// ```
#[derive(Debug)]
pub enum SignalResult<T>
where
    T: HList,
{
    Loading,
    Ok(T),
    Err(Vec<AppError>),
}

impl<H, T> SignalResult<HCons<H, T>>
where
    HCons<H, T>: HList,
{
    /// Combines this `SignalResult` with another, producing a new `SignalResult` that contains
    /// the results of both if they are both `Ok`, or the appropriate error or loading state otherwise.
    ///
    /// # Type Parameters
    ///
    /// - `H2`, `T2`: The type of the other `SignalResult` to combine with.
    /// - `HResult`: The resulting type after combination.
    ///
    /// # Examples
    ///
    /// ```rust
    /// #![feature(assert_matches)]
    /// use std::assert_matches::assert_matches;
    /// use app::helpers::signal_result::SignalResult;
    /// use app::errors::AppError;
    /// use frunk::{hlist, hlist_pat};
    /// use leptos::prelude::*;
    /// # tokio_test::block_on(async move {
    /// # tokio::task::LocalSet::new().run_until(async move {
    ///
    /// let resource1 = Resource::new(|| (), |_| async { Ok::<_, AppError>(42) });
    /// let resource2 = Resource::new(|| (), |_| async { Ok::<_, AppError>(true) });
    ///
    /// let result1 = SignalResult::from_option_result(resource1.get());
    /// let result2 = SignalResult::from_option_result(resource2.get());
    ///
    /// let combined = result1.combine(result2);
    ///
    /// match combined {
    ///     SignalResult::Ok(hlist_pat!(num, boolean)) => println!("Number: {}, Bool: {}", num, boolean),
    ///     SignalResult::Err(ref errors) => println!("Errors: {:?}", errors),
    ///     SignalResult::Loading => println!("Still loading..."),
    /// }
    ///
    /// assert_matches!(combined, SignalResult::Ok(hlist![42, true]));
    /// # });
    /// # });
    /// ```
    pub fn combine<H2, T2, HResult>(
        self,
        other: SignalResult<HCons<H2, T2>>,
    ) -> SignalResult<HResult>
    where
        HCons<H2, T2>: HList,
        HResult: HList,
        HCons<H, T>: std::ops::Add<HCons<H2, T2>, Output = HResult>,
    {
        combine(self, other)
    }
}

impl<H> SignalResult<HCons<H, HNil>> {
    /// Creates a `SignalResult` from an `Option<Result<H, AppError>>`.
    ///
    /// This method is particularly useful when working with Leptos Resources.
    /// Specifically, it's designed to handle the return type of `Resource<Result<T, AppError>>::get()`,
    /// which typically returns an `Option<Result<T, AppError>>`.
    ///
    /// Note: The `Option` in the return type of `Resource::get()` is used to handle the Loading state.
    /// When the resource is still loading, `get()` returns `None`, which this method translates into
    /// `SignalResult::Loading`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// #![feature(assert_matches)]
    /// use std::assert_matches::assert_matches;
    /// use app::helpers::signal_result::SignalResult;
    /// use app::errors::AppError;
    /// use frunk::hlist;
    /// use leptos::prelude::*;
    /// # tokio_test::block_on(async move {
    /// # tokio::task::LocalSet::new().run_until(async move {
    ///
    /// let resource: Resource<Result<i32, AppError>> = Resource::new(|| (), |_| async { Ok(42) });
    /// let result = SignalResult::from_option_result(resource.get());
    ///
    /// assert_matches!(result, SignalResult::Ok(hlist![42]));
    /// # });
    /// # });
    /// ```
    pub fn from_option_result(value: Option<Result<H, AppError>>) -> Self {
        match value {
            Some(Ok(t)) => SignalResult::Ok(hlist![t]),
            Some(Err(e)) => SignalResult::Err(vec![e]),
            None => SignalResult::Loading,
        }
    }

    /// Creates a `SignalResult` from a `Result<H, AppError>`.
    ///
    /// This method is particularly useful when working with `Memo<Result<T, AppError>>`,
    /// as the `get()` method on such memos typically returns `Result<T, AppError>`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// #![feature(assert_matches)]
    /// use std::assert_matches::assert_matches;
    /// use app::helpers::signal_result::SignalResult;
    /// use app::errors::AppError;
    /// use frunk::hlist;
    /// use leptos::prelude::*;
    /// # tokio_test::block_on(async move {
    /// # tokio::task::LocalSet::new().run_until(async move {
    ///
    /// let memo: Memo<Result<i32, AppError>> = Memo::new(move |_| Ok(42));
    /// let signal_result = SignalResult::from_result(memo.get());
    ///
    /// assert_matches!(signal_result, SignalResult::Ok(hlist![42]));
    /// # });
    /// # });
    /// ```
    pub fn from_result(value: Result<H, AppError>) -> Self {
        match value {
            Ok(t) => SignalResult::Ok(hlist![t]),
            Err(e) => SignalResult::Err(vec![e]),
        }
    }

    /// Creates a `SignalResult` from an `Option<H>`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// #![feature(assert_matches)]
    /// use std::assert_matches::assert_matches;
    /// use app::helpers::signal_result::SignalResult;
    /// use frunk::hlist;
    /// let option = Some(42);
    /// let signal_result = SignalResult::from_option(option);
    ///
    /// assert_matches!(signal_result, SignalResult::Ok(hlist![42]));
    /// ```
    pub fn from_option(value: Option<H>) -> Self {
        match value {
            Some(t) => SignalResult::Ok(hlist![t]),
            None => SignalResult::Loading,
        }
    }
}

impl<H> From<Resource<Result<H, AppError>>> for SignalResult<HCons<H, HNil>>
where
    H: Clone + Send + Sync,
{
    fn from(value: Resource<Result<H, AppError>>) -> Self {
        SignalResult::from_option_result(value.get())
    }
}

impl<H> From<Memo<Result<H, AppError>>> for SignalResult<HCons<H, HNil>>
where
    H: Clone + Send + Sync + 'static,
{
    fn from(value: Memo<Result<H, AppError>>) -> Self {
        SignalResult::from_result(value.get())
    }
}

/// Combines two `SignalResult`s into a single `SignalResult`.
///
/// This function is used internally by the `combine` method.
fn combine<H0, T0, H1, T1, HResult>(
    right: SignalResult<HCons<H0, T0>>,
    left: SignalResult<HCons<H1, T1>>,
) -> SignalResult<HResult>
where
    HCons<H0, T0>: HList,
    HCons<H1, T1>: HList,
    HResult: HList,
    HCons<H0, T0>: std::ops::Add<HCons<H1, T1>, Output = HResult>,
{
    // Until all the signals are loaded, we return loading.
    // If one of the signals returns an error, we return the error.
    // If both signals return Ok, we return the result of combining the two results.
    match (right, left) {
        (SignalResult::Loading, _) => SignalResult::Loading,
        (_, SignalResult::Loading) => SignalResult::Loading,
        (SignalResult::Ok(t), SignalResult::Ok(t_other)) => SignalResult::Ok(t.extend(t_other)),
        (SignalResult::Err(e), SignalResult::Err(e_other)) => {
            SignalResult::Err(e.into_iter().chain(e_other).collect())
        }
        (SignalResult::Ok(_), SignalResult::Err(e)) => SignalResult::Err(e.to_vec()),
        (SignalResult::Err(e), SignalResult::Ok(_)) => SignalResult::Err(e.to_vec()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use frunk::{hlist, hlist_pat, HCons, HNil};
    use std::assert_matches::assert_matches;

    #[test]
    fn test_combine_loading() {
        let a: SignalResult<HCons<i32, HNil>> = SignalResult::Loading;
        let b: SignalResult<HCons<i32, HNil>> = SignalResult::Ok(hlist![1]);
        let result: SignalResult<HCons<i32, HCons<i32, HNil>>> = combine(a, b);
        assert_matches!(result, SignalResult::Loading);

        let a: SignalResult<HCons<i32, HNil>> = SignalResult::Ok(hlist![1]);
        let b: SignalResult<HCons<i32, HNil>> = SignalResult::Loading;
        let result: SignalResult<HCons<i32, HCons<i32, HNil>>> = combine(a, b);
        assert_matches!(result, SignalResult::Loading);
    }

    #[test]
    fn test_combine_ok() {
        let a = SignalResult::Ok(hlist![1]);
        let b = SignalResult::Ok(hlist![2.0]);
        let result: SignalResult<HCons<i32, HCons<f64, HNil>>> = combine(a, b);
        assert_matches!(result, SignalResult::Ok(_));
        if let SignalResult::Ok(hlist_pat!(x, y)) = result {
            assert_eq!(x, 1);
            assert_eq!(y, 2.0);
        }
    }

    #[test]
    fn test_combine_err() {
        let a: SignalResult<HCons<i32, HNil>> = SignalResult::Err(vec![AppError::PageNotFound]);
        let b: SignalResult<HCons<i32, HNil>> = SignalResult::Err(vec![AppError::PageNotFound]);
        let result: SignalResult<HCons<i32, HCons<i32, HNil>>> = combine(a, b);
        assert_matches!(result, SignalResult::Err(_));
        if let SignalResult::Err(errors) = result {
            assert_eq!(errors.len(), 2);
        }

        let a: SignalResult<HCons<i32, HNil>> = SignalResult::Ok(hlist![1]);
        let b: SignalResult<HCons<i32, HNil>> = SignalResult::Err(vec![AppError::PageNotFound]);
        let result: SignalResult<HCons<i32, HCons<i32, HNil>>> = combine(a, b);
        assert_matches!(result, SignalResult::Err(_));
        if let SignalResult::Err(errors) = result {
            assert_eq!(errors.len(), 1);
        }

        let a: SignalResult<HCons<i32, HNil>> = SignalResult::Err(vec![AppError::PageNotFound]);
        let b: SignalResult<HCons<i32, HNil>> = SignalResult::Ok(hlist![1]);
        let result: SignalResult<HCons<i32, HCons<i32, HNil>>> = combine(a, b);
        assert_matches!(result, SignalResult::Err(_));
        if let SignalResult::Err(errors) = result {
            assert_eq!(errors.len(), 1);
        }
    }

    #[test]
    fn test_combine() {
        let a = SignalResult::Ok(hlist![1]);
        let b = SignalResult::Ok(hlist![2.0]);
        let result: SignalResult<HCons<i32, HCons<f64, HNil>>> = a.combine(b);
        assert_matches!(result, SignalResult::Ok(_));
        if let SignalResult::Ok(hlist_pat!(x, y)) = result {
            assert_eq!(x, 1);
            assert_eq!(y, 2.0);
        }
    }

    #[test]
    fn test_from_option_result() {
        let ok_result: Option<Result<i32, AppError>> = Some(Ok(42));
        let err_result: Option<Result<i32, AppError>> = Some(Err(AppError::PageNotFound));
        let none_result: Option<Result<i32, AppError>> = None;

        assert_matches!(
            SignalResult::from_option_result(ok_result),
            SignalResult::Ok(_)
        );
        assert_matches!(
            SignalResult::from_option_result(err_result),
            SignalResult::Err(_)
        );
        assert_matches!(
            SignalResult::from_option_result(none_result),
            SignalResult::Loading
        );
    }

    #[test]
    fn test_from_result() {
        let ok_result: Result<i32, AppError> = Ok(42);
        let err_result: Result<i32, AppError> = Err(AppError::PageNotFound);

        assert_matches!(SignalResult::from_result(ok_result), SignalResult::Ok(_));
        assert_matches!(SignalResult::from_result(err_result), SignalResult::Err(_));
    }

    #[test]
    fn test_from_option() {
        let some_value: Option<i32> = Some(42);
        let none_value: Option<i32> = None;

        assert_matches!(SignalResult::from_option(some_value), SignalResult::Ok(_));
        assert_matches!(SignalResult::from_option(none_value), SignalResult::Loading);
    }
}
