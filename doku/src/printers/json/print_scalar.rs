use super::*;

impl Ctxt<'_, '_> {
    pub fn print_bool(&mut self) {
        self.out.text(self.example().unwrap_or("true"));
    }

    pub fn print_float(&mut self) {
        self.out.text(self.example().unwrap_or("123.45"));
    }

    pub fn print_integer(&mut self) {
        self.out.text(self.example().unwrap_or("123"));
    }

    pub fn print_string(&mut self) {
        self.out.text(format!("\"{}\"", self.example().unwrap_or("string")));
    }
}
