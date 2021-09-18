use crate::*;

document! {
    for bool
        => TypeKind::Bool.into();

    for char
        => TypeKind::String.into();

    for str
        => TypeKind::String.into();

    for f32
        => TypeKind::Float.into();

    for f64
        => TypeKind::Float.into();

    for u8
        => TypeKind::Integer.into();

    for i8
        => TypeKind::Integer.into();

    for u16
        => TypeKind::Integer.into();

    for i16
        => TypeKind::Integer.into();

    for u32
        => TypeKind::Integer.into();

    for i32
        => TypeKind::Integer.into();

    for u64
        => TypeKind::Integer.into();

    for i64
        => TypeKind::Integer.into();

    for u128
        => TypeKind::Integer.into();

    for i128
        => TypeKind::Integer.into();

    for usize
        => TypeKind::Integer.into();

    for isize
        => TypeKind::Integer.into();

    /* ----- */

    for &T where (T: Document + ?Sized)
        => T::ty();

    for &mut T where (T: Document + ?Sized)
        => T::ty();

    /* ----- */

    for [T; N] where (T: Document, const N: usize)
        => TypeKind::Array {
            ty: Box::new(T::ty()),
            size: Some(N),
        }.into();

    for &[T] where (T: Document)
        => TypeKind::Array {
            ty: Box::new(T::ty()),
            size: None,
        }.into();

    for &mut [T] where (T: Document)
        => <&[T]>::ty();

    /* ----- */

    for (A,) where (A: Document)
        => TypeKind::Tuple {
            fields: vec![A::ty()],
        }.into();

    for (A, B) where (A: Document, B: Document)
        => TypeKind::Tuple {
            fields: vec![A::ty(), B::ty()],
        }.into();

    for (A, B, C) where (A: Document, B: Document, C: Document)
        => TypeKind::Tuple {
            fields: vec![A::ty(), B::ty(), C::ty()],
        }.into();

    for (A, B, C, D) where (A: Document, B: Document, C: Document, D: Document)
        => TypeKind::Tuple {
            fields: vec![A::ty(), B::ty(), C::ty(), D::ty()],
        }.into();

    for (A, B, C, D, E) where (A: Document, B: Document, C: Document, D: Document, E: Document)
        => TypeKind::Tuple {
            fields: vec![A::ty(), B::ty(), C::ty(), D::ty(), E::ty()],
        }.into();

    for (A, B, C, D, E, F) where (A: Document, B: Document, C: Document, D: Document, E: Document, F: Document)
        => TypeKind::Tuple {
            fields: vec![A::ty(), B::ty(), C::ty(), D::ty(), E::ty(), F::ty()],
        }.into();

    for (A, B, C, D, E, F, G) where (A: Document, B: Document, C: Document, D: Document, E: Document, F: Document, G: Document)
        => TypeKind::Tuple {
            fields: vec![A::ty(), B::ty(), C::ty(), D::ty(), E::ty(), F::ty(), G::ty()],
        }.into();

    for (A, B, C, D, E, F, G, H) where (A: Document, B: Document, C: Document, D: Document, E: Document, F: Document, G: Document, H: Document)
        => TypeKind::Tuple {
            fields: vec![A::ty(), B::ty(), C::ty(), D::ty(), E::ty(), F::ty(), G::ty(), H::ty()],
        }.into();

    for (A, B, C, D, E, F, G, H, I) where (A: Document, B: Document, C: Document, D: Document, E: Document, F: Document, G: Document, H: Document, I: Document)
        => TypeKind::Tuple {
            fields: vec![A::ty(), B::ty(), C::ty(), D::ty(), E::ty(), F::ty(), G::ty(), H::ty(), I::ty()],
        }.into();

    for (A, B, C, D, E, F, G, H, I, J) where (A: Document, B: Document, C: Document, D: Document, E: Document, F: Document, G: Document, H: Document, I: Document, J: Document)
        => TypeKind::Tuple {
            fields: vec![A::ty(), B::ty(), C::ty(), D::ty(), E::ty(), F::ty(), G::ty(), H::ty(), I::ty(), J::ty()],
        }.into();
}
