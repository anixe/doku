// run: test_json_ty

#[derive(Serialize, Doku)]
#[serde(deny_unknown_fields)]
pub struct Ty;
