#[macro_use] extern crate lazy_static;
#[macro_use] extern crate prometheus;
#[macro_use] extern crate quick_error;

use warp::Filter;
use routes::*;
use tokio::time;
use std::time::Duration;

mod database;
mod error;
mod routes;


#[tokio::main]
async fn main() {

    let routes = ping().or(person().or(db_query()));

    let timer = tokio::spawn(async move {

        loop {
            tokio::time::delay_for(Duration::from_secs(5)).await;
            println!("Here!");
        }
    });
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
    timer.await;

}