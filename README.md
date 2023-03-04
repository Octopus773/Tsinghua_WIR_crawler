
# Tsingha Search Engine Crawler
## Project made as Assignment 1 of Web Information Retrieval course 2023

### Description

This is a crawler made in Rust that fetches and parse Google and Bing search results.
Libraries used are:
- Tokio
- Reqwest
- Scraper
- Serde

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
The save parameter is optional and if present, the SE results will be saved as files in the root and search results in the 'results_website_data' folder. 