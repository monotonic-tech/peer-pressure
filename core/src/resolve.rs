//! Async counterparts of [Validate](crate::Validate) and [ValidateFrom](crate::ValidateFrom).

#![allow(async_fn_in_trait)]

/// Async analogue of [Validate](crate::Validate).
pub trait Resolve: Sized {
    /// Immutable context of the resolution.
    ///
    /// Use `()` for context-free checks.
    type Context: ?Sized;
    /// Resolved form of `Self`.
    type Output;
    /// Resolution error.
    type Error;

    /// Resolves `self` in given context, returning either a valid `Output` or `Error`.
    async fn resolve_in_context(self, ctx: &Self::Context) -> Result<Self::Output, Self::Error>;

    /// Resolves `self`, returning either a valid `Output` or `Error`.
    ///
    /// Available only for types with context-free resolution (ie. for which `Context: From<()>`).
    async fn resolve(self) -> Result<Self::Output, Self::Error>
    where
        Self::Context: From<()>,
    {
        self.resolve_in_context(&().into()).await
    }
}

/// Async analogue of [ValidateFrom](crate::ValidateFrom).
pub trait ResolveFrom<Raw>: Sized {
    /// Immutable context of the resolution.
    ///
    /// Use `()` for context-free checks.
    type Context: ?Sized;
    /// Resolution error.
    type Error;

    /// Resolves `raw` in given context, returning either a valid `Self` or `Error`.
    async fn resolve_from_in_context(raw: Raw, ctx: &Self::Context) -> Result<Self, Self::Error>;

    /// Resolves `raw`, returning either a valid `Self` or `Error`.
    ///
    /// Available only for types with context-free resolution (ie. for which `Context: From<()>`).
    async fn resolve_from(raw: Raw) -> Result<Self, Self::Error>
    where
        Self::Context: From<()>,
    {
        Self::resolve_from_in_context(raw, &().into()).await
    }
}
