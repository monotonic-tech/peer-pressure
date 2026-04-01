//! Iterator combinators for [Validate].

use crate::Validate;

#[cfg(feature = "async")]
use crate::Resolve;

/// Extension trait on iterators whose items implement [Validate].
#[cfg_attr(
    feature = "async",
    doc = "\nWith the `async` feature, also covers items implementing [Resolve](crate::Resolve)."
)]
#[cfg_attr(feature = "async", allow(async_fn_in_trait))]
pub trait IteratorExt: IntoIterator {
    /// Validates every item within given context, short-circuiting on the first error.
    fn validate_all_in_context<I>(
        self,
        ctx: &<Self::Item as Validate>::Context,
    ) -> Result<I, <Self::Item as Validate>::Error>
    where
        Self::Item: Validate,
        I: Extend<<Self::Item as Validate>::Output> + Default;

    /// Context-free variant of [validate_all_in_context], available when item `Context = ()`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use peer_pressure::{Validate, IteratorExt};
    ///
    /// struct RawNum(i32);
    /// struct Positive(i32);
    ///
    /// impl Validate for RawNum {
    ///     type Context = ();
    ///     type Output = Positive;
    ///     type Error = &'static str;
    ///     fn validate_in_context(self, _: &()) -> Result<Positive, &'static str> {
    ///         if self.0 > 0 { Ok(Positive(self.0)) } else { Err("non-positive") }
    ///     }
    /// }
    ///
    /// let raws = [RawNum(1), RawNum(2), RawNum(3)];
    /// let out: Result<Vec<_>, _> = raws.validate_all();
    /// assert!(out.is_ok());
    /// ```
    ///
    /// [validate_all_in_context]: IteratorExt::validate_all_in_context
    fn validate_all<I>(self) -> Result<I, <Self::Item as Validate>::Error>
    where
        Self::Item: Validate,
        <Self::Item as Validate>::Context: From<()>,
        I: Extend<<Self::Item as Validate>::Output> + Default;

    /// Validates every item within given context, partitioning successes into `I1` and errors into `I2`.
    fn partition_validated_in_context<I1, I2>(
        self,
        ctx: &<Self::Item as Validate>::Context,
    ) -> (I1, I2)
    where
        Self::Item: Validate,
        I1: Extend<<Self::Item as Validate>::Output> + Default,
        I2: Extend<<Self::Item as Validate>::Error> + Default;

    /// Context-free variant of [partition_validated_in_context], available when items' `Context = ()`.
    ///
    /// Input order is preserved within each output collection.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use peer_pressure::{Validate, IteratorExt};
    ///
    /// struct RawNum(i32);
    /// struct Positive(i32);
    ///
    /// impl Validate for RawNum {
    ///     type Context = ();
    ///     type Output = Positive;
    ///     type Error = &'static str;
    ///     fn validate_in_context(self, _: &()) -> Result<Positive, &'static str> {
    ///         if self.0 > 0 { Ok(Positive(self.0)) } else { Err("non-positive") }
    ///     }
    /// }
    ///
    /// let raws = [RawNum(1), RawNum(-1), RawNum(2)];
    /// let (oks, errs): (Vec<Positive>, Vec<&'static str>) = raws.partition_validated();
    /// assert_eq!(oks.len(), 2);
    /// assert_eq!(errs, vec!["non-positive"]);
    /// ```
    ///
    /// [partition_validated_in_context]: IteratorExt::partition_validated_in_context
    fn partition_validated<I1, I2>(self) -> (I1, I2)
    where
        Self::Item: Validate,
        <Self::Item as Validate>::Context: From<()>,
        I1: Extend<<Self::Item as Validate>::Output> + Default,
        I2: Extend<<Self::Item as Validate>::Error> + Default;

    /// Resolves every item within given context, short-circuiting on the first error.
    #[cfg(feature = "async")]
    async fn resolve_all_in_context<I>(
        self,
        ctx: &<Self::Item as Resolve>::Context,
    ) -> Result<I, <Self::Item as Resolve>::Error>
    where
        Self: Sized,
        Self::Item: Resolve,
        I: Extend<<Self::Item as Resolve>::Output> + Default;

    /// Context-free variant of [resolve_all_in_context], available when items' `Context = ()`.
    ///
    /// [resolve_all_in_context]: IteratorExt::resolve_all_in_context
    #[cfg(feature = "async")]
    async fn resolve_all<I>(self) -> Result<I, <Self::Item as Resolve>::Error>
    where
        Self: Sized,
        Self::Item: Resolve,
        <Self::Item as Resolve>::Context: From<()>,
        I: Extend<<Self::Item as Resolve>::Output> + Default;

    /// Resolves every item within given context, partitioning successes into `I1` and errors into `I2`.
    #[cfg(feature = "async")]
    async fn partition_resolved_in_context<I1, I2>(
        self,
        ctx: &<Self::Item as Resolve>::Context,
    ) -> (I1, I2)
    where
        Self: Sized,
        Self::Item: Resolve,
        I1: Extend<<Self::Item as Resolve>::Output> + Default,
        I2: Extend<<Self::Item as Resolve>::Error> + Default;

    /// Context-free variant of [partition_resolved_in_context], available when items' `Context = ()`.
    ///
    /// [partition_resolved_in_context]: IteratorExt::partition_resolved_in_context
    #[cfg(feature = "async")]
    async fn partition_resolved<I1, I2>(self) -> (I1, I2)
    where
        Self: Sized,
        Self::Item: Resolve,
        <Self::Item as Resolve>::Context: From<()>,
        I1: Extend<<Self::Item as Resolve>::Output> + Default,
        I2: Extend<<Self::Item as Resolve>::Error> + Default;
}

impl<Iter: IntoIterator> IteratorExt for Iter {
    fn validate_all_in_context<I>(
        self,
        ctx: &<Self::Item as Validate>::Context,
    ) -> Result<I, <Self::Item as Validate>::Error>
    where
        Self::Item: Validate,
        I: Extend<<Self::Item as Validate>::Output> + Default,
    {
        let mut out = I::default();
        for item in self {
            out.extend(Some(item.validate_in_context(ctx)?));
        }
        Ok(out)
    }

    fn validate_all<I>(self) -> Result<I, <Self::Item as Validate>::Error>
    where
        Self::Item: Validate,
        <Self::Item as Validate>::Context: From<()>,
        I: Extend<<Self::Item as Validate>::Output> + Default,
    {
        self.validate_all_in_context(&().into())
    }

    fn partition_validated_in_context<I1, I2>(
        self,
        ctx: &<Self::Item as Validate>::Context,
    ) -> (I1, I2)
    where
        Self::Item: Validate,
        I1: Extend<<Self::Item as Validate>::Output> + Default,
        I2: Extend<<Self::Item as Validate>::Error> + Default,
    {
        let mut oks = I1::default();
        let mut errs = I2::default();
        for item in self {
            match item.validate_in_context(ctx) {
                Ok(v) => oks.extend(Some(v)),
                Err(e) => errs.extend(Some(e)),
            }
        }
        (oks, errs)
    }

    fn partition_validated<I1, I2>(self) -> (I1, I2)
    where
        Self::Item: Validate,
        <Self::Item as Validate>::Context: From<()>,
        I1: Extend<<Self::Item as Validate>::Output> + Default,
        I2: Extend<<Self::Item as Validate>::Error> + Default,
    {
        self.partition_validated_in_context(&().into())
    }

    #[cfg(feature = "async")]
    async fn resolve_all_in_context<I>(
        self,
        ctx: &<Self::Item as Resolve>::Context,
    ) -> Result<I, <Self::Item as Resolve>::Error>
    where
        Self: Sized,
        Self::Item: Resolve,
        I: Extend<<Self::Item as Resolve>::Output> + Default,
    {
        let mut out = I::default();
        for item in self {
            out.extend(Some(item.resolve_in_context(ctx).await?));
        }
        Ok(out)
    }

    #[cfg(feature = "async")]
    async fn resolve_all<I>(self) -> Result<I, <Self::Item as Resolve>::Error>
    where
        Self: Sized,
        Self::Item: Resolve,
        <Self::Item as Resolve>::Context: From<()>,
        I: Extend<<Self::Item as Resolve>::Output> + Default,
    {
        self.resolve_all_in_context(&().into()).await
    }

    #[cfg(feature = "async")]
    async fn partition_resolved_in_context<I1, I2>(
        self,
        ctx: &<Self::Item as Resolve>::Context,
    ) -> (I1, I2)
    where
        Self: Sized,
        Self::Item: Resolve,
        I1: Extend<<Self::Item as Resolve>::Output> + Default,
        I2: Extend<<Self::Item as Resolve>::Error> + Default,
    {
        let mut oks = I1::default();
        let mut errs = I2::default();
        for item in self {
            match item.resolve_in_context(ctx).await {
                Ok(v) => oks.extend(Some(v)),
                Err(e) => errs.extend(Some(e)),
            }
        }
        (oks, errs)
    }

    #[cfg(feature = "async")]
    async fn partition_resolved<I1, I2>(self) -> (I1, I2)
    where
        Self: Sized,
        Self::Item: Resolve,
        <Self::Item as Resolve>::Context: From<()>,
        I1: Extend<<Self::Item as Resolve>::Output> + Default,
        I2: Extend<<Self::Item as Resolve>::Error> + Default,
    {
        self.partition_resolved_in_context(&().into()).await
    }
}
