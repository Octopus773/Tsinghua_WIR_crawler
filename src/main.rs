use serde_json::Result;
use std::{env, process::exit};
use std::io::Write;

pub mod types;
pub mod google;
pub mod bing;

use types::{Search, SearchResult, Error, QueryDescription};
use google::Google;
use bing::Bing;

async fn save_site_as_file(url: &str, filename: &str, auto_filetype: bool) {
    let res = reqwest::get(url).await.unwrap();
    let filetype = match res.headers().get("content-type") {
        None => "html",
        Some(x) => match x.to_str() {
            Ok("application/pdf") => "pdf",
            _ => "html",
        },
    };
    let req_res = res.bytes().await.unwrap();

    let mut file;
    if auto_filetype {
        file = std::fs::File::create(format!("{}.{}", filename, filetype)).unwrap();
    } else {
        file = std::fs::File::create(filename).unwrap();
    }
    file.write_all(&req_res).unwrap();
}


#[tokio::main]
async fn main() {
    let student_id = env::var("STUDENT_ID").unwrap_or_else(|_x| "anonymous".to_string());
    let qd_file_path = env::args().nth(1).expect("Please provide a query description file");
    let save_results = env::args().nth(2).unwrap_or("false".to_string()) == "save";

    let search_engines: Vec<Box<dyn Search>> = vec![Box::new(Google), Box::new(Bing)];

    let qd_file = std::fs::read_to_string(&qd_file_path).map_err(|e| {
        println!("Failed reading {}: {}", &qd_file_path, e);
        exit(1);
    }).unwrap();

    // print niely the error returned by serde
    let queries: Vec<QueryDescription> = serde_json::from_str(&qd_file).map_err(|e| {
        println!("Failed parsing {}: {}", &qd_file_path, e);
        exit(1);
    }).unwrap();


    for engine in search_engines {
        for query in queries.iter().enumerate() {
            let results = engine.search(&query.1.query).await.unwrap();
            if save_results {
                let json = serde_json::to_string(&results).unwrap();

                let mut file = std::fs::File::create(format!(
                    "SE_{}_{}_{}.json",
                    engine.name(),
                    query.0 + 1,
                    student_id
                ))
                .unwrap();
                file.write_all(json.as_bytes()).unwrap();

                let result_folder = "results_websites_data";
                std::fs::create_dir_all(result_folder).unwrap();

                for (idx, result) in results.iter().enumerate() {
                    save_site_as_file(
                        &result.url,
                        &format!(
                            "{}/SE_{}_{}_{}_{}",
                            result_folder,
                            engine.name(),
                            query.0 + 1,
                            idx + 1,
                            student_id
                        ),
                        true,
                    )
                    .await;
                    println!("[{}] Retrieved {}", engine.name(), result.url);
                }
            }

            println!("Results from {}, {}:", engine.name(), results.len());
            println!("=========================");
            println!("");
            for result in results {
                println!("{:?}", result);
            }
        }
    }
}
