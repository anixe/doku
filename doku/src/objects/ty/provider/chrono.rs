use super::*;
use ::chrono::{DateTime, TimeZone};

impl<Tz: TimeZone> Provider for DateTime<Tz> {
    fn ty() -> ty::Type {
        ty::Type {
            example: Some("2018-04-05T11:44:42.621513958Z"),
            ..String::ty()
        }
    }
}
