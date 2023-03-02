use async_trait::async_trait;
use scraper::{ElementRef, Html, Selector};
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
        let req_res = reqwest::get(format!("https://www.google.com/search?q={}", query))
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        let doc = Html::parse_document(&req_res);
        let sel = Selector::parse("div > div > a > div > div > h3").unwrap();

        let results = doc.select(&sel).take(10);

        let results_text = results.map(|x| {
            let x = x.ancestors().nth(4).unwrap();
            let x = ElementRef::wrap(x).unwrap();

            let texts = x.text().collect::<Vec<_>>();
			println!("{:?}", texts);
            let url = x
                .select(&Selector::parse("a").unwrap())
                .next()
                .unwrap()
                .value()
                .attr("href")
                .unwrap();
            SearchResult {
                title: texts[0].to_string(),
                url: get_target_url(url),
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

fn get_target_url(url: &str) -> String {
    if url.starts_with("/url?q=") {
        url.chars().skip(7).take_while(|x| *x != '&').collect()
    } else {
        url.to_string()
    }
}

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

struct Bing;

#[async_trait]
impl Search for Bing {
    async fn search(&self, query: &str) -> Result<Vec<SearchResult>> {
        let req_res = reqwest::get(format!("https://www.bing.com/search?q={}", query))
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        let doc = Html::parse_document(&req_res);
        let sel = Selector::parse("li.b_algo").unwrap();

        let results = doc.select(&sel).take(10);

        let results_text = results.map(|x| {
            let des_sel = x.select(&Selector::parse("p").unwrap()).next().unwrap();

            let link = x.select(&Selector::parse("a").unwrap()).next().unwrap();

            let description = des_sel
                .text()
                .skip(1)
                .collect::<Vec<_>>()
                .iter()
                .map(|x| x.trim())
				.collect::<Vec<_>>()
                .join(" ");
            let url = link.value().attr("href").unwrap();
            let title = link.text().collect::<Vec<_>>().join(" ");
            SearchResult {
                title,
                url: get_target_url(url),
                description,
            }
        });
        Ok(results_text.collect())
    }

    fn name(&self) -> String {
        "Bing".to_string()
    }
}

#[tokio::main]
async fn main() {
    let save_results = env::args().nth(1).unwrap_or("false".to_string()) == "save";
    let student_id = env::var("STUDENT_ID").unwrap_or_else(|_x| "anonymous".to_string());

    let search_engines: Vec<Box<dyn Search>> = vec![Box::new(Google), Box::new(Bing)];
    let queries = vec!["stack overflow parse html with regex"];

    for engine in search_engines {
        for query in queries.iter().enumerate() {
            let results = engine.search(query.1).await.unwrap();
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
