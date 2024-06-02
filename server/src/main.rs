mod api;
mod bus_servo;
mod gpio;
use api::rest_api;
use gpio::gpio_status;


use actix_web::{web::service, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    match gpio_status::print_all_headers() {
        Ok(_) => {} //println!("Ok!"),
        Err(error) => println!("{:?}", error),
    };

    HttpServer::new(|| {
        App::new()
            .service(rest_api::gpio_status_api)
            .service(rest_api::test_servo)
            .service(rest_api::get_bus_servo_bytes)
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}
