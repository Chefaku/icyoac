use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Style {
    selector: String,
    text: Option<Text>,
    size: Option<Size>,
    border: Option<String>,
    margin: Option<String>,
    padding: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Text {
    size: Option<String>,
    color: Option<String>,
    family: Option<String>,
    align: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Size {
    width: Option<String>,
    height: Option<String>,
}
