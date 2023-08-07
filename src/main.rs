slint::include_modules!();

fn main() {
    let application = App::new().unwrap();
    match application.run() {
        Ok(_) => {
            println!("It seems fine.");
        }
        Err(_) => {
            println!("Uhh ohh... Something went wrong!");
        }
    }
}
