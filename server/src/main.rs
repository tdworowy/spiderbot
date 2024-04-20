mod api;
mod gpio;
use api::rest_api;
use gpio::gpio_status;

use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    match gpio_status::print_all_headers() {
        Ok(_) => {} //println!("Ok!"),
        Err(error) => println!("{:?}", error),
    };

    HttpServer::new(|| App::new().service(rest_api::gpio_status_api))
        .bind(("0.0.0.0", 3000))?
        .run()
        .await
}
