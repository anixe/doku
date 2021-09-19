// run: test_json_ty

#[derive(Serialize, Doku)]
#[serde(default)]
pub struct Ty {
    f1: String,
}
