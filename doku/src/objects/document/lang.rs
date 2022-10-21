use crate::*;

document! {
    for ()
        => TypeKind::Tuple {
            fields: Default::default()
        };

    for bool
        => TypeKind::Bool;

    for char
        => TypeKind::String;

    for str
        => TypeKind::String;

    for f32
        => TypeKind::Float;

    for f64
        => TypeKind::Float;

    for u8
        => TypeKind::Integer;

    for i8
        => TypeKind::Integer;

    for u16
        => TypeKind::Integer;

    for i16
        => TypeKind::Integer;

    for u32
        => TypeKind::Integer;

    for i32
        => TypeKind::Integer;

    for u64
        => TypeKind::Integer;

    for i64
        => TypeKind::Integer;

    for u128
        => TypeKind::Integer;

    for i128
        => TypeKind::Integer;

    for usize
        => TypeKind::Integer;

    for isize
        => TypeKind::Integer;

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
        };

    for &[T] where (T: Document)
        => TypeKind::Array {
            ty: Box::new(T::ty()),
            size: None,
        };

    for &mut [T] where (T: Document)
        => <&[T]>::ty();

    /* ----- */

    for (A,) where (A: Document)
        => TypeKind::Tuple {
            fields: vec![A::ty()],
        };

    for (A, B) where (A: Document, B: Document)
        => TypeKind::Tuple {
            fields: vec![A::ty(), B::ty()],
        };

    for (A, B, C) where (A: Document, B: Document, C: Document)
        => TypeKind::Tuple {
            fields: vec![A::ty(), B::ty(), C::ty()],
        };

    for (A, B, C, D) where (A: Document, B: Document, C: Document, D: Document)
        => TypeKind::Tuple {
            fields: vec![A::ty(), B::ty(), C::ty(), D::ty()],
        };

    for (A, B, C, D, E) where (A: Document, B: Document, C: Document, D: Document, E: Document)
        => TypeKind::Tuple {
            fields: vec![A::ty(), B::ty(), C::ty(), D::ty(), E::ty()],
        };

    for (A, B, C, D, E, F) where (A: Document, B: Document, C: Document, D: Document, E: Document, F: Document)
        => TypeKind::Tuple {
            fields: vec![A::ty(), B::ty(), C::ty(), D::ty(), E::ty(), F::ty()],
        };

    for (A, B, C, D, E, F, G) where (A: Document, B: Document, C: Document, D: Document, E: Document, F: Document, G: Document)
        => TypeKind::Tuple {
            fields: vec![A::ty(), B::ty(), C::ty(), D::ty(), E::ty(), F::ty(), G::ty()],
        };

    for (A, B, C, D, E, F, G, H) where (A: Document, B: Document, C: Document, D: Document, E: Document, F: Document, G: Document, H: Document)
        => TypeKind::Tuple {
            fields: vec![A::ty(), B::ty(), C::ty(), D::ty(), E::ty(), F::ty(), G::ty(), H::ty()],
        };

    for (A, B, C, D, E, F, G, H, I) where (A: Document, B: Document, C: Document, D: Document, E: Document, F: Document, G: Document, H: Document, I: Document)
        => TypeKind::Tuple {
            fields: vec![A::ty(), B::ty(), C::ty(), D::ty(), E::ty(), F::ty(), G::ty(), H::ty(), I::ty()],
        };

    for (A, B, C, D, E, F, G, H, I, J) where (A: Document, B: Document, C: Document, D: Document, E: Document, F: Document, G: Document, H: Document, I: Document, J: Document)
        => TypeKind::Tuple {
            fields: vec![A::ty(), B::ty(), C::ty(), D::ty(), E::ty(), F::ty(), G::ty(), H::ty(), I::ty(), J::ty()],
        };
}
