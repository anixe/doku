#[derive(Doku)]
pub struct F1(String);

#[derive(Doku)]
pub struct F2(String, usize);

#[derive(Doku)]
pub struct F3(String, usize, f32);

#[derive(Doku)]
pub struct Ty {
    f1: F1,
    f2: F2,
    f3: F3,
}
