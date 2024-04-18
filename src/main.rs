mod utils;
use utils::gpio_status;

fn main() {
    println!("Start!");
    
    match gpio_status::print_all_headers() {
        Ok(_) => println!("Ok!"),
        Err(error) => println!("{:?}", error),
    };
    println!("End!");
}
