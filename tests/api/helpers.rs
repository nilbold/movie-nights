use std::net::TcpListener;

use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;

use movie_nights::config::{AppConfig, Database as DatabaseConfig};

pub struct TestApp {
    pub address: String,
    //pub port: u16,
    pub api: reqwest::Client,
}

pub async fn configure_database(config: &DatabaseConfig) -> PgPool {
    let mut connection = PgConnection::connect(&config.conncetion_string_without_name())
        .await
        .expect("failed to connect to postgres");

    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.name).as_str())
        .await
        .expect("failed to create database");

    let db_pool = PgPool::connect(&config.connection_string())
        .await
        .expect("failed to connect to postgres");
    sqlx::migrate!("./migrations")
        .run(&db_pool)
        .await
        .expect("failed to migrate the database");

    db_pool
}

pub async fn spawn_app() -> TestApp {
    // binding to port 0 requests a random, available port from the OS
    let listener = TcpListener::bind("127.0.0.1:0").expect("failed to bind to random port");
    let port = listener.local_addr().unwrap().port();

    let mut config = AppConfig::read().expect("could not read app config");
    config.database.name = Uuid::new_v4().to_string();

    let db_pool = configure_database(&config.database).await;

    let server = movie_nights::run(listener, db_pool).expect("failed to bind address");
    let _ = tokio::spawn(server);

    let client = reqwest::Client::builder().build().unwrap();

    TestApp {
        address: format!("http://localhost:{}", port),
        //port: port,
        api: client,
    }
}
