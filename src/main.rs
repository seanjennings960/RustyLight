extern crate weblights;

use warp::Filter;
use std::net::ToSocketAddrs;

use weblights::ctrl::{Mode, lights};


fn light_post(mode_str: String) -> String {
    let result = mode_str.parse::<Mode>();
    match result {
        Ok(mode) => {
            lights(mode);
            String::from("Turning lights: ") + &mode_str
        }
        Err(e) => format!("{}", e)
    }
}

#[tokio::main]
async fn main() {
    let cors = warp::cors()
        .allow_origin("http://beaglebone.local")
        .allow_origin("http://192.168.0.102")
        .allow_methods(vec!["GET", "POST"]);

    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));
    let light_ctrl = warp::post()
        .and(warp::path!("light" / String))
        .map(light_post);
    let routes = hello.or(light_ctrl).with(cors);
    let sock_addr = "192.168.0.102:3030"
        .to_socket_addrs() .unwrap()
        .next().unwrap();
    warp::serve(routes)
        .run(sock_addr)
        .await;
}
