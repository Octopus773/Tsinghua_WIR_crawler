
use crate::types::{Search, SearchResult, Error};
use scraper::{ElementRef, Html, Selector};

use async_trait::async_trait;

pub struct Google;

#[async_trait]
impl Search for Google {
    async fn search(&self, query: &str) -> Result<Vec<SearchResult>, Error> {
        /*
        let req_res = reqwest::get(format!("https://www.google.com/search?q={}", query))
            .await
            .unwrap()
            .text()
            .await
            .unwrap();*/

        let req_res = std::fs::read_to_string("cachegoogle.html").unwrap();

        let doc = Html::parse_document(&req_res);
        let sel = Selector::parse("div[lang] a h3").unwrap();

        let results = doc.select(&sel).take(10);

        let results_text = results.map(|x| {
            let mut elem = x;
            while elem.value().name() != "div" || elem.value().attr("lang").is_none() {
                elem = ElementRef::wrap(elem.parent().unwrap()).unwrap();
            }

            let texts = elem.text().collect::<Vec<_>>();
			println!("{:?}", texts);
            let url = elem
                .select(&Selector::parse("a").unwrap())
                .next()
                .unwrap()
                .value()
                .attr("href")
                .unwrap();
            SearchResult {
                title: texts[0].to_string(),
                url: Google::get_target_url(url),
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
    fn get_description(texts: Vec<&str>) -> String {
        let mut description = String::new();
        for text in texts {
            description.push_str(text);
            description.push_str(" ");
        }
        description
    }

    fn get_target_url(url: &str) -> String {
        if url.starts_with("/url?q=") {
            url.chars().skip(7).take_while(|x| *x != '&').collect()
        } else {
            url.to_string()
        }
    }
}