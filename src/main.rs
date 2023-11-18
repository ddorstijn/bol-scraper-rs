fn main() {
    let product_id = "930000015305363";
    let offer_id = "0";

    scrape(product_id, offer_id);
}

fn scrape(product_id: &str, offer_id: &str) {
    let agent = ureq::AgentBuilder::new()
        .user_agent("ferris/1.0")
        .build();

    agent
        .get("https://www.bol.com/nl/order/basket/addItems.html")
        .query("productId",product_id)
        .query("offerId", offer_id)
        .query("quantity", "1")
        .call()
        .unwrap();
}