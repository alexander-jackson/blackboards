//! Allows modifications of the `nominations` table in the database.

use crate::schema::Pool;

/// Represents a row in the `nominations` table.
#[derive(Copy, Clone, Debug, Serialize)]
pub struct Nomination {
    /// The identifier of the exec position.
    pub position_id: i32,
    /// The identifier of the candidate.
    pub warwick_id: i32,
}

/// Helper struct for (warwick_id, name) combinations.
#[derive(Clone, Debug, Serialize)]
pub struct NamedNominationForPosition {
    /// The identifier of the candidate.
    pub warwick_id: i32,
    /// The name of the candidate.
    pub name: String,
}

impl Nomination {
    /// Inserts the [`Nomination`] into the database.
    pub async fn insert(&self, pool: &mut Pool) -> sqlx::Result<()> {
        sqlx::query!(
            "INSERT INTO nominations (position_id, warwick_id) VALUES ($1, $2)",
            self.position_id,
            self.warwick_id
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    /// Gets all [`Nomination`] entries in the database.
    pub async fn get_results(pool: &mut Pool) -> sqlx::Result<Vec<Self>> {
        sqlx::query_as!(Self, "SELECT * FROM nominations")
            .fetch_all(pool)
            .await
    }

    /// Gets all the [`Nomination`] entries for a position identifier.
    pub async fn for_position_with_names(
        position_id: i32,
        pool: &mut Pool,
    ) -> sqlx::Result<Vec<NamedNominationForPosition>> {
        sqlx::query_as!(
            NamedNominationForPosition,
            r#"
                SELECT n.warwick_id AS warwick_id, name
                FROM nominations n
                INNER JOIN candidates c ON n.warwick_id = c.warwick_id
                WHERE c.elected IS false AND n.position_id = $1
            "#,
            position_id
        )
        .fetch_all(pool)
        .await

        // nominations::dsl::nominations
        //     .filter(nominations::dsl::position_id.eq(position_id))
        //     .inner_join(
        //         candidates::dsl::candidates
        //             .on(candidates::dsl::warwick_id.eq(nominations::dsl::warwick_id)),
        //     )
        //     .filter(candidates::dsl::elected.eq(false))
        //     .select((nominations::dsl::warwick_id, candidates::dsl::name))
        //     .get_results::<(i32, String)>(conn)
    }
}
