use crate::prelude::*;

#[derive(Document)]
enum GenericEnum<'a, T1, T2, const N: usize> {
    A,
    B(T1),
    C(T2),
    D(&'a str),
    E([u32; N]),
}

printer_test! {
    "output.numbers.json" => to_json(GenericEnum<'static, u32, u64, 3>),
    "output.strings.json" => to_json(GenericEnum<'static, String, String, 4>),
}
