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
        let req_res = reqwest::get(format!("https://www.google.com/search?q={}", query))
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

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

            let link = x
                .select(&Selector::parse("a").unwrap())
                .next()
                .unwrap();

			let description = des_sel.text().skip(1).collect::<Vec<_>>().join(" ");
            let url = link.value().attr("href").unwrap();
			let title = link.text().collect::<Vec<_>>()[0].to_string();
            SearchResult {
                title: title,
                url: get_target_url(url),
                description: description,
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
    //let google = Google;
    let bing = Bing;

    let search_engines: Vec<Box<dyn Search>> = vec![Box::new(bing)];
    let queries = vec!["tsinghua best courses"];

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

                for result in results.iter().enumerate() {
                    let res = reqwest::get(&result.1.url).await.unwrap();

                    let filetype = match res.headers().get("content-type") {
                        None => "html",
                        Some(x) => match x.to_str() {
                            Ok("application/pdf") => "pdf",
                            _ => "html",
                        },
                    };

                    let req_res = res.bytes().await.unwrap();
                    let mut file = std::fs::File::create(format!(
                        "{}/TP_{}_{}_{}_{}.{}",
                        result_folder,
                        engine.name(),
                        query.0 + 1,
                        result.0 + 1,
                        student_id,
                        filetype
                    ))
                    .unwrap();
                    file.write_all(&req_res).unwrap();
                    println!("[{}] Retrieved {}", engine.name(), result.1.url);
                }
            }
            for result in results {
                println!("{:?}", result);
            }
        }
    }
}
