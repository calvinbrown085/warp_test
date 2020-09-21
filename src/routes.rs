
use prometheus::{CounterVec, HistogramVec, TextEncoder, Encoder};
use serde::{Deserialize, Serialize};
use warp::Filter;
use warp::reply::Json;
use crate::database::{DoSomethingWithDatabase, DatabaseQuery};


lazy_static! {
    static ref ROUTE_COUNTER: CounterVec = register_counter_vec!(
        "example_http_requests_total",
        "Total number of HTTP requests made.",
        &["count_name", "status"]
    ).unwrap();
    static ref ROUTE_TIMER: HistogramVec = register_histogram_vec!(
        "example_http_request_duration_seconds",
        "The HTTP request latencies in seconds.",
        &["handler"]
    )
    .unwrap();
}

#[derive(Deserialize, Serialize)]
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
    ROUTE_COUNTER.with_label_values(&["ping", "2xx"]).inc();
    resp

}

pub fn db_query() -> impl Filter<Extract = (Json,), Error = warp::Rejection> + Copy {
    let timer = ROUTE_TIMER.with_label_values(&["database_query"]).start_timer();
    let resp = warp::get()
        .and(warp::path!("database-query" / u32))
        .map(|id: u32| {
            let db_query: DatabaseQuery = DatabaseQuery { id };
            warp::reply::json(&db_query.select_data().unwrap())
        });
    timer.observe_duration();
    resp
}

pub fn person() -> impl Filter<Extract = (Json,), Error = warp::Rejection> + Copy {
    let timer = ROUTE_TIMER.with_label_values(&["person"]).start_timer();
    let resp = warp::get()
        .and(warp::path!("person" / u32))
        .map(|age: u32| {
            let person = Person { name: String::from("Calvin"), age: age};
            warp::reply::json(&person)
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