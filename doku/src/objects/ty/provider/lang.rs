use crate::*;

providers! {
    for &T where (T: ty::Provider + ?Sized) =>
        T::ty().def

    for &mut T where (T: ty::Provider + ?Sized) =>
        T::ty().def

    for &[T] where (T: ty::Provider) =>
        ty::Def::Array {
            ty: Box::new(T::ty()),
            size: None,
        }

    for &mut [T] where (T: ty::Provider) =>
        ty::Def::Array {
            ty: Box::new(T::ty()),
            size: None,
        }

    for bool => ty::Def::Bool

    for char => ty::Def::String
    for str  => ty::Def::String

    for u8    => ty::Def::Integer
    for u16   => ty::Def::Integer
    for u32   => ty::Def::Integer
    for u64   => ty::Def::Integer
    for u128  => ty::Def::Integer
    for usize => ty::Def::Integer

    for i8    => ty::Def::Integer
    for i16   => ty::Def::Integer
    for i32   => ty::Def::Integer
    for i64   => ty::Def::Integer
    for i128  => ty::Def::Integer
    for isize => ty::Def::Integer

    for f32 => ty::Def::Float
    for f64 => ty::Def::Float

    for [T; N] where (T: ty::Provider, const N: usize) =>
        ty::Def::Array {
            ty: Box::new(T::ty()),
            size: Some(N),
        }

    for (A,) where (A: ty::Provider) =>
        ty::Def::Tuple {
            fields: vec![A::ty()],
        }

    for (A, B) where (A: ty::Provider, B: ty::Provider) =>
        ty::Def::Tuple {
            fields: vec![A::ty(), B::ty()],
        }

    for (A, B, C) where (A: ty::Provider, B: ty::Provider, C: ty::Provider) =>
        ty::Def::Tuple {
            fields: vec![A::ty(), B::ty(), C::ty()],
        }

    for (A, B, C, D) where (A: ty::Provider, B: ty::Provider, C: ty::Provider, D: ty::Provider) =>
        ty::Def::Tuple {
            fields: vec![A::ty(), B::ty(), C::ty(), D::ty()],
        }

    for (A, B, C, D, E) where (A: ty::Provider, B: ty::Provider, C: ty::Provider, D: ty::Provider, E: ty::Provider) =>
        ty::Def::Tuple {
            fields: vec![A::ty(), B::ty(), C::ty(), D::ty(), E::ty()],
        }

    for (A, B, C, D, E, F) where (A: ty::Provider, B: ty::Provider, C: ty::Provider, D: ty::Provider, E: ty::Provider, F: ty::Provider) =>
        ty::Def::Tuple {
            fields: vec![A::ty(), B::ty(), C::ty(), D::ty(), E::ty(), F::ty()],
        }

    for (A, B, C, D, E, F, G) where (A: ty::Provider, B: ty::Provider, C: ty::Provider, D: ty::Provider, E: ty::Provider, F: ty::Provider, G: ty::Provider) =>
        ty::Def::Tuple {
            fields: vec![A::ty(), B::ty(), C::ty(), D::ty(), E::ty(), F::ty(), G::ty()],
        }

    for (A, B, C, D, E, F, G, H) where (A: ty::Provider, B: ty::Provider, C: ty::Provider, D: ty::Provider, E: ty::Provider, F: ty::Provider, G: ty::Provider, H: ty::Provider) =>
        ty::Def::Tuple {
            fields: vec![A::ty(), B::ty(), C::ty(), D::ty(), E::ty(), F::ty(), G::ty(), H::ty()],
        }
}
