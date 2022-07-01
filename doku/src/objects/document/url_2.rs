use crate::*;
use url::Url;

document! {
    for Url
        => Type {
            example: Some(Example::Simple("https://www.rust-lang.org")),
            ..String::ty()
        };
}
