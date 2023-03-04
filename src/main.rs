use serde_json::Result;
use std::{env, process::exit};

pub mod bing;
pub mod google;
pub mod types;
pub mod utils;

use types::{Error, QueryDescription, SearchEngine, SearchResult};

use bing::Bing;
use google::Google;

#[tokio::main]
async fn main() {
    let qd_file_path = env::args()
        .nth(1)
        .expect("Please provide a query description file");
    let save_results = env::args().nth(2).unwrap_or("false".to_string()) == "save";
    let student_id = env::var("STUDENT_ID").unwrap_or_else(|_x| "2022400437".to_string());
    let search_engines: Vec<Box<dyn SearchEngine>> = vec![Box::new(Google), Box::new(Bing)];

    let qd_file = std::fs::read_to_string(&qd_file_path)
        .map_err(|e| {
            println!("Failed reading {}: {}", &qd_file_path, e);
            exit(1);
        })
        .unwrap();

    let queries: Vec<QueryDescription> = serde_json::from_str(&qd_file)
        .map_err(|e| {
            println!("Failed parsing {}: {}", &qd_file_path, e);
            exit(1);
        })
        .unwrap();

    for engine in search_engines {
        for query in queries.iter() {
            let results = engine.search(&query.query, true).await.unwrap();
            println!("Results from {}, {}:", engine.name(), results.len());
            println!("=========================");
            println!("");
            for result in &results {
                println!("{:?}", result);
            }

            if !save_results {
                continue;
            }
            utils::save_results_to_disk(
                "results_website_data",
                &engine.name(),
                query.query_num,
                &student_id,
                &results,
            ).await;
        }
    }
}
