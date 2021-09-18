use super::*;

type_providers! {
    for &T where (T: TypeProvider + ?Sized) =>
        T::ty().def

    for &mut T where (T: TypeProvider + ?Sized) =>
        T::ty().def

    for &[T] where (T: TypeProvider) =>
        TypeDef::Array {
            ty: Box::new(T::ty()),
            size: None,
        }

    for &mut [T] where (T: TypeProvider) =>
        TypeDef::Array {
            ty: Box::new(T::ty()),
            size: None,
        }

    for bool => TypeDef::Bool

    for char => TypeDef::String
    for str  => TypeDef::String

    for u8    => TypeDef::Integer
    for u16   => TypeDef::Integer
    for u32   => TypeDef::Integer
    for u64   => TypeDef::Integer
    for u128  => TypeDef::Integer
    for usize => TypeDef::Integer

    for i8    => TypeDef::Integer
    for i16   => TypeDef::Integer
    for i32   => TypeDef::Integer
    for i64   => TypeDef::Integer
    for i128  => TypeDef::Integer
    for isize => TypeDef::Integer

    for f32 => TypeDef::Float
    for f64 => TypeDef::Float

    for [T; N] where (T: TypeProvider, const N: usize) =>
        TypeDef::Array {
            ty: Box::new(T::ty()),
            size: Some(N),
        }

    for (A,) where (A: TypeProvider) =>
        TypeDef::Tuple {
            fields: vec![A::ty()],
        }

    for (A, B) where (A: TypeProvider, B: TypeProvider) =>
        TypeDef::Tuple {
            fields: vec![A::ty(), B::ty()],
        }

    for (A, B, C) where (A: TypeProvider, B: TypeProvider, C: TypeProvider) =>
        TypeDef::Tuple {
            fields: vec![A::ty(), B::ty(), C::ty()],
        }

    for (A, B, C, D) where (A: TypeProvider, B: TypeProvider, C: TypeProvider, D: TypeProvider) =>
        TypeDef::Tuple {
            fields: vec![A::ty(), B::ty(), C::ty(), D::ty()],
        }

    for (A, B, C, D, E) where (A: TypeProvider, B: TypeProvider, C: TypeProvider, D: TypeProvider, E: TypeProvider) =>
        TypeDef::Tuple {
            fields: vec![A::ty(), B::ty(), C::ty(), D::ty(), E::ty()],
        }

    for (A, B, C, D, E, F) where (A: TypeProvider, B: TypeProvider, C: TypeProvider, D: TypeProvider, E: TypeProvider, F: TypeProvider) =>
        TypeDef::Tuple {
            fields: vec![A::ty(), B::ty(), C::ty(), D::ty(), E::ty(), F::ty()],
        }

    for (A, B, C, D, E, F, G) where (A: TypeProvider, B: TypeProvider, C: TypeProvider, D: TypeProvider, E: TypeProvider, F: TypeProvider, G: TypeProvider) =>
        TypeDef::Tuple {
            fields: vec![A::ty(), B::ty(), C::ty(), D::ty(), E::ty(), F::ty(), G::ty()],
        }

    for (A, B, C, D, E, F, G, H) where (A: TypeProvider, B: TypeProvider, C: TypeProvider, D: TypeProvider, E: TypeProvider, F: TypeProvider, G: TypeProvider, H: TypeProvider) =>
        TypeDef::Tuple {
            fields: vec![A::ty(), B::ty(), C::ty(), D::ty(), E::ty(), F::ty(), G::ty(), H::ty()],
        }
}
