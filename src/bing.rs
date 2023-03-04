use crate::types::{Error, SearchEngine, SearchResult};
use crate::utils::{APP_ACCEPT_LANGUAGE, APP_USER_AGENT};
use reqwest::header::{ACCEPT_LANGUAGE, USER_AGENT};
use scraper::{Html, Selector};

use async_trait::async_trait;

pub struct Bing;

#[async_trait]
impl SearchEngine for Bing {
    async fn search(&self, query: &str, save_html_page: bool) -> Result<Vec<SearchResult>, Error> {
        let http_client = reqwest::Client::new();
        let req_res = http_client
            .get(format!("https://www.bing.com/search?q={}&count=20", query))
            .header(USER_AGENT, APP_USER_AGENT)
            .header(ACCEPT_LANGUAGE, APP_ACCEPT_LANGUAGE)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        if save_html_page {
            std::fs::write("bing.html", &req_res).unwrap();
        }
        let doc = Html::parse_document(&req_res);
        let sel = Selector::parse("li.b_algo").unwrap();

        let results = doc.select(&sel).take(10);

        let results_text = results.map(|x| {
            let des_sel = x.select(&Selector::parse("p").unwrap()).next().unwrap();

            let link = x.select(&Selector::parse("a").unwrap()).next().unwrap();

            let description = des_sel.text().skip(1).collect::<Vec<_>>().join("");
            let url = link.value().attr("href").unwrap();
            let title = link.text().collect::<Vec<_>>().join(" ");
            SearchResult {
                title,
                url: url.to_string(),
                description: Some(description),
            }
        });
        Ok(results_text.collect())
    }

    fn name(&self) -> String {
        "Bing".to_string()
    }
}
