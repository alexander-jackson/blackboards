#[rocket::main]
async fn main() {
    dotenv::dotenv().unwrap();

    let filters = vec![
        ("blackboards", log::LevelFilter::Trace),
        ("rocket", log::LevelFilter::Debug),
    ];
    blackboards::setup_logger_with_filters(filters);

    let rocket = blackboards::build_rocket();
    rocket.launch().await.expect("Failed to launch");
}
