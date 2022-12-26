use handlebars::Handlebars;

use crate::input::block::Block;

pub fn create_block(block: &Block) -> String {
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
</div>
"#,
    )
    .unwrap();

    reg.render("block", block).unwrap()
}
