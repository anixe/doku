use super::*;
use ::std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};
use ::std::net::IpAddr;
use ::std::rc::Rc;
use ::std::sync::{Arc, Mutex, RwLock};

type_providers! {
    for IpAddr =>
        Type {
            example: Some("127.0.0.1"),
            ..String::ty()
        }

    for String => TypeDef::String

    for Option<T> where (T: TypeProvider)  =>
        TypeDef::Optional {
            ty: Box::new(T::ty()),
        }

    for Box<T> where (T: TypeProvider + ?Sized) =>
        T::ty().def

    for Rc<T> where (T: TypeProvider + ?Sized) =>
        T::ty().def

    for Arc<T> where (T: TypeProvider + ?Sized) =>
        T::ty().def

    for RwLock<T> where (T: TypeProvider + ?Sized) =>
        T::ty().def

    for Mutex<T> where (T: TypeProvider + ?Sized) =>
        T::ty().def

    for Vec<T> where (T: TypeProvider)  =>
        TypeDef::Array {
            ty: Box::new(T::ty()),
            size: None,
        }

    for VecDeque<T> where (T: TypeProvider)  =>
        TypeDef::Array {
            ty: Box::new(T::ty()),
            size: None,
        }

    for LinkedList<T> where (T: TypeProvider)  =>
        TypeDef::Array {
            ty: Box::new(T::ty()),
            size: None,
        }

    for HashMap<K, V> where (K: TypeProvider, V: TypeProvider) =>
        TypeDef::Map {
            key: Box::new(K::ty()),
            value: Box::new(V::ty()),
        }

    for BTreeMap<K, V> where (K: TypeProvider, V: TypeProvider) =>
        TypeDef::Map {
            key: Box::new(K::ty()),
            value: Box::new(V::ty()),
        }

    for HashSet<T> where (T: TypeProvider) =>
        TypeDef::Array {
            ty: Box::new(T::ty()),
            size: None,
        }

    for BTreeSet<T> where (T: TypeProvider) =>
        TypeDef::Array {
            ty: Box::new(T::ty()),
            size: None,
        }

    for BinaryHeap<T> where (T: TypeProvider) =>
        TypeDef::Array {
            ty: Box::new(T::ty()),
            size: None,
        }
}
