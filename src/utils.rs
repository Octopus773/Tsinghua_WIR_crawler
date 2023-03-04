use crate::types::SearchResult;
use reqwest::header::{ACCEPT_LANGUAGE, USER_AGENT};
use std::io::Write;

// faking Chrome 108 on macOS 10.15.7
pub static APP_USER_AGENT: &'static str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/108.0.0.0 Safari/537.36";
pub static APP_ACCEPT_LANGUAGE: &'static str = "en-US,en;q=0.9";

pub async fn save_results_to_disk(
    folder: &str,
    engine_name: &str,
    query_num: u32,
    student_id: &str,
    results: &[SearchResult],
) {
    let json = serde_json::to_string(&results).unwrap();

    let mut file = std::fs::File::create(format!(
        "SE_{}_{}_{}.json",
        engine_name, query_num, student_id
    ))
    .unwrap();
    file.write_all(json.as_bytes()).unwrap();

    std::fs::create_dir_all(folder).unwrap();

    for (idx, result) in results.iter().enumerate() {
        save_site_as_file(
            &result.url,
            &format!(
                "{}/TP_{}_{}_{}_{}",
                folder,
                engine_name,
                query_num,
                idx + 1,
                student_id
            ),
            true,
        ).await;
    }
}

pub async fn save_site_as_file(url: &str, filename: &str, auto_filetype: bool) {
    let http_client = reqwest::Client::new();
    let res = http_client
        .get(url)
        .header(USER_AGENT, APP_USER_AGENT)
        .header(ACCEPT_LANGUAGE, APP_ACCEPT_LANGUAGE)
        .send()
        .await
        .unwrap();
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
