# Full-Text Search engine for Rust

https://artem.krylysov.com/blog/2020/07/28/lets-build-a-full-text-search-engine

## Build and Run
```bash
$ wget https://dumps.wikimedia.org/enwiki/latest/enwiki-latest-abstract1.xml.gz
$ gzip -d enwiki-latest-abstract1.xml.gz
$ cargo build --relase
$ ./target/release/fts
```
