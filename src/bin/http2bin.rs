extern crate hyper;

static URL: &'static str = "https://http2bin.org/get";

fn main() {
    let client = hyper::Client::new();
    let mut res = client.get(URL).send().unwrap();

    println!("Response: {}", res.status);
    println!("Headers:\n{}", res.headers);
    std::io::copy(&mut res, &mut std::io::stdout()).unwrap();
}