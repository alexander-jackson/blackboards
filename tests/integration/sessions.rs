use crate::{cleanup_database, create_database};

use blackboards::context;
use blackboards::schema::{custom_types, Session};

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
