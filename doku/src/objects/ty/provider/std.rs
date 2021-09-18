use super::*;
use ::std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};
use ::std::net::IpAddr;
use ::std::rc::Rc;
use ::std::sync::{Arc, Mutex, RwLock};

providers! {
    for IpAddr =>
        ty::Type {
            example: Some("127.0.0.1"),
            ..String::ty()
        }

    for String => ty::Def::String

    for Option<T> where (T: ty::Provider)  =>
        ty::Def::Optional {
            ty: Box::new(T::ty()),
        }

    for Box<T> where (T: ty::Provider + ?Sized) =>
        T::ty().def

    for Rc<T> where (T: ty::Provider + ?Sized) =>
        T::ty().def

    for Arc<T> where (T: ty::Provider + ?Sized) =>
        T::ty().def

    for RwLock<T> where (T: ty::Provider + ?Sized) =>
        T::ty().def

    for Mutex<T> where (T: ty::Provider + ?Sized) =>
        T::ty().def

    for Vec<T> where (T: ty::Provider)  =>
        ty::Def::Array {
            ty: Box::new(T::ty()),
            size: None,
        }

    for VecDeque<T> where (T: ty::Provider)  =>
        ty::Def::Array {
            ty: Box::new(T::ty()),
            size: None,
        }

    for LinkedList<T> where (T: ty::Provider)  =>
        ty::Def::Array {
            ty: Box::new(T::ty()),
            size: None,
        }

    for HashMap<K, V> where (K: ty::Provider, V: ty::Provider) =>
        ty::Def::Map {
            key: Box::new(K::ty()),
            value: Box::new(V::ty()),
        }

    for BTreeMap<K, V> where (K: ty::Provider, V: ty::Provider) =>
        ty::Def::Map {
            key: Box::new(K::ty()),
            value: Box::new(V::ty()),
        }

    for HashSet<T> where (T: ty::Provider) =>
        ty::Def::Array {
            ty: Box::new(T::ty()),
            size: None,
        }

    for BTreeSet<T> where (T: ty::Provider) =>
        ty::Def::Array {
            ty: Box::new(T::ty()),
            size: None,
        }

    for BinaryHeap<T> where (T: ty::Provider) =>
        ty::Def::Array {
            ty: Box::new(T::ty()),
            size: None,
        }
}
