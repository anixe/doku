use crate::*;
use chrono::{DateTime, TimeZone};

document! {
    for DateTime<Tz> where (Tz) { Tz: TimeZone }
        => Type {
            example: Some(Example::Simple("2018-04-05T11:44:42.621513958Z")),
            ..String::ty()
        };
}
