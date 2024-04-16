mod utils;
use utils::gpio_status;

fn main() {
    println!("Start!");
    gpio_status::print_all_headers();
}
