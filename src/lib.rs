#![doc = include_str!("../README.md")]

use axum::{
    extract::{rejection::PathRejection, FromRequest, FromRequestParts, Path, Query},
    http::Request,
};
use serde::Deserialize;

/// Validated Option allows your extractors to use either a valid
/// `T`, or a missing `T`, but reject an invalid `T`, based on the
/// definition of 'missing' for those items.
///
/// For this to work, the crate that defines the extractor must
/// implement `FromRequestPartsOptional` for the extractor.
pub struct ValidOption<T>(pub Option<T>);

#[axum::async_trait]
impl<S, T> FromRequestParts<S> for ValidOption<T>
where
    S: Send + Sync,
    T: FromRequestPartsOptional<S> + Send + Sync,
    T::Rejection: Send + Sync,
{
    type Rejection = <T as FromRequestParts<S>>::Rejection;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        T::option_reject(T::from_request_parts(parts, state).await)
            .await
            .map(ValidOption)
    }
}

#[axum::async_trait]
impl<S, B, T> FromRequest<S, B> for ValidOption<T>
where
    S: Send + Sync,
    B: Sync + Send + 'static,
    T: FromRequestOptional<S, B> + Send + Sync,
    T::Rejection: Send + Sync,
{
    type Rejection = <T as FromRequest<S, B>>::Rejection;

    async fn from_request(req: Request<B>, body: &S) -> Result<Self, Self::Rejection> {
        T::option_reject(T::from_request(req, body).await)
            .await
            .map(ValidOption)
    }
}

#[axum::async_trait]
#[cfg_attr(
    nightly_error_messages,
    rustc_on_unimplemented(
        note = "Function argument is not a valid optional extractor. \nSee `https://docs.rs/axum-option/latest/axum-option` for details",
    )
)]
pub trait FromRequestPartsOptional<S>: FromRequestParts<S> {
    async fn option_reject(
        result: Result<Self, Self::Rejection>,
    ) -> Result<Option<Self>, Self::Rejection>;
}

#[axum::async_trait]
#[cfg_attr(
    nightly_error_messages,
    rustc_on_unimplemented(
        note = "Function argument is not a valid optional extractor. \nSee `https://docs.rs/axum-option/latest/axum-option` for details",
    )
)]
pub trait FromRequestOptional<S, B>: FromRequest<S, B> {
    async fn option_reject(
        result: Result<Self, Self::Rejection>,
    ) -> Result<Option<Self>, Self::Rejection>;
}

/// note: requires a PR into axum first
///
#[axum::async_trait]
impl<S: Send + Sync, T> FromRequestPartsOptional<S> for Query<T>
where
    T: std::fmt::Debug + for<'de> Deserialize<'de> + Send + Sync,
{
    async fn option_reject(
        result: Result<Self, Self::Rejection>,
    ) -> Result<Option<Self>, Self::Rejection> {
        println!("{:?}", result);
        match result {
            Ok(query) => Ok(Some(query)),
            // Err(QueryRejection::MissingQueryString(_)) => Ok(None),
            Err(e) => Err(e),
        }
    }
}

/// note: requires a PR into axum first
///
#[axum::async_trait]
impl<S: Send + Sync, T: Send + Sync + for<'de> Deserialize<'de>> FromRequestPartsOptional<S>
    for Path<T>
{
    async fn option_reject(
        result: Result<Self, Self::Rejection>,
    ) -> Result<Option<Self>, Self::Rejection> {
        match result {
            Ok(p) => Ok(Some(p)),
            Err(PathRejection::MissingPathParams(_)) => Ok(None),
            Err(e) => Err(e),
        }
    }
}

#[cfg(feature = "headers")]
mod headers {
    use axum::extract::rejection::TypedHeaderRejectionReason;
    use axum::headers::Header;
    use axum::TypedHeader;

    use super::FromRequestPartsOptional;

    #[axum::async_trait]
    impl<S: Send + Sync, T: Header + Send + Sync> FromRequestPartsOptional<S> for TypedHeader<T> {
        async fn option_reject(
            result: Result<Self, Self::Rejection>,
        ) -> Result<Option<Self>, Self::Rejection> {
            match result {
                Ok(header) => Ok(Some(header)),
                Err(e) if matches!(e.reason(), TypedHeaderRejectionReason::Missing) => Ok(None),
                Err(e) => Err(e),
            }
        }
    }
}

#[cfg(feature = "jwt-authorizer")]
mod jwt_authorizer {
    use jwt_authorizer::{error::AuthError, JwtClaims};
    use serde::Deserialize;

    use super::FromRequestPartsOptional;

    #[axum::async_trait]
    impl<S, T> FromRequestPartsOptional<S> for JwtClaims<T>
    where
        S: Send + Sync,
        T: Clone + 'static + Sync + Send + for<'de> Deserialize<'de>,
    {
        async fn option_reject(
            result: Result<Self, Self::Rejection>,
        ) -> Result<Option<Self>, Self::Rejection> {
            match result {
                Ok(claims) => Ok(Some(claims)),
                Err(AuthError::MissingToken()) => Ok(None),
                Err(e) => Err(e),
            }
        }
    }
}
