#[macro_use] extern crate lazy_static;
#[macro_use] extern crate prometheus;

use prometheus::{Counter, HistogramVec, TextEncoder, Encoder};
use warp::Filter;

lazy_static! {
    static ref ROUTE_COUNTER: Counter = register_counter!(opts!(
        "example_http_requests_total",
        "Total number of HTTP requests made.",
        labels! {"handler" => "all",}
    )).unwrap();
    static ref ROUTE_TIMER: HistogramVec = register_histogram_vec!(
        "example_http_request_duration_seconds",
        "The HTTP request latencies in seconds.",
        &["handler"]
    )
    .unwrap();
}


#[tokio::main]
async fn main() {

    let routes = ping().or(person());

    let http_server = tokio::spawn(async move {
        warp::serve(routes)
            .run(([0, 0, 0, 0], 8080))
            .await;
    });

    let health_server = tokio::spawn(async move {
        warp::serve(ping().or(metrics()))
            .run(([0, 0, 0, 0], 8081))
            .await;
    });
    http_server.await;
    health_server.await;

}

struct Person {
    name: String,
    age: u32,
}

fn ping() -> impl Filter<Extract = (String,), Error = warp::Rejection> + Copy {
    let _timer = ROUTE_TIMER.with_label_values(&["ping"]).start_timer();
    let resp = warp::get()
        .and(warp::path!("ping"))
        .map(|| {
            format!("pong")
        });
    _timer.observe_duration();
    resp

}

fn person() -> impl Filter<Extract = (String,), Error = warp::Rejection> + Copy {
    let timer = ROUTE_TIMER.with_label_values(&["person"]).start_timer();
    let resp = warp::get()
        .and(warp::path!("person" / u32))
        .map(|age: u32| {
            let person = Person { name: String::from("Calvin"), age: age};
            format!("Person name: {}, age: {}", person.name, person.age)
        });
    timer.observe_duration();
    resp
}

fn metrics() -> impl Filter<Extract = (String,), Error = warp::Rejection> + Copy {
    warp::get()
        .and(warp::path!("metrics"))
        .map(|| {
            let mut buffer = Vec::new();
            let encoder = TextEncoder::new();
            let metric_families = prometheus::gather();
            encoder.encode(&metric_families, &mut buffer).unwrap();
            String::from_utf8(buffer.clone()).unwrap()
        })
}