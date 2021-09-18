// run: to_json()

#[derive(Serialize, Document)]
#[serde(default)]
pub struct Ty {
    f1: String,
}
