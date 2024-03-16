use scraper::{Html, Selector};
use reqwest;


pub async fn get_reqwest() -> Result<String, Box<dyn std::error::Error>>{
    let body = reqwest::get("https://www.rust-lang.org").await?.text().await?;

    Ok(body)
}

pub fn try_parse_html(html: &str){
    let document = Html::parse_document(html);
    let selector_str = "h2"; // タグ名はここで指定する
    let selector = Selector::parse(selector_str).unwrap();

    for element in document.select(&selector){
        if let Some(unicode) = element.text().next(){
            println!("{}",unicode)
        }
    }
}