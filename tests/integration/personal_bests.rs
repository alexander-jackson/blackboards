use crate::{cleanup_database, create_database};

use blackboards::schema::PersonalBest;

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
