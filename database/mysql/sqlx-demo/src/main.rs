// https://juejin.cn/post/7239739777688092728

use sqlx::mysql::{MySqlConnection, MySqlPoolOptions};
use sqlx::{Acquire, FromRow, Row};

#[derive(Debug, FromRow)]
struct User {
    id: i32,
    name: String,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv::dotenv().ok();
    let db_url = dotenv::var("DATABASE_URL").unwrap_or(String::new());
    let pool = MySqlPoolOptions::new()
        .max_connections(1000)
        .connect(&db_url)
        .await?;
    let mut conn = pool.acquire().await?;

    // select_demo(&mut conn).await?;
    // insert_demo(&mut conn).await?;
    // update_demo(&mut conn).await?;
    // delete_demo(&mut conn).await?;
    tx_demo(&mut conn).await?;

    Ok(())
}

async fn select_demo(conn: &mut MySqlConnection) -> Result<(), sqlx::Error> {
    let mut rows = sqlx::query("SELECT id, name FROM users")
        .map(|row: sqlx::mysql::MySqlRow| User {
            id: row.get(0),
            name: row.get(1),
        })
        .fetch_all(&mut *conn)
        .await?;

    // let mut rows = sqlx::query_as::<_, User>("SELECT id, name FROM users")
    //     .fetch_all(&mut *conn)
    //     .await?;

    for row in rows.iter() {
        println!("{:?}", row);
    }

    Ok(())
}

async fn insert_demo(conn: &mut MySqlConnection) -> Result<(), sqlx::Error> {
    let user = User {
        id: 0,
        name: "John".to_string(),
    };

    let result = sqlx::query("INSERT INTO users (name, created_at) VALUES (?, 1653448155)")
        .bind(user.name)
        .execute(&mut *conn)
        .await?;

    // let result = sqlx::query_with::<_, User>(
    //     "INSERT INTO users (name, created_at) VALUES (?, 1653448155)",
    //     user,
    // )
    // .execute(&mut *conn)
    // .await?;

    println!("{:?}", result);
    Ok(())
}

async fn update_demo(conn: &mut MySqlConnection) -> Result<(), sqlx::Error> {
    let user = User {
        id: 1,
        name: "John".to_string(),
    };

    let result = sqlx::query("UPDATE users SET name = ? WHERE id = ?")
        .bind(user.name)
        .bind(user.id)
        .execute(&mut *conn)
        .await?;

    // let result = sqlx::query_with::<_, User>("UPDATE users SET name = :name WHERE id = :id", user)
    //     .execute(&mut *conn)
    //     .await?;

    println!("{:?}", result);

    Ok(())
}

async fn delete_demo(conn: &mut MySqlConnection) -> Result<(), sqlx::Error> {
    let result = sqlx::query("DELETE FROM users WHERE id = ?")
        .bind(2)
        .execute(&mut *conn)
        .await?;

    // let user = User {
    //     id: 2,
    //     name: "".to_string(),
    // };
    //
    // let result = sqlx::query_with::<_, User>("DELETE FROM users WHERE id = :id", user)
    //     .execute(&mut *conn)
    //     .await?;

    println!("{:?}", result);

    Ok(())
}

async fn tx_demo(conn: &mut MySqlConnection) -> Result<(), sqlx::Error> {
    let mut tx = conn.begin().await?;

    let user = User {
        id: 0,
        name: "John".to_string(),
    };

    let result = sqlx::query("INSERT INTO users (name, created_at) VALUES (?, 1653448155)")
        .bind(user.name)
        .execute(&mut *tx)
        .await?;

    println!("{:?}", result);

    tx.commit().await?;

    Ok(())
}
