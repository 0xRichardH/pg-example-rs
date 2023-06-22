use sqlx::types::{
    chrono::{DateTime, Utc},
    Uuid,
};
use sqlx::{postgres::PgPoolOptions, Row};

const MAX_CONNECTIONS: u32 = 5;

struct User {
    id: Uuid,
    name: String,
    email: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let url = "postgres://postgres:password@localhost:5432/rust_pg_example";
    let pool = PgPoolOptions::new()
        .max_connections(MAX_CONNECTIONS)
        .connect(url)
        .await?;

    query_example(&pool).await?;
    execute_migrate(&pool).await?;

    let user = User {
        id: Uuid::new_v4(),
        name: "John Doe".to_string(),
        email: "jKv5S@example.com".to_string(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    add_user(&pool, user).await?;

    println!("Done!");

    Ok(())
}

async fn execute_migrate(pool: &sqlx::PgPool) -> Result<(), sqlx::Error> {
    sqlx::migrate!().run(pool).await?;

    Ok(())
}

async fn query_example(pool: &sqlx::PgPool) -> Result<(), sqlx::Error> {
    let result = sqlx::query("SELECT 1 + 1 as sum").fetch_one(pool).await?;
    let sum: i32 = result.get("sum");
    println!("Hello, world! 1 + 1 = {}", sum);

    Ok(())
}

async fn add_user(pool: &sqlx::PgPool, user: User) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO users (id, name, email, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5)",
    )
    .bind(user.id)
    .bind(user.name)
    .bind(user.email)
    .bind(user.created_at)
    .bind(user.updated_at)
    .execute(pool)
    .await?;

    Ok(())
}
