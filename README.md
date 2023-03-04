# Tsinghua Search Engine Crawler
## Project made as Assignment 1 of Web Information Retrieval course 2023

### Description

This is a crawler made in Rust that fetches and parse Google and Bing search results.
Libraries used are:
- Tokio (async)
- Reqwest (HTTP requests)
- Scraper (HTML parsing and selection)
- Serde (JSON serialization and deserialization)

### How to run

To run the project, you need to have Rust installed on your machine.
Then, you can run the project with the following command:

```bash
cargo run query_design_file [save]
```
Note: Built binaries will be located in the target/debug folder. (target/release if you use the --release flag)


Query design file is a file that contains the queries to be made to the search engines.
The file must be in the following format:
```json
[
    {"queryNum": 1, "query": "Why is rust memory safe ?", "description": "I want to learn rust borrow checker"},
    {"queryNum": 2, "query": "What is Wayland and the main departures from X11 rendering", "description": "My new game is allowing me to choose between the two"}
]
```
The save parameter is optional and if present, the SE results will be saved as files in the root and search results in the `results_website_data` folder also located at the root.

SE_*.json example, the description fireld is nullable when it fails to parse it or find it
```json
[
  {
    "title": "Rust (programming language) - Wikipedia",
    "url": "https://en.wikipedia.org/wiki/Rust_(programming_language)#:~:text=Rust%20is%20designed%20to%20be,inputs%20to%20be%20already%20initialized.",
    "description": "Rust is designed to be memory safe. It does not permit null pointers, dangling pointers, or data races. Data values can be initialized only through a fixed set of forms, all of which require their inputs to be already initialized.À propos des extraits optimisés•Commentaires"
  },
  {
    "title": "Why Safe Programming Matters and Why a Language Like ...",
    "url": "https://developer.okta.com/blog/2022/03/18/programming-security-and-why-rust",
    "description": "18 mars 2022 — Rust ensures memory safety at compile time using its innovative ownership mechanism and the borrow checker built into the compiler. The compiler ..."
  }
]

```