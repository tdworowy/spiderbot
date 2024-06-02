use crate::bus_servo;
use crate::gpio;

use actix_web::{cookie::time::error, get, post, web, HttpResponse, Responder};
use rppal::system::DeviceInfo;
use serde::{Deserialize, Serialize};

use bus_servo::uart;
use gpio::{gpio_status, servo};

#[get("/get_bus_servo_bytes")]
pub async fn get_bus_servo_bytes() -> impl Responder {
    match uart::read_uart() {
        Ok(bus_servo_bytes) => HttpResponse::Ok().body(bus_servo_bytes.to_string()),
        Err(error) => {
            eprintln!("{:?}", error);
            HttpResponse::InternalServerError().body("Serialization error")
        }
    }
}

#[get("/get_gpio_status")]
pub async fn gpio_status_api() -> impl Responder {
    match DeviceInfo::new() {
        Ok(device) => {
            let gpio_data = gpio_status::get_gpio_data(device.model());
            match serde_json::to_string(&gpio_data) {
                Ok(json_data) => HttpResponse::Ok().json(json_data),
                Err(error) => {
                    eprintln!("{:?}", error);
                    HttpResponse::InternalServerError().body("Serialization error")
                }
            }
        }
        Err(error) => {
            eprintln!("{:?}", error);
            HttpResponse::InternalServerError().body("Error")
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct ServoTestParameters {
    period_ms: u64,
    pulse_min_us: u64,
    pulse_neutral_us: u64,
    pulse_max_us: u64,
    step_by: usize,
}

#[post("/test_servo")]
pub async fn test_servo(parameters: web::Json<ServoTestParameters>) -> impl Responder {
    let _parameters = parameters.into_inner();
    match servo::test_servo(
        _parameters.period_ms,
        _parameters.pulse_min_us,
        _parameters.pulse_neutral_us,
        _parameters.pulse_max_us,
        _parameters.step_by,
    ) {
        Ok(()) => HttpResponse::Ok(),
        Err(error) => {
            eprintln!("{:?}", error);
            HttpResponse::InternalServerError()
        }
    }
}
