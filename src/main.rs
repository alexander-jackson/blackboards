#[rocket::main]
async fn main() {
    setup();

    blackboards::build_rocket()
        .launch()
        .await
        .expect("Failed to launch");
}

/// Runs the setup for the server.
///
/// Sources the environment variables from `.env` and creates the logging instance.
fn setup() {
    // Populate the environment variables
    dotenv::dotenv().ok();

    if std::env::var("RUST_LOG").is_err() {
        // Set a reasonable default for logging in production
        std::env::set_var("RUST_LOG", "info,blackboards=debug,rocket=info,_=off")
    }

    tracing_subscriber::fmt::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
}
