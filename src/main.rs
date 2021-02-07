fn main() {
    dotenv::dotenv().unwrap();

    let rocket = blackboards::build_rocket();
    rocket.launch();
}
