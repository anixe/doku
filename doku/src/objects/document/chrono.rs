use crate::*;
use ::chrono::{DateTime, TimeZone};
use std::fmt::Display;

document! {
    for DateTime<Tz> where (Tz) { Tz: TimeZone, Tz::Offset: Display }
        => Type {
            example: Some(Example::Simple("2018-04-05T11:44:42.621513958Z")),
            ..String::ty()
        };
}
