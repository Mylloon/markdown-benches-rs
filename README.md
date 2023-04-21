# Rust markdown parsers benchmarks

Compares [pulldown_cmark](https://github.com/google/pulldown-cmark),
[comrak](https://github.com/kivikakk/comrak) and
[markdown-rs](https://github.com/wooorm/markdown-rs).

```bash
test bench_comrak_article              ... bench:     150,062 ns/iter (+/- 37,905)
test bench_comrak_awesome_rust         ... bench:   3,267,189 ns/iter (+/- 1,081,891)
test bench_markdown_rs_article         ... bench:   1,653,713 ns/iter (+/- 90,975)
test bench_markdown_rs_awesome_rust    ... bench:  40,706,485 ns/iter (+/- 4,468,671)
test bench_pulldown_cmark_article      ... bench:      43,833 ns/iter (+/- 5,542)
test bench_pulldown_cmark_awesome_rust ... bench:   1,399,065 ns/iter (+/- 180,537)
```

It only has 2 samples right now a medium length article and the readme of
[awesome-rust](https://github.com/rust-unofficial/awesome-rust) which is pretty long.
