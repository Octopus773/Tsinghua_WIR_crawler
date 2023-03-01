use async_trait::async_trait;
use scraper::{Html, Selector};
use reqwest::Error;

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

        let req_res = std::fs::read_to_string("cachegoogle").unwrap();



        // parse the google search page and return results and skip ads
        let results: Vec<String> = Html::parse_document(&req_res)
            .select(&Selector::parse("div.MjjYud").unwrap())
            .map(|x| x.text().collect::<Vec<_>>().join(""))
            //.map(|x| x.select(&Selector::parse("a").unwrap()).next().unwrap())
            //.map(|x| x.value().attr("href").unwrap().to_string())
            .collect();
        
        Ok(results.iter().map(|x| {
            SearchResult {
                title: x.to_string(),
                url: "".to_string(),
                description: "".to_string(),
            }
        }).collect())
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
        println!("{:?}", results);
    }
}
