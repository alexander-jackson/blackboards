fn main() {
    dotenv::dotenv().ok();

    let rocket = sessions::build_rocket();
    rocket.launch();
}
