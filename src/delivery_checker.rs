use reqwest::Url;

pub async fn check_if_product_is_home_deliverable(
    product_id: &str,
    zipcode: &str,
    token: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
    let url = Url::parse_with_params(
        "https://api.ingka.ikea.com/cia/availabilities/ru/fr",
        &[("itemNos", product_id), ("zip", zipcode)],
    )?;

    let res = reqwest::Client::new()
        .get(url)
        .header("x-client-id", token)
        .header("accept", "application/json;version=2")
        .send()
        .await?;

    let json = res.json::<serde_json::Value>().await?;
    let is_available = &json["availabilities"][0]["availableForHomeDelivery"]
        .as_bool()
        .unwrap();

    return Ok(*is_available);
}
