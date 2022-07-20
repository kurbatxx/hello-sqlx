use sqlx::{migrate::MigrateDatabase, query_as, sqlite::SqlitePoolOptions};
use tokio_stream::StreamExt;

#[derive(sqlx::FromRow, Debug)]
struct User {
    id: u32,
    name: String,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {

    let uri = "sqlite://data.db";
    sqlx::Sqlite::create_database(uri).await?;
    let pool = SqlitePoolOptions::new().connect(uri).await?;

    let _create_users_table = sqlx::query(
        "create table if not exists users (
    id integer primary key autoincrement,
    name text unique)",
    )
    .execute(&pool)
    .await?;

    let mvec = vec![
        User {
            id: Default::default(),
            name: "-".to_string(),
        },
        User {
            id: Default::default(),
            name: "--".to_string(),
        },
    ];

    let mut stream = tokio_stream::iter(&mvec);
    while let Some(row) = stream.next().await {
        sqlx::query(
            "INSERT INTO users (name)
            VALUES ($1)",
        )
        .bind(&row.name)
        .execute(&pool)
        .await?;
    }

    let row: (u32,) = query_as("SELECT $1").bind(150_u32).fetch_one(&pool).await?;

    dbg!(&row);

    let res = sqlx::query_as::<_, User>("SELECT id, name FROM users")
        .fetch_all(&pool)
        .await?;

    let _ = &res
        .iter()
        .for_each(|x| println!("id: {}\nname: {}", x.id, x.name));

    pool.close().await;
    Ok(())
}
