use async_trait::async_trait;
use reqwest::Error;
use scraper::{Html, Selector};

#[derive(Debug)]
struct SearchResult {
    title: String,
    url: String,
    description: String,
}

#[async_trait]
trait Search {
    async fn search(&self, query: &str) -> Result<Vec<SearchResult>, Error>;
}
struct Google;

#[async_trait]
impl Search for Google {
    async fn search(&self, query: &str) -> Result<Vec<SearchResult>, Error> {
        /*let req_res = reqwest::get(format!("https://www.google.com/search?q={}", query))
        .await?
        .text()
        .await?; */

        let req_res = std::fs::read_to_string("cachegoogle.html").unwrap();

        let doc = Html::parse_document(&req_res);
        let sel = Selector::parse("div.Gx5Zad.fP1Qef.xpd.EtOod.pkphOe").unwrap();

        let results = doc.select(&sel).take(4);

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
}

impl Google {
    fn get_target_url(&self, url: &str) -> String {
        if (url.starts_with("/url?q")) {
            url.chars().skip(7).take_while(|x| *x != '&').collect()
        } else {
            url.to_string()
        }
    }
}

struct Bing;

#[async_trait]
impl Search for Bing {
    async fn search(&self, query: &str) -> Result<Vec<SearchResult>, Error> {
        Ok(vec![{
            SearchResult {
                title: "Bing".to_string(),
                url: "https://www.bing.com".to_string(),
                description: "Bing is a search engine".to_string(),
            }
        }])
    }
}

#[tokio::main]
async fn main() {
    let google = Google;
    let bing = Bing;

    let search_engines: Vec<Box<dyn Search>> = vec![Box::new(google), Box::new(bing)];

    for engine in search_engines {
        let results = engine.search("tsinghua best courses").await.unwrap();
        for result in results {
            println!("{:?}", result);
        }
    }
}
