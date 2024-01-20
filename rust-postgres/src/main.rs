#![allow(unused)]

use std::error::Error;

use sqlx::postgres::{PgConnection, PgPoolOptions, PgRow};
use sqlx::{Connection, FromRow, Row};
use sql_query_builder as sql;

struct User {
    id: i32,
    name: String,
}
#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let mut conn: PgConnection =
        PgConneSction::connect("postgres://girish:Harish@2023@localhost:5432/users").await?;

    let mut select: sql::Select = sql::Select::new()
        .select("id,name")
        .from("public.\"test-users\"");

    let res: Vec<PgRow> = sqlx::query(&select.as_string())
        .fetch_all(&mut conn)
        .await?;

    let result: String = res
        .iter()
        .map(|r: &PgRow| format!("{} - {}", r.get::<i32 ,_>("id"),r.get::<String, _>("name")))
        .collect::<Vec<String>>()
        .join(",");

    let select_query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> = sqlx::query("");
    let data: Vec<User> = select_query.map(|x: PgRow| User{
        id: x.get("id"),
        name: x.get("name")
    }).fetch_all(&mut conn)
    .await?
    ;

    println!("{}",result);
    Ok(())
}
