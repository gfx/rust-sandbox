extern crate hyper;

fn create_client() -> hyper::Client {
    let mut client = hyper::Client::new();
    client.set_read_timeout(Some(std::time::Duration::from_millis(1000)));
    client.set_write_timeout(Some(std::time::Duration::from_millis(1000)));
    return client;
}

fn main() {
    let url = "https://http2bin.org/get";
    let ua = "rust-sandbox/1.0".to_string();

    let client = create_client();
    let request_builder = client.get(url)
        .header(hyper::header::UserAgent(ua));
    let mut res = request_builder.send().unwrap();

    println!("Response: {}", res.status);
    println!("Headers:\n{}", res.headers);
    std::io::copy(&mut res, &mut std::io::stdout()).unwrap();
}