mod scraping_test;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let result = scraping_test::get_reqwest().await?;
    scraping_test::try_parse_html(&result);
    Ok(())
}
