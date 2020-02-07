use warp::Filter;
use std::net::ToSocketAddrs;

#[tokio::main]
async fn main() {
    let cors = warp::cors()
        .allow_origin("http://beaglebone.local")
        .allow_methods(vec!["GET", "POST"]);

    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name))
        .with(cors);
    let sock_addr = "beaglebone.local:3030".to_socket_addrs()
        .unwrap()
        .next().unwrap();
    warp::serve(hello)
        .run(sock_addr)
        .await;
}
