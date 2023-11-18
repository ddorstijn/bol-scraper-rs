use ureq::Response;

fn main() {
    let product_id = "930000015305363";
    let offer_id = "0";

    scrape(product_id, offer_id);
}

fn scrape(product_id: &str, offer_id: &str) {
    let agent = ureq::AgentBuilder::new().user_agent("ferris/1.0").build();

    let respo = agent
        .get("https://www.bol.com/nl/order/basket/addItems.html")
        .query("productId", product_id)
        .query("offerId", offer_id)
        .query("quantity", "1")
        .set("Host", "www.bol.com")
        .set("Referer", "https://www.bol.com/nl/nl/basket")
        .set(
            "Accept",
            "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8",
        )
        .set("Accept-Language", "en,nl;q=0.7,en-US;q=0.3")
        .set("Connection", "keep-alive")
        .call()
        .unwrap();

    let xsrf_token = respo.header("X-XSRF-TOKEN").unwrap();

    let resp = agent
        .get("https://www.bol.com/nl/rnwy/basket/state")
        .set("Host", "www.bol.com")
        .set("Accept", "*/*")
        .set("Accept-Language", "en,nl;q=0.7,en-US;q=0.3")
        .set("Accept-Encoding", "gzip, deflate, br")
        .set("Referer", "https://www.bol.com/nl/nl/basket")
        .set("X-XSRF-TOKEN", xsrf_token)
        .set("DNT", "1")
        .set("Connection", "keep-alive")
        .set("Cookie", "shopping_session_id=16ee0c3b7cc7d60afd7af8af245ee684c62226a78347e14bc0932227d2f66507; bltgSessionId=e5431c56-2177-4812-8dd0-a25c4cbc9e5e; XSRF-TOKEN=0ed4869b-6963-4bd3-9897-1140bbcc7010; BUI=w0ow4xcopjyymlgbd8yhri1atlc3ie4p; bolConsentChoices=source#OFC|version#6|int-tran#false|ext-tran#false|int-beh#false|ext-beh#false; locale=NL; language=nl-NL; P=.wspc-deployment-f6cc86b44-vz4lt; XSC=w7lrfFEIDLH1kJ3uEdjDrNXU7cu1vXmK; s_fid=6C9EB70E7DF458D8-0398530B190055EE; px_page=no%20cms%20page; px_nr=1700316478125-New; px_eVar14=Browse; px_visit=1; px_prop36=xlarge; px_prop34=consent; px_pp=winkelwagentje; px_eVar80=%7B%22journey%22%3A1%2C%22page%22%3A6%7D; s_cc=true; s_vi=[CS]v1|32AC629EE1D8B3C3-40001957E2A51418[CE]")
        .set("Sec-Fetch-Dest", "empty")
        .set("Sec-Fetch-Mode", "cors")
        .set("Sec-Fetch-Site", "same-origin")
        .set("TE", "trailers")
        .call()
        .unwrap();

    println!("{:?}", resp.into_string().unwrap());
}
