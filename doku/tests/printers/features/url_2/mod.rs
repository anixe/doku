use crate::prelude::*;
use url::Url;

#[derive(Document)]
struct Ty {
    url: Url,
}

printer_test! {
    "output.json" => to_json(Ty),
}
