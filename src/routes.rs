
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

struct Person {
    name: String,
    age: u32,
}

pub fn ping() -> impl Filter<Extract = (String,), Error = warp::Rejection> + Copy {
    let _timer = ROUTE_TIMER.with_label_values(&["ping"]).start_timer();
    let resp = warp::get()
        .and(warp::path!("ping"))
        .map(|| {
            format!("pong")
        });
    _timer.observe_duration();
    ROUTE_COUNTER.inc();
    resp

}

pub fn person() -> impl Filter<Extract = (String,), Error = warp::Rejection> + Copy {
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

pub fn metrics() -> impl Filter<Extract = (String,), Error = warp::Rejection> + Copy {
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

#[tokio::test]
async fn test_ping() {
    let filter = ping();



    let res = warp::test::request()
        .path("/ping")
        .reply(&filter)
        .await;

    assert_eq!(res.status(), 200);
    assert_eq!(res.body(), "pong");
}

#[tokio::test]
async fn test_person() {
    let filter = person();



    let res = warp::test::request()
        .path("/person/1")
        .reply(&filter)
        .await;

    assert_eq!(res.status(), 200);
}