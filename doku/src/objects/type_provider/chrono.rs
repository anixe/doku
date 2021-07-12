use super::*;
use ::chrono::{DateTime, TimeZone};

impl<Tz: TimeZone> TypeProvider for DateTime<Tz> {
    fn ty() -> Type {
        Type {
            example: Some("2018-04-05T11:44:42.621513958Z"),
            ..String::ty()
        }
    }
}
