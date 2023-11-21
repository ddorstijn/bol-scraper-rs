pub mod curlrs;
pub mod reqwestrs;
pub mod ureqrs;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    simple_logger::SimpleLogger::new().with_level(log::LevelFilter::Trace).init().unwrap();

    let product_url = "https://www.bol.com/nl/nl/p/antraciet-tapijt-wasbaar-laagpolig-vloerkleed-met-anti-slip-koho-soft-comfort-wasbaar-op-30-160x230cm-modern-woonkamer-salon-slaapkamer-eetkamer/9300000129253393/";
    reqwestrs::scrape_reqwest(product_url).await;

    return Ok(())
}