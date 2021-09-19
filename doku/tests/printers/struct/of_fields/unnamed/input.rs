// run: test_json_ty, test_json_val

#[derive(Doku)]
pub struct F1(String);

impl Default for F1 {
    fn default() -> Self {
        Self("I'm F1".to_string())
    }
}

#[derive(Doku)]
pub struct F2(String, usize);

impl Default for F2 {
    fn default() -> Self {
        Self("I'm F2".to_string(), 100)
    }
}

#[derive(Doku)]
pub struct F3(String, usize, f32);

impl Default for F3 {
    fn default() -> Self {
        Self("I'm F3".to_string(), 200, 300.5)
    }
}

#[derive(Doku, Default)]
pub struct Ty {
    f1: F1,
    f2: F2,
    f3: F3,
}
