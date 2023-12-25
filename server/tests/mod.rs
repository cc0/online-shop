#[tokio::test]
async fn test_home() {

    let url = "http://localhost:5000/";

    let response = reqwest::Client::new()
        .get(url)
        .send()
        .await
        .expect("Failed to send the request to the home page");

    let body_text = response.text().await
        .expect("Failed to get the body text from the home page");

    assert_eq!("Welcome", body_text);
}