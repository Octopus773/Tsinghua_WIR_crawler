use crate::types::{Error, Search, SearchResult};
use scraper::{ElementRef, Html, Selector};

use async_trait::async_trait;

pub struct Google;

#[async_trait]
impl Search for Google {
    async fn search(&self, query: &str) -> Result<Vec<SearchResult>, Error> {
        let req_res = reqwest::get(format!("https://www.google.com/search?q={}", query))
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        //let req_res = std::fs::read_to_string("cachegoogle.html").unwrap();

        let doc = Html::parse_document(&req_res);
        let sel = Selector::parse("div[lang] a h3").unwrap();

        let results = doc.select(&sel).take(10);

        let results_text = results.map(|x| {
            let mut elem = x;
            while elem.value().name() != "div" || elem.value().attr("lang").is_none() {
                elem = ElementRef::wrap(elem.parent().unwrap()).unwrap();
            }

            let texts = elem.text().collect::<Vec<_>>();
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
                description: Google::get_description(texts),
            }
        });

        let ret = results_text.collect();
        Ok(ret)
    }

    fn name(&self) -> String {
        "Google".to_string()
    }
}

fn is_base_domain(text: &str) -> bool {
    text.contains(".")
        && (text.starts_with("http://") || text.starts_with("https://"))
        && text.matches("/").count() == 2
}

fn is_url_representation(text: &str) -> bool {
    text.starts_with(" › ")
}

fn is_combo_url(texts: &[&str], idx: usize) -> bool {
    texts.len() > idx + 1 && is_base_domain(texts[idx]) && is_url_representation(texts[idx + 1])
}

fn is_combo_title_and_url(texts: &[&str], idx: usize) -> bool {
    texts.len() > idx + 2 && is_combo_url(texts, idx + 1)
}

impl Google {
    fn get_description(texts: Vec<&str>) -> String {
        println!("{:?}", texts);
        // first text is the title
        let texts = &texts[1..];
        let mut description = String::new();
        let mut do_continues = 0;
        let mut adjusted_idx_base = 0;

        for (idx, text) in texts.iter().enumerate() {
            println!("process {}", text);
            if do_continues > 0 {
                do_continues -= 1;
                continue;
            }
            if is_combo_title_and_url(texts, idx) {
                do_continues = 2;
                adjusted_idx_base = idx + 3;
                continue;
            }
            if is_combo_url(texts, idx) {
                do_continues = 1;
                adjusted_idx_base = idx + 2;
                continue;
            }

            if idx != adjusted_idx_base && !text.starts_with(" ") && texts[idx - 1].ends_with(".") {
                println!("out {}", text);
                break;
            }
            description.push_str(text);
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
