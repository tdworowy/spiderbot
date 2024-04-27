use crate::gpio;

use actix_web::{cookie::time::error, get, post, HttpResponse, Responder, web};
use serde::{Deserialize, Serialize};
use rppal::system::DeviceInfo;

use gpio::{gpio_status, servo};

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
}

#[post("/test_servo")]
pub async fn test_servo(parameters: web::Json<ServoTestParameters>) -> impl Responder {
    let _parameters = parameters.into_inner();
    match servo::test_servo(
        _parameters.period_ms,
        _parameters.pulse_min_us,
        _parameters.pulse_neutral_us,
        _parameters.pulse_max_us,
    ) {
        Ok(()) => HttpResponse::Ok(),
        Err(error) => {
            eprintln!("{:?}", error);
            HttpResponse::InternalServerError()
        }
    }
}
