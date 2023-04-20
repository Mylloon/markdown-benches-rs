#![feature(test)]
extern crate comrak;
extern crate markdown;
extern crate pulldown_cmark;
extern crate test;
#[macro_use]
extern crate lazy_static;

use comrak::{markdown_to_html, ComrakOptions};
use markdown::to_html;
use pulldown_cmark::{html, Parser};

lazy_static! {
    // one of my article, medium length
    static ref ARTICLE: &'static str = include_str!("samples/article.md");
    // readme from https://github.com/rust-unofficial/awesome-rust
    static ref AWESOME_RUST: &'static str = include_str!("samples/awesome_rust.md");
}

#[bench]
fn bench_pulldown_cmark_article(b: &mut test::Bencher) {
    b.iter(|| {
        let mut s = String::new();

        let p = Parser::new(&ARTICLE);
        html::push_html(&mut s, p);
        s
    });
}

#[bench]
fn bench_comrak_article(b: &mut test::Bencher) {
    b.iter(|| markdown_to_html(&ARTICLE, &ComrakOptions::default()));
}

#[bench]
fn bench_markdown_rs_article(b: &mut test::Bencher) {
    b.iter(|| to_html(&ARTICLE));
}

#[bench]
fn bench_pulldown_cmark_awesome_rust(b: &mut test::Bencher) {
    b.iter(|| {
        let mut s = String::new();

        let p = Parser::new(&AWESOME_RUST);
        html::push_html(&mut s, p);
        s
    });
}

#[bench]
fn bench_comrak_awesome_rust(b: &mut test::Bencher) {
    b.iter(|| markdown_to_html(&AWESOME_RUST, &ComrakOptions::default()));
}

#[bench]
fn bench_markdown_rs_awesome_rust(b: &mut test::Bencher) {
    b.iter(|| to_html(&AWESOME_RUST));
}
