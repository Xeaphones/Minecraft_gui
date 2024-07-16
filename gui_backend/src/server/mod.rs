mod router;

use actix_web::{middleware::DefaultHeaders, App, HttpServer};
use std::io;

use router::route;

pub async fn start_server(bind_addr: &str) -> io::Result<()> {
    println!("Running on {}.", bind_addr);
    HttpServer::new(|| {
        App::new()
        .configure(route)
    })
    .bind(bind_addr)
    .expect("Failed to bind address.")
    .run()
    .await
}