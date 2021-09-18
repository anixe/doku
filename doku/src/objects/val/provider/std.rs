use crate::*;
use std::net::IpAddr;

impl val::Provider for IpAddr {
    fn val(&self) -> Option<val::Value> {
        Some(val::Value::String(self.to_string()))
    }
}

impl val::Provider for String {
    fn val(&self) -> Option<val::Value> {
        Some(val::Value::String(self.to_owned()))
    }
}

impl<T> val::Provider for Option<T>
where
    T: val::Provider,
{
    fn val(&self) -> Option<val::Value> {
        self.as_ref().and_then(val::Provider::val)
    }
}

impl<T> val::Provider for Vec<T>
where
    T: val::Provider,
{
    fn val(&self) -> Option<val::Value> {
        Some(val::Value::Array {
            items: self.iter().flat_map(|item| item.val()).collect(),
        })
    }
}
