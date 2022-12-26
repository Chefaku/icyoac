use minify_html_onepass::{Cfg, in_place_str};

use std::io::Write;
use std::path::Path;
use std::{fs, fs::File};

use super::block::create_block;
use crate::input::{block::Block, vars::Vars, style::Style};

pub fn create(vars: Box<Vars>, blocks: Box<Vec<Block>>, styles: Box<Vec<Style>>) {
    dir_output();
    create_blocks(blocks);
}

fn dir_output() {
    let path = Path::new("./output");
    if path.exists() {
        return;
    }
    fs::create_dir(path).unwrap();
}

fn create_blocks(blocks: Box<Vec<Block>>) {
    let html = create_block(blocks);
    let html = minify(html);

    File::create("./output/index.html")
        .unwrap()
        .write_all(html.as_bytes())
        .unwrap();
}

fn minify(mut html: String) -> String {
    let cfg = &Cfg {
        minify_js: false,
        minify_css: false,
    };

    let html = html.as_mut_str();
    in_place_str(html, cfg).unwrap().to_string()
}
