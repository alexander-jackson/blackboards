use sqlx::migrate::Migrator;
use sqlx::{PgPool, Pool, Postgres};

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

/*
/// Builds the configuration for the Rocket instance.
fn config_from_env() -> Figment {
    let mut databases = HashMap::new();
    let mut urls = HashMap::new();

    let database_url =
        env::var("DATABASE_URL").expect("Failed to find `DATABASE_URL` in the environment");

    urls.insert("url", database_url);
    databases.insert("blackboards", urls);

    Figment::from(Config::default())
        .merge(Env::prefixed("ROCKET_").global())
        .merge(("log_level", "off"))
        .merge(("databases", databases))
}
*/

async fn run_migrations(pool: &Pool<Postgres>) -> sqlx::Result<()> {
    static MIGRATOR: Migrator = sqlx::migrate!();
    MIGRATOR.run(pool).await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup();

    let database_url = "postgresql://localhost:5432/postgres";
    let pool = PgPool::connect(database_url).await?;

    run_migrations(&pool)
        .await
        .expect("Failed to run migrations");

    blackboards::build_router(pool).await;

    Ok(())
}
