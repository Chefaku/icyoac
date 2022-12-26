use handlebars::Handlebars;

use std::fmt::Write;

use crate::input::block::Block;

pub fn create_block(blocks: Box<Vec<Block>>) -> String {
    let mut reg = Handlebars::new();

    reg.register_template_string(
        "block",
        r#"
    <div id="{{id}}">
    {{#if title}}
        <h2>{{title}}</h2>
    {{/if}}
    {{#if text}}
        <p>{{text}}</p>
    {{/if}}
    {{#if img}}
        {{#with img}}
            <img src="{{src}}" alt="{{alt}}">
        {{/with}}
    {{/if}}
    {{#if blocks}}
        <div>
            {{#each blocks}}
                {{> block }}
            {{/each}}
        </div>
    {{/if}}
    </div>"#)
    .unwrap();

    let mut result = String::from(r#"
<!doctype html>
<html lang="en">
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <title>ICYOA</title>
    <script src="script.js" defer></script>
</head>
<body>"#);

    for block in blocks.iter() {
        let html = reg.render("block", block).unwrap();
        write!(result, "{}", html).unwrap();
    }
    format!("{}</body></html>", result)
}
