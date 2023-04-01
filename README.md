# axum-option

> Better handling of optional extractors in axum.

This crate exposes a trait, and an extractor, that allows
you to extract a potentially-missing value in your handlers.

Crates may implement `FromRequestPartsOptional` for their
extractors to signify that the extractor can have a missing value,
and then the `ValidOption` extractor can be used to extract
either a valid `T`, or a missing `T`, but reject an invalid `T`.

## How does this differ from using `Option<T>` directly?

Unfortunately, axum's default 'Option' handler means either
'T passed validation' or 'T failed validation', essentially
meaning the same as `Result<T, T::Rejection>.ok()`, which
is typically not what you want to do. It means that supporting
handlers that, for example, need either a valid cookie, or
no cookie at all, requires per-crate newtype wrapper
boilerplate and knowledge over the internal workings of the
crate itself. `ValidOption` solves this M + N problem.

Now you can do `ValidOption<Session>` and get either a valid
session, or no session at all, but reject an invalid session.
