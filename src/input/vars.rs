use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Vars {
    points: Vec<(String, i32)>,
    rows: Vec<(String, i32)>,
}
