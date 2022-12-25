use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Block {
    id: Option<String>,
    title: Option<String>,
    text: Option<String>,
    blocks: Option<Vec<Block>>,
}
