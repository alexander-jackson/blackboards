fn main() {
    dotenv::dotenv().unwrap();

    let rocket = sessions::build_rocket();
    rocket.launch();
}
