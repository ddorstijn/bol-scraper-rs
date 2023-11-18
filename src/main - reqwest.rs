use std::sync::Arc;

use reqwest::{header::{HeaderMap, HOST, REFERER, ACCEPT, USER_AGENT, ACCEPT_LANGUAGE, ACCEPT_ENCODING, CONNECTION, TE}, cookie::Jar};
use serde::Deserialize;

#[derive(Deserialize)]
struct ItemRow {
    pub id: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct BolBasketState {
    pub item_rows: Vec<ItemRow>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let client = reqwest::ClientBuilder::new().cookie_store(true).build().unwrap();
    let product_id = "930000015305363";
    let offer_id = "0";

    scrape(product_id, offer_id).await;

    // let _res = client
    //     .get(format!("https://www.bol.com/nl/order/basket/addItems.html?productId={}&offerId={}&quantity=1", product_id, offer_id)).send().await.unwrap();
    
    // let response = client.get("https://www.bol.com/nl/rnwy/basket/state").send().await.unwrap();
    // let json: BolBasketState = response.json().await.unwrap();
    // let id = &json.item_rows.get(0).unwrap().id;

    // println!("{:?}", id);
    // println!("{:?}", response.text().await.unwrap());
    // client.patch("");

    Ok(())
}

async fn scrape(product_id: &str, offer_id: &str) {
    let jar = Arc::new(Jar::default());
    let client = reqwest::ClientBuilder::new().cookie_store(true).cookie_provider(jar.clone()).build().unwrap();

    let mut headers = HeaderMap::new();
    headers.insert(HOST, "www.bol.com".parse().unwrap());
    headers.insert(REFERER, "https://www.bol.com/nl/nl/basket".parse().unwrap());
    headers.insert(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:120.0) Gecko/20100101 Firefox/120.0".parse().unwrap());
    headers.insert(ACCEPT, "*/*".parse().unwrap());
    headers.insert(ACCEPT_LANGUAGE, "en,nl;q=0.7,en-US;q=0.3".parse().unwrap());
    headers.insert(ACCEPT_ENCODING, "gzip, deflate, br".parse().unwrap());
    headers.insert(CONNECTION, "keep-alive".parse().unwrap());
    headers.insert(TE, "trailers".parse().unwrap());
    
    let _response = client
        .get("https://www.bol.com/nl/order/basket/addItems.html")
        .query(&[("productId",product_id), ("offerId", offer_id), ("quantity", "1")])
        .headers(headers)
        .send().await.unwrap();

    let resp = client.get("https://www.bol.com/nl/rnwy/basket/state").send().await.unwrap();
    println!("{:?}", resp.cookies());
}