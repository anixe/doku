use crate::*;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::rc::Rc;
use std::sync::{Arc, Mutex};

pub trait TypeProvider {
    fn ty() -> Type;
}

macro_rules! type_providers {
    (
        $(
            $(where ( $($tt:tt)+ ))?
            for $rust_ty:ty
            => $doku_ty:expr
        )+
    ) => {
        $(
            impl $(< $($tt)+ >)? TypeProvider for $rust_ty {
                fn ty() -> Type {
                    Type::from_def($doku_ty)
                }
            }
        )+
    };
}

#[cfg(feature = "chrono-04")]
mod chrono;

type_providers! {
    for bool => TypeDef::Bool

    for char   => TypeDef::String
    for str    => TypeDef::String
    for String => TypeDef::String

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

    where (A: TypeProvider) for (A,) =>
        TypeDef::Tuple {
            fields: vec![A::ty()],
        }

    where (A: TypeProvider, B: TypeProvider) for (A, B) =>
        TypeDef::Tuple {
            fields: vec![A::ty(), B::ty()],
        }

    where (A: TypeProvider, B: TypeProvider, C: TypeProvider) for (A, B, C) =>
        TypeDef::Tuple {
            fields: vec![A::ty(), B::ty(), C::ty()],
        }

    where (A: TypeProvider, B: TypeProvider, C: TypeProvider, D: TypeProvider) for (A, B, C, D) =>
        TypeDef::Tuple {
            fields: vec![A::ty(), B::ty(), C::ty(), D::ty()],
        }

    where (A: TypeProvider, B: TypeProvider, C: TypeProvider, D: TypeProvider, E: TypeProvider) for (A, B, C, D, E) =>
        TypeDef::Tuple {
            fields: vec![A::ty(), B::ty(), C::ty(), D::ty(), E::ty()],
        }

    where (A: TypeProvider, B: TypeProvider, C: TypeProvider, D: TypeProvider, E: TypeProvider, F: TypeProvider) for (A, B, C, D, E, F) =>
        TypeDef::Tuple {
            fields: vec![A::ty(), B::ty(), C::ty(), D::ty(), E::ty(), F::ty()],
        }

    where (T: TypeProvider, const N: usize) for [T; N] =>
        TypeDef::Array {
            ty: Box::new(T::ty()),
            size: Some(N),
        }

    where (T: TypeProvider) for Option<T> =>
        TypeDef::Optional {
            ty: Box::new(T::ty()),
        }
    where (T: TypeProvider) for Vec<T> =>
        TypeDef::Array {
            ty: Box::new(T::ty()),
            size: None,
        }

    where (K: TypeProvider, V: TypeProvider) for BTreeMap<K, V> =>
        TypeDef::Map {
            key: Box::new(K::ty()),
            value: Box::new(V::ty()),
        }

    where (K: TypeProvider, V: TypeProvider) for HashMap<K, V> =>
        TypeDef::Map {
            key: Box::new(K::ty()),
            value: Box::new(V::ty()),
        }

    where (T: TypeProvider) for BTreeSet<T> =>
        TypeDef::Array {
            ty: Box::new(T::ty()),
            size: None,
        }

    where (T: TypeProvider) for HashSet<T> =>
        TypeDef::Array {
            ty: Box::new(T::ty()),
            size: None,
        }

    where (T: TypeProvider + ?Sized) for &T =>
        T::ty().def

    where (T: TypeProvider + ?Sized) for &mut T =>
        T::ty().def

    where (T: TypeProvider + ?Sized) for Box<T> =>
        T::ty().def

    where (T: TypeProvider + ?Sized) for Rc<T> =>
        T::ty().def

    where (T: TypeProvider + ?Sized) for Arc<T> =>
        T::ty().def

    where (T: TypeProvider + ?Sized) for Mutex<T> =>
        T::ty().def
}
