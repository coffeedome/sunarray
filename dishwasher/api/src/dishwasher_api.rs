use warp::Filter;

#[tokio::main]
pub async fn dishwasher_api() {
    let hi = warp::path("hello")
        .and(warp::path::param())
        .and(warp::header("user-agent"))
        .map(|param: String, agent: String| format!("Hello {}, whose agent is {}", param, agent));

    warp::serve(hi).run(([127, 0, 0, 1], 3030)).await;
}
