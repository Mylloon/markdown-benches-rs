# Rust markdown parsers benchmarks

Compares [pulldown_cmark](https://github.com/google/pulldown-cmark),
[comrak](https://github.com/kivikakk/comrak) and
[markdown-rs](https://github.com/wooorm/markdown-rs).

```bash
test bench_comrak_article              ... bench:     137,516 ns/iter (+/- 3,772)
test bench_comrak_awesome_rust         ... bench:   1,397,234 ns/iter (+/- 20,278)
test bench_markdown_rs_article         ... bench:   1,616,042 ns/iter (+/- 62,861)
test bench_markdown_rs_awesome_rust    ... bench:  16,967,376 ns/iter (+/- 1,502,322)
test bench_pulldown_cmark_article      ... bench:      40,973 ns/iter (+/- 1,108)
test bench_pulldown_cmark_awesome_rust ... bench:     611,563 ns/iter (+/- 17,536)
```

It has 3 samples right now a medium length article and the readme of
[awesome-rust](https://github.com/rust-unofficial/awesome-rust) which is pretty long.
