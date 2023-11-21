use std::str::FromStr;

use reqwest::{
    header::{HeaderMap, ACCEPT},
    Client, Url,
};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ItemRow {
    pub id: String,
    pub quantity: i32,
    pub product_id: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct BolBasketState {
    pub item_rows: Vec<ItemRow>,
}

pub async fn scrape_reqwest(product_url: &str) {
    let cookie_store = reqwest_cookie_store::CookieStore::default();
    let cookie_store = reqwest_cookie_store::CookieStoreMutex::new(cookie_store);
    let cookie_store = std::sync::Arc::new(cookie_store);

    let mut default_headers = HeaderMap::new();
    default_headers.insert(
        "Accept-Language",
        "en,nl;q=0.7,en-US;q=0.3".parse().unwrap(),
    );
    default_headers.insert("Accept-Encoding", "gzip".parse().unwrap());
    default_headers.insert("Connection", "keep-alive".parse().unwrap());

    let client = reqwest::ClientBuilder::new()
        .cookie_provider(std::sync::Arc::clone(&cookie_store))
        .connection_verbose(true)
        .default_headers(default_headers)
        .user_agent(
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:109.0) Gecko/20100101 Firefox/119.0",
        )
        .build()
        .unwrap();

    let product_info = get_product_info(&client, product_url).await;
    println!("{}", product_info.url);

    add_to_basket(&client, product_info.url).await;

    // Get Token
    let xsrf_token = {
        let store = cookie_store.lock().unwrap();
        store
            .get("www.bol.com", "/", "XSRF-TOKEN")
            .unwrap()
            .value()
            .to_string()
    };

    let row = get_basket_state(&client, &product_info.product_id, &xsrf_token)
        .await
        .unwrap();

    change_product_quantity(&client, row.id, &xsrf_token).await;

    let row = get_basket_state(&client, &product_info.product_id, &xsrf_token)
        .await
        .unwrap();
    println!("{}", row.quantity);
}

async fn add_to_basket(client: &Client, basket_url: Url) {
    let mut basket_headers = HeaderMap::new();
    basket_headers.insert(ACCEPT, "text/html,application/xhtml+xml".parse().unwrap());

    client
        .get(basket_url)
        .headers(basket_headers)
        .send()
        .await
        .unwrap();
}

struct ProductInfo {
    url: Url,
    product_id: String,
    offer_id: String,
}

async fn get_product_info(client: &Client, product_url: &str) -> ProductInfo {
    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, "text/html,application/xhtml+xml".parse().unwrap());

    let response = client
        .get(product_url)
        .headers(headers)
        .send()
        .await
        .unwrap();

    let raw = response.text().await.unwrap();
    let dom = tl::parse(&raw, tl::ParserOptions::default()).unwrap();
    let el = dom
        .query_selector(r#"a[data-test="add-to-basket"]"#)
        .unwrap()
        .next()
        .unwrap()
        .get(dom.parser())
        .unwrap();

    let attributes = el.as_tag().unwrap().attributes();
    let href = attributes.get("href").unwrap().unwrap().as_utf8_str();
    let product_id = attributes.id().unwrap().as_utf8_str().to_string();
    let offer_id = attributes
        .get("data-offer-id")
        .unwrap()
        .unwrap()
        .as_utf8_str()
        .to_string();

    let url = String::from("https://www.bol.com/nl") + &href;
    let url = Url::parse(&url).unwrap();

    ProductInfo {
        url,
        product_id,
        offer_id,
    }
}

async fn get_basket_state(
    client: &Client,
    product_id: &str,
    xsrf_token: &String,
) -> Option<ItemRow> {
    let mut state_headers = HeaderMap::new();
    state_headers.insert("Accept", "*/*".parse().unwrap());
    state_headers.insert("X-XSRF-TOKEN", xsrf_token.parse().unwrap());

    let state_reponse = client
        .get("https://www.bol.com/nl/rnwy/basket/state")
        .headers(state_headers)
        .send()
        .await
        .unwrap();

    let basket_state: BolBasketState = state_reponse.json().await.unwrap();
    basket_state
        .item_rows
        .into_iter()
        .find(|r| r.product_id == product_id)
}

async fn change_product_quantity(client: &Client, id: String, xsrf_token: &String) {
    let patch_url = format!("https://www.bol.com/nl/rnwy/basket/v1/items/{}", id);
    let mut patch_headers = HeaderMap::new();
    patch_headers.insert("Accept", "*/*".parse().unwrap());
    patch_headers.insert("X-XSRF-TOKEN", xsrf_token.parse().unwrap());
    patch_headers.insert(
        "Content-Type",
        "application/json;charset=utf-8".parse().unwrap(),
    );

    client
        .patch(patch_url)
        .headers(patch_headers)
        .body("{\"quantity\":500}")
        .send()
        .await
        .unwrap();
}
