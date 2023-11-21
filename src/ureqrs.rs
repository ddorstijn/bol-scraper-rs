pub fn scrape_ureq(product_id: &str, offer_id: &str) {
    let agent = ureq::AgentBuilder::new().user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:109.0) Gecko/20100101 Firefox/119.0").build();

    let respo = agent
        .get("https://www.bol.com/nl/order/basket/addItems.html")
        .query("productId", product_id)
        .query("offerId", offer_id)
        .query("quantity", "1")
        .set("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8")
        .set("Accept-Language", "en,nl;q=0.7,en-US;q=0.3")
        .set("Accept-Encoding", "gzip, deflate, br")
        .set("DNT", "1")
        .set("Connection", "keep-alive")
        .set("Upgrade-Insecure-Requests", "1")
        .set("Sec-Fetch-Dest", "document")
        .set("Sec-Fetch-Mode", "navigate")
        .set("Sec-Fetch-Site", "none")
        .set("Sec-Fetch-User", "?1")
        .call()
        .unwrap();

    for name in respo.headers_names() {
        println!("{}: {}", name, respo.header(&name).unwrap());
    }

    // let xsrf_token = respo.header("X-XSRF-TOKEN").unwrap();

    let resp = agent
        .get("https://www.bol.com/nl/rnwy/basket/state")
        .set("Accept", "*/*")
        .set("Accept-Language", "en,nl;q=0.7,en-US;q=0.3")
        .set("Accept-Encoding", "gzip, deflate, br")
        .set("Referer", "https://www.bol.com/nl/nl/basket")
        .set("DNT", "1")
        .set("Connection", "keep-alive")
        .set("Sec-Fetch-Dest", "empty")
        .set("Sec-Fetch-Mode", "cors")
        .set("Sec-Fetch-Site", "same-origin")
        .call()
        .unwrap();

    println!("{:?}", resp.into_string());
    // println!("{:?}", resp.into_string().unwrap());
}
