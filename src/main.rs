use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenvy::dotenv().ok();
    // Create a connection pool
    //  for MySQL/MariaDB, use MySqlPoolOptions::new()
    //  for SQLite, use SqlitePoolOptions::new()
    //  etc.
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:postgres@localhost/db")
        .await?;

    // Make a simple query to return the given parameter (use a question mark `?` instead of `$1` for MySQL/MariaDB)
    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&pool)
        .await?;

    assert_eq!(row.0, 150);
    /*
        if let Err(e) = create_metrics_table(pool.clone()).await {
            eprintln!("Error creating table: {:?}", e);
        }
    */
    let res = sqlx::query(r#"WITH per_day AS (
    SELECT
        time,
        value
    FROM kwh_day_by_day
    WHERE "time" > now() - interval '1 year'
    ORDER BY 1
), per_month AS (
    SELECT
        to_char(time, 'Mon') as month,
        sum(value) as value
    FROM per_day
    GROUP BY 1
)
SELECT
    m.month,
    m.ordinal,
    pd.value
FROM unnest(array['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun', 'Jul', 'Aug', 'Sep', 'Oct', 'Nov', 'Dec']) WITH ORDINALITY AS m(month, ordinal)
         LEFT JOIN per_month pd ON lower(pd.month) = lower(m.month)
ORDER BY ordinal"#).fetch_all(&pool).await.unwrap();
    Ok(())
}

async fn create_metrics_table(pool: Pool<Postgres>) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "CREATE TABLE metrics (
    created timestamp with time zone default now() not null,
    type_id integer                                not null,
    value   double precision                       not null
)"
    )
    .execute(&pool)
    .await?;
    sqlx::query!("SELECT create_hypertable('metrics', by_range('created'));")
        .fetch_all(&pool)
        .await?;
    Ok(())
}
