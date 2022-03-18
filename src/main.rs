#[tokio::main]
async fn main() {
    let http_proxy = reqwest::Proxy::https("http://127.0.0.1:8000").unwrap();
    let client = reqwest::ClientBuilder::new()
        .proxy(http_proxy)
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();

    let mut handles = vec![];

    for _ in 0..500 {
        handles.push(tokio::task::spawn(send_request(client.clone())));
    }
    let _ = futures::future::join_all(handles).await;
}

async fn send_request(client: reqwest::Client) {
    loop {
        let body = client.get("https://example.com/").send().await;

        if let Ok(resp) = body {
            let _ = resp.text().await;
        }
    }
}
