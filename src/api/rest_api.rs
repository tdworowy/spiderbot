use crate::gpio;

use actix_web::{get, post, HttpResponse, Responder};
use rppal::system::DeviceInfo;

use gpio::gpio_status;

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
