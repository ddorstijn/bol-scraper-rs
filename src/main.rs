use reqwest::{
    header::{HeaderMap, CONTENT_TYPE, ACCEPT},
    Client,
};
use serde::Deserialize;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    simple_logger::SimpleLogger::new()
        .with_level(log::LevelFilter::Off)
        .init()
        .unwrap();

    let product_url = "https://www.bol.com/nl/nl/p/antraciet-tapijt-wasbaar-laagpolig-vloerkleed-met-anti-slip-koho-soft-comfort-wasbaar-op-30-160x230cm-modern-woonkamer-salon-slaapkamer-eetkamer/9300000129253393/";
    let quantity = scrape(product_url).await;
    println!("Quantity: {}", quantity);

    return Ok(());
}

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

pub async fn scrape(product_url: &str) -> i32 {
    let product_info = get_product_info(product_url).await;
    
    let cookie_store = reqwest_cookie_store::CookieStore::default();
    let cookie_store = reqwest_cookie_store::CookieStoreMutex::new(cookie_store);
    let cookie_store = std::sync::Arc::new(cookie_store);

    let mut default_headers = HeaderMap::new();
    default_headers.insert("Accept-Language", "en,nl".parse().unwrap());
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

    client
        .get("https://www.bol.com/nl/order/basket/addItems.html")
        .query(&[
            ("productId", &product_info.product_id),
            ("offerId", &product_info.offer_id),
            ("quantity", &"1".to_string()),
        ])
        .send()
        .await
        .unwrap();

    // Get Token
    let xsrf_token = {
        let store = cookie_store.lock().unwrap();
        store
            .get("www.bol.com", "/", "XSRF-TOKEN")
            .unwrap()
            .value()
            .to_string()
    };

    let row = get_basket_state(&client, &product_info, &xsrf_token)
        .await
        .unwrap();

    change_product_quantity(&client, row.id, &xsrf_token).await;

    let row = get_basket_state(&client, &product_info, &xsrf_token)
        .await
        .unwrap();
   
   
   row.quantity
}

#[derive(Debug)]
struct ProductInfo {
    product_id: String,
    offer_id: String,
}

async fn get_product_info(product_url: &str) -> ProductInfo {
    let response = reqwest::get(product_url).await.unwrap();

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
    let product_id = attributes
        .get("data-product-id")
        .unwrap()
        .unwrap()
        .as_utf8_str()
        .to_string();
    let offer_id = attributes
        .get("data-offer-id")
        .unwrap()
        .unwrap()
        .as_utf8_str()
        .to_string();

    ProductInfo {
        product_id,
        offer_id,
    }
}

async fn get_basket_state(
    client: &Client,
    product_info: &ProductInfo,
    xsrf_token: &String,
) -> Option<ItemRow> {
    let state_reponse = client
        .get("https://www.bol.com/nl/rnwy/basket/state")
        .header("X-XSRF-TOKEN", xsrf_token)
        .send()
        .await
        .unwrap();

    let basket_state: BolBasketState = state_reponse.json().await.unwrap();
    basket_state
        .item_rows
        .into_iter()
        .find(|r| r.product_id == product_info.product_id)
}

async fn change_product_quantity(client: &Client, id: String, xsrf_token: &String) {
    let patch_url = format!("https://www.bol.com/nl/rnwy/basket/v1/items/{}", id);

    client
        .patch(patch_url)
        .header("X-XSRF-TOKEN", xsrf_token)
        .header(CONTENT_TYPE, "application/json;charset=utf-8")
        .header(ACCEPT, "*/*")
        .body("{\"quantity\":500}")
        .send()
        .await
        .unwrap();
}
