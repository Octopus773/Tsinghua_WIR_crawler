use async_trait::async_trait;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::env;
use std::io::Write;

#[derive(Debug, Serialize, Deserialize)]
struct SearchResult {
    title: String,
    url: String,
    description: String,
}

#[async_trait]
trait Search {
    async fn search(&self, query: &str) -> Result<Vec<SearchResult>>;

    fn name(&self) -> String;
}
struct Google;

#[async_trait]
impl Search for Google {
    async fn search(&self, query: &str) -> Result<Vec<SearchResult>> {
        /*let req_res = reqwest::get(format!("https://www.google.com/search?q={}", query))
        .await?
        .text()
        .await?; */

        let req_res = std::fs::read_to_string("cachegoogle.html").unwrap();

        let doc = Html::parse_document(&req_res);
        let sel = Selector::parse("div.Gx5Zad.fP1Qef.xpd.EtOod.pkphOe").unwrap();

        let results = doc.select(&sel).take(10);

        let results_text = results.map(|x| {
            let texts = x.text().collect::<Vec<_>>();
            let url = x
                .select(&Selector::parse("a").unwrap())
                .next()
                .unwrap()
                .value()
                .attr("href")
                .unwrap()
                .to_string();
            let url = self.get_target_url(&url);
            SearchResult {
                title: texts[0].to_string(),
                url: url,
                description: texts[2].to_string(),
            }
        });

        let ret = results_text.collect();
        Ok(ret)
    }

    fn name(&self) -> String {
        "Google".to_string()
    }
}

impl Google {
    fn get_target_url(&self, url: &str) -> String {
        if url.starts_with("/url?q=") {
            url.chars().skip(7).take_while(|x| *x != '&').collect()
        } else {
            url.to_string()
        }
    }
}

struct Bing;

#[async_trait]
impl Search for Bing {
    async fn search(&self, query: &str) -> Result<Vec<SearchResult>> {
        Ok(vec![{
            SearchResult {
                title: "Bing".to_string(),
                url: "https://www.bing.com".to_string(),
                description: "Bing is a search engine".to_string(),
            }
        }])
    }

    fn name(&self) -> String {
        "Bing".to_string()
    }
}

#[tokio::main]
async fn main() {
    let save_results = env::args().nth(1).unwrap_or("false".to_string()) == "save";
    let student_id = env::var("STUDENT_ID").unwrap_or_else(|_x| "anonymous".to_string());
    let google = Google;
    let bing = Bing;

    let search_engines: Vec<Box<dyn Search>> = vec![Box::new(google), Box::new(bing)];
    let queries = vec!["tsinghua best courses"];

    for engine in search_engines {
        for query in queries.iter().enumerate() {
            let results = engine.search(query.1).await.unwrap();
            if save_results {
                let mut file = std::fs::File::create(format!(
                    "SE_{}_{}_{}.json",
                    engine.name(),
                    query.0 + 1,
                    student_id
                ))
                .unwrap();

                let json = serde_json::to_string(&results).unwrap();
                file.write_all(json.as_bytes()).unwrap();

                // create folder to store the results website data (html or pdf)
                let result_folder = "results_websites_data";

                std::fs::create_dir_all(result_folder).unwrap();

                // download the results websites data
                for result in results.iter().enumerate() {
                    let res = reqwest::get(&result.1.url).await.unwrap();

                    let filetype = match res.headers().get("content-type") {
                        None => "html",
                        Some(x) => match x.to_str() {
                            Ok("application/pdf") => "pdf",
                            _ => "html",
                        },
                    };

                    let req_res = res.text().await.unwrap();
                    let filename = format!(
                        "{}/TP_{}_{}_{}_{}.{}",
                        result_folder,
                        engine.name(),
                        query.0 + 1,
                        result.0 + 1,
                        student_id,
                        filetype
                    );
                    println!("filename: {}", filename);
                    let mut file = std::fs::File::create(filename).unwrap();
                    file.write_all(req_res.as_bytes()).unwrap();
                    println!("[{}] Retrieved {}", engine.name(), result.1.url);
                }
            }
            for result in results {
                println!("{:?}", result);
            }
        }
    }
}
