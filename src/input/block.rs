use serde::{Deserialize, Serialize};

/*
 * id: id (for logic)
 * title: <h2>
 * text: <p>
 * img: (src, alt)
 * class: class (for style)
 * blocks: <div>
 */
#[derive(Debug, Deserialize, Serialize)]
pub struct Block {
    id: Option<String>,
    title: Option<String>,
    text: Option<String>,
    img: Option<Img>,
    class: Option<String>,
    //func: Option<>,
    blocks: Option<Vec<Block>>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Img {
    src: String,
    alt: String,
}
