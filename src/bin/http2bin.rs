extern crate hyper;

fn main() {
    let url = "https://http2bin.org/get";
    let ua = "rust-sandbox/1.0".to_string();

    let client = hyper::Client::new();
    let request_builder = client.get(url)
        .header(hyper::header::UserAgent(ua));
    let mut res = request_builder.send().unwrap();

    println!("Response: {}", res.status);
    println!("Headers:\n{}", res.headers);
    std::io::copy(&mut res, &mut std::io::stdout()).unwrap();
}