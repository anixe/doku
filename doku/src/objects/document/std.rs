use crate::*;
use ::std::collections::{
    BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque,
};
use ::std::net::IpAddr;
use ::std::rc::Rc;
use ::std::sync::{Arc, Mutex, RwLock};
use ::std::time::Duration;
use std::path::PathBuf;

document! {
    for String
        => TypeKind::String.into();

    for IpAddr
        => Type { example: Some(Example::Simple("127.0.0.1")), ..String::ty() };

    /* ----- */

    for Option<T> where (T: Document)
        => TypeKind::Optional {
            ty: Box::new(T::ty()),
        }.into();

    for Box<T> where (T: Document + ?Sized)
        => T::ty();

    for Rc<T> where (T: Document + ?Sized)
        => T::ty();

    for Arc<T> where (T: Document + ?Sized)
        => T::ty();

    for RwLock<T> where (T: Document + ?Sized)
        => T::ty();

    for Mutex<T> where (T: Document + ?Sized)
        => T::ty();

    /* ----- */

    for Vec<T> where (T: Document)
        => <&[T]>::ty();

    for VecDeque<T> where (T: Document)
        => <&[T]>::ty();

    for LinkedList<T> where (T: Document)
        => <&[T]>::ty();

    for HashSet<T> where (T: Document)
        => <&[T]>::ty();

    for BTreeSet<T> where (T: Document)
        => <&[T]>::ty();

    for BinaryHeap<T> where (T: Document)
        => <&[T]>::ty();

    for HashMap<K, V> where (K: Document, V: Document)
        => TypeKind::Map {
            key: Box::new(K::ty()),
            value: Box::new(V::ty()),
        }.into();

    for BTreeMap<K, V> where (K: Document, V: Document)
        => <HashMap<K, V>>::ty();

    for PathBuf
        => TypeKind::String.into();

    /* ----- */

    for Duration
        => duration();
}

fn duration() -> Type {
    Type::from(TypeKind::Struct {
        fields: Fields::Named {
            fields: vec![
                (
                    "secs",
                    Field {
                        ty: Type {
                            example: Some(Example::Simple("123")),
                            ..u64::ty()
                        },
                        flattened: false,
                    },
                ),
                (
                    "nanos",
                    Field {
                        ty: Type {
                            example: Some(Example::Simple("456000000")),
                            ..u32::ty()
                        },
                        flattened: false,
                    },
                ),
            ],
        },
        transparent: false,
    })
}
