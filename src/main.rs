use sqlx::{postgres::PgPoolOptions, Row};

const MAX_CONNECTIONS: u32 = 5;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let url = "postgres://postgres:password@localhost:5432/rust_pg_example";
    let pool = PgPoolOptions::new()
        .max_connections(MAX_CONNECTIONS)
        .connect(url)
        .await?;

    query_example(&pool).await?;

    Ok(())
}

async fn query_example(pool: &sqlx::PgPool) -> Result<(), sqlx::Error> {
    let result = sqlx::query("SELECT 1 + 1 as sum").fetch_one(pool).await?;
    let sum: i32 = result.get("sum");
    println!("Hello, world! 1 + 1 = {}", sum);

    Ok(())
}
