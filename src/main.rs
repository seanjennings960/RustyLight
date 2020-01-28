use warp::Filter;

#[tokio::main]
async fn main() {
    let cors = warp::cors()
        .allow_origin("http://localhost:8000")
        .allow_methods(vec!["GET", "POST"]);

    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name))
        .with(cors);
    warp::serve(hello)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
