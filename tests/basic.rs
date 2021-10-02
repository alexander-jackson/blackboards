use sqlx::pool::PoolConnection;
use sqlx::{migrate::Migrator, pool::Pool, Postgres};
use uuid::Uuid;

use blackboards::context;
use blackboards::schema::{custom_types, PersonalBest, Registration, Session};

static MIGRATOR: Migrator = sqlx::migrate!();
static BASE_URL: &str = "postgres://postgres:password@localhost:5433";

fn get_test_db_name(uuid: Uuid) -> String {
    format!("test_db_{}", uuid.as_u128())
}

async fn insert_sessions(conn: &mut PoolConnection<Postgres>) -> sqlx::Result<()> {
    let sessions = vec![
        Session {
            id: 1,
            title: String::from("title"),
            start_time: custom_types::DateTime::new(0),
            spaces: 10,
        },
        Session {
            id: 2,
            title: String::from("full"),
            start_time: custom_types::DateTime::new(100),
            spaces: 2,
        },
    ];

    for session in sessions {
        session.insert(conn).await;
    }

    Ok(())
}

async fn insert_registrations(conn: &mut PoolConnection<Postgres>) -> sqlx::Result<()> {
    let registrations = vec![
        Registration {
            session_id: 2,
            warwick_id: 1,
            name: String::from("Dan"),
        },
        Registration {
            session_id: 2,
            warwick_id: 2,
            name: String::from("James"),
        },
    ];

    for registration in registrations {
        registration.insert(conn).await?;
    }

    Ok(())
}

async fn insert_personal_bests(conn: &mut PoolConnection<Postgres>) -> sqlx::Result<()> {
    let personal_bests = vec![
        PersonalBest {
            warwick_id: 1,
            name: String::from("Dan"),
            squat: Some(180.0),
            bench: None,
            deadlift: Some(210.0),
            snatch: Some(45.0),
            clean_and_jerk: None,
            show_pl: true,
            show_wl: true,
        },
        PersonalBest {
            warwick_id: 2,
            name: String::from("James"),
            squat: Some(150.0),
            bench: Some(97.5),
            deadlift: Some(175.0),
            snatch: None,
            clean_and_jerk: None,
            show_pl: true,
            show_wl: false,
        },
        PersonalBest {
            warwick_id: 3,
            name: String::from("Michael"),
            squat: None,
            bench: None,
            deadlift: None,
            snatch: Some(70.0),
            clean_and_jerk: Some(95.0),
            show_pl: false,
            show_wl: true,
        },
    ];

    for personal_best in personal_bests {
        personal_best.insert(conn).await?;
    }

    Ok(())
}

async fn insert_test_data(conn: &mut PoolConnection<Postgres>) -> sqlx::Result<()> {
    insert_sessions(conn).await?;
    insert_registrations(conn).await?;
    insert_personal_bests(conn).await?;

    Ok(())
}

async fn create_pool(uuid: Uuid) -> sqlx::Result<Pool<Postgres>> {
    // Create a pool for the new database
    let conn_str = format!("{}/test_db_{}", BASE_URL, uuid.as_u128());
    let pool: Pool<Postgres> = Pool::connect(&conn_str).await?;

    let mut conn = pool.acquire().await?;

    // Run the migrations to get it in the right state
    MIGRATOR.run(&mut conn).await?;

    // Insert everything into the database
    insert_test_data(&mut conn).await?;

    Ok(pool)
}

async fn create_database() -> sqlx::Result<(Pool<Postgres>, Uuid)> {
    // Connect to Postgres itself
    let pool: Pool<Postgres> = Pool::connect(BASE_URL).await?;

    let mut conn = pool.acquire().await?;

    // Generate a unique identifier for the test
    let uuid = Uuid::new_v4();
    let query = format!("CREATE DATABASE {}", get_test_db_name(uuid));

    // Create the database itself
    sqlx::query(&query).execute(&mut conn).await?;

    let pool = create_pool(uuid).await?;

    Ok((pool, uuid))
}

async fn cleanup_database(
    pool: Pool<Postgres>,
    conn: PoolConnection<Postgres>,
    uuid: Uuid,
) -> sqlx::Result<()> {
    // Drop the active connection
    drop(conn);

    // Close the pool itself
    pool.close().await;

    // Connect to Postgres itself
    let pool: Pool<Postgres> = Pool::connect(BASE_URL).await?;

    let mut conn = pool.acquire().await?;

    // Delete the database
    let query = format!("DROP DATABASE {}", get_test_db_name(uuid));

    sqlx::query(&query).execute(&mut conn).await?;

    Ok(())
}

#[tokio::test]
async fn sessions_can_be_queried() -> sqlx::Result<()> {
    let (pool, uuid) = create_database().await?;
    let mut conn = pool.acquire().await?;
    let sessions = Session::get_results(&mut conn).await?;

    let expected = vec![
        context::Session {
            id: 1,
            title: String::from("title"),
            start_time: custom_types::DateTime::new(0),
            remaining_spaces: Some(10),
        },
        context::Session {
            id: 2,
            title: String::from("full"),
            start_time: custom_types::DateTime::new(100),
            remaining_spaces: Some(0),
        },
    ];

    assert_eq!(sessions, expected);

    cleanup_database(pool, conn, uuid).await?;

    Ok(())
}

#[tokio::test]
async fn sessions_can_be_deleted() -> sqlx::Result<()> {
    let (pool, uuid) = create_database().await?;
    let mut conn = pool.acquire().await?;

    // Delete the session in the database
    Session::delete(1, &mut conn).await?;

    // Ensure it got deleted
    let sessions = Session::get_results(&mut conn).await?;
    let expected = vec![context::Session {
        id: 2,
        title: String::from("full"),
        start_time: custom_types::DateTime::new(100),
        remaining_spaces: Some(0),
    }];

    assert_eq!(sessions, expected);

    cleanup_database(pool, conn, uuid).await?;

    Ok(())
}

#[tokio::test]
async fn sessions_can_be_found() -> sqlx::Result<()> {
    let (pool, uuid) = create_database().await?;
    let mut conn = pool.acquire().await?;

    let session = Session::find(1, &mut conn).await?;
    let expected = context::Session {
        id: 1,
        title: String::from("title"),
        start_time: custom_types::DateTime::new(0),
        remaining_spaces: Some(10),
    };

    assert_eq!(session, Some(expected));

    cleanup_database(pool, conn, uuid).await?;

    Ok(())
}

#[tokio::test]
async fn sessions_can_be_checked_for_capacity() -> sqlx::Result<()> {
    let (pool, uuid) = create_database().await?;
    let mut conn = pool.acquire().await?;

    let full = Session::is_full(1, &mut conn).await?;
    assert!(!full);

    let full = Session::is_full(2, &mut conn).await?;
    assert!(full);

    cleanup_database(pool, conn, uuid).await?;

    Ok(())
}

#[tokio::test]
async fn powerlifting_pbs_can_be_queried() -> sqlx::Result<()> {
    let (pool, uuid) = create_database().await?;
    let mut conn = pool.acquire().await?;

    let pbs = PersonalBest::get_pl(&mut conn).await?;
    let expected = vec![
        PersonalBest {
            warwick_id: 1,
            name: String::from("Dan"),
            squat: Some(180.0),
            bench: None,
            deadlift: Some(210.0),
            snatch: Some(45.0),
            clean_and_jerk: None,
            show_pl: true,
            show_wl: true,
        },
        PersonalBest {
            warwick_id: 2,
            name: String::from("James"),
            squat: Some(150.0),
            bench: Some(97.5),
            deadlift: Some(175.0),
            snatch: None,
            clean_and_jerk: None,
            show_pl: true,
            show_wl: false,
        },
    ];

    assert_eq!(expected, pbs);

    cleanup_database(pool, conn, uuid).await?;

    Ok(())
}

#[tokio::test]
async fn weightlifting_pbs_can_be_queried() -> sqlx::Result<()> {
    let (pool, uuid) = create_database().await?;
    let mut conn = pool.acquire().await?;

    let pbs = PersonalBest::get_wl(&mut conn).await?;
    let expected = vec![
        PersonalBest {
            warwick_id: 1,
            name: String::from("Dan"),
            squat: Some(180.0),
            bench: None,
            deadlift: Some(210.0),
            snatch: Some(45.0),
            clean_and_jerk: None,
            show_pl: true,
            show_wl: true,
        },
        PersonalBest {
            warwick_id: 3,
            name: String::from("Michael"),
            squat: None,
            bench: None,
            deadlift: None,
            snatch: Some(70.0),
            clean_and_jerk: Some(95.0),
            show_pl: false,
            show_wl: true,
        },
    ];

    assert_eq!(expected, pbs);

    cleanup_database(pool, conn, uuid).await?;

    Ok(())
}

#[tokio::test]
async fn all_pbs_can_be_queried() -> sqlx::Result<()> {
    let (pool, uuid) = create_database().await?;
    let mut conn = pool.acquire().await?;

    let pbs = PersonalBest::get_results(&mut conn).await?;
    let expected = (
        vec![
            PersonalBest {
                warwick_id: 1,
                name: String::from("Dan"),
                squat: Some(180.0),
                bench: None,
                deadlift: Some(210.0),
                snatch: Some(45.0),
                clean_and_jerk: None,
                show_pl: true,
                show_wl: true,
            },
            PersonalBest {
                warwick_id: 2,
                name: String::from("James"),
                squat: Some(150.0),
                bench: Some(97.5),
                deadlift: Some(175.0),
                snatch: None,
                clean_and_jerk: None,
                show_pl: true,
                show_wl: false,
            },
        ],
        vec![
            PersonalBest {
                warwick_id: 1,
                name: String::from("Dan"),
                squat: Some(180.0),
                bench: None,
                deadlift: Some(210.0),
                snatch: Some(45.0),
                clean_and_jerk: None,
                show_pl: true,
                show_wl: true,
            },
            PersonalBest {
                warwick_id: 3,
                name: String::from("Michael"),
                squat: None,
                bench: None,
                deadlift: None,
                snatch: Some(70.0),
                clean_and_jerk: Some(95.0),
                show_pl: false,
                show_wl: true,
            },
        ],
    );

    assert_eq!(expected, pbs);

    cleanup_database(pool, conn, uuid).await?;

    Ok(())
}
