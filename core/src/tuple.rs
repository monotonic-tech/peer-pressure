//! [Validate] and [Resolve] implementations for tuples.

use crate::{Validate, ValidateFrom};

#[cfg(feature = "async")]
use crate::{Resolve, ResolveFrom};

macro_rules! valid_for_tuple {
    ($T1:ty) => {};
    ($T1:ident, $($T:ident),+) => {
        valid_for_tuple!(@inner $T1, $($T),+);
        valid_for_tuple!($($T),+);
    };
    (@inner $($T:ident),+) => {
        #[allow(non_snake_case)]
        impl<Ctx: ?Sized, Err, $($T: Validate<Context = Ctx, Error = Err>),+> Validate for ($($T),+) {
            type Context = Ctx;
            type Output = ($(<$T as Validate>::Output),+);
            type Error = Err;

            fn validate_in_context(self, ctx: &Self::Context) -> Result<Self::Output, Self::Error> {
                let ($($T),+) = self;

                Ok(($($T.validate_in_context(ctx)?),+))
            }
        }
    };
}

macro_rules! valid_from_for_tuple {
    ($T1:ident $R1:ident) => {};
    ($T1:ident $R1:ident, $($T:ident $R:ident),+) => {
        valid_from_for_tuple!(@inner $T1 $R1, $($T $R),+);
        valid_from_for_tuple!($($T $R),+);
    };
    (@inner $($T:ident $R:ident),+) => {
        #[allow(non_snake_case)]
        impl<Ctx: ?Sized, Err, $($T,)+ $($R,)+> ValidateFrom<($($R),+)> for ($($T),+)
        where
            $($T: ValidateFrom<$R, Context = Ctx, Error = Err>,)+
        {
            type Context = Ctx;
            type Error = Err;

            fn validate_from_in_context(
                raw: ($($R),+),
                ctx: &Self::Context,
            ) -> Result<Self, Self::Error> {
                let ($($R,)+) = raw;

                Ok(($($T::validate_from_in_context($R, ctx)?),+))
            }
        }
    };
}

#[cfg(feature = "async")]
macro_rules! resolve_for_tuple {
    ($T1:ty) => {};
    ($T1:ident, $($T:ident),+) => {
        resolve_for_tuple!(@inner $T1, $($T),+);
        resolve_for_tuple!($($T),+);
    };
    (@inner $($T:ident),+) => {
        #[allow(non_snake_case)]
        impl<Ctx: ?Sized, Err, $($T: Resolve<Context = Ctx, Error = Err>),+> Resolve for ($($T),+) {
            type Context = Ctx;
            type Output = ($(<$T as Resolve>::Output),+);
            type Error = Err;

            async fn resolve_in_context(
                self,
                ctx: &Self::Context,
            ) -> Result<Self::Output, Self::Error> {
                let ($($T),+) = self;

                Ok(($($T.resolve_in_context(ctx).await?),+))
            }
        }
    };
}

#[cfg(feature = "async")]
macro_rules! resolve_from_for_tuple {
    ($T1:ident $R1:ident) => {};
    ($T1:ident $R1:ident, $($T:ident $R:ident),+) => {
        resolve_from_for_tuple!(@inner $T1 $R1, $($T $R),+);
        resolve_from_for_tuple!($($T $R),+);
    };
    (@inner $($T:ident $R:ident),+) => {
        #[allow(non_snake_case)]
        impl<Ctx: ?Sized, Err, $($T,)+ $($R,)+> ResolveFrom<($($R),+)> for ($($T),+)
        where
            $($T: ResolveFrom<$R, Context = Ctx, Error = Err>,)+
        {
            type Context = Ctx;
            type Error = Err;

            async fn resolve_from_in_context(
                raw: ($($R),+),
                ctx: &Self::Context,
            ) -> Result<Self, Self::Error> {
                let ($($R,)+) = raw;

                Ok(($($T::resolve_from_in_context($R, ctx).await?),+))
            }
        }
    };
}

// 26-tuple max should be reasonable, right?
valid_for_tuple!(
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z
);

valid_from_for_tuple!(
    A RA, B RB, C RC, D RD, E RE, F RF, G RG, H RH, I RI, J RJ, K RK, L RL, M RM,
    N RN, O RO, P RP, Q RQ, R RR, S RS, T RT, U RU, V RV, W RW, X RX, Y RY, Z RZ
);

#[cfg(feature = "async")]
resolve_for_tuple!(
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z
);

#[cfg(feature = "async")]
resolve_from_for_tuple!(
    A RA, B RB, C RC, D RD, E RE, F RF, G RG, H RH, I RI, J RJ, K RK, L RL, M RM,
    N RN, O RO, P RP, Q RQ, R RR, S RS, T RT, U RU, V RV, W RW, X RX, Y RY, Z RZ
);
