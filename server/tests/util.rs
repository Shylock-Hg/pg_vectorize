pub mod common {
    use actix_http::Request;
    use actix_service::Service;
    use actix_web::test;
    use actix_web::{App, Error, dev::ServiceResponse, web};
    use rand::prelude::*;
    use vectorize_core::init;
    #[cfg(test)]
    pub async fn get_test_app() -> impl Service<Request, Response = ServiceResponse, Error = Error>
    {
        let cfg = vectorize_core::config::Config::from_env();
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(5)
            .connect(&cfg.database_url)
            .await
            .expect("unable to connect to postgres");
        init::init_project(&pool, Some(&cfg.database_url))
            .await
            .expect("Failed to initialize project");
        test::init_service(
            App::new()
                .app_data(web::Data::new(pool))
                .configure(vectorize_server::server::route_config),
        )
        .await
    }

    // Initialize test environment without creating Actix test service
    // For use with reqwest-based tests that hit a running server
    #[cfg(test)]
    pub async fn init_test_environment() {
        let cfg = vectorize_core::config::Config::from_env();
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(5)
            .connect(&cfg.database_url)
            .await
            .expect("unable to connect to postgres");
        init::init_project(&pool, Some(&cfg.database_url))
            .await
            .expect("Failed to initialize project");
    }

    // creates a table in the vectorize_test schema
    pub async fn create_test_table() -> String {
        let cfg = vectorize_core::config::Config::from_env();
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(2)
            .connect(&cfg.database_url)
            .await
            .expect("unable to connect to postgres");

        sqlx::query("create schema if not exists vectorize_test;")
            .execute(&pool)
            .await
            .expect("unable to create vectorize_test schema");

        let mut rng = rand::rng();
        let test_num = rng.random_range(1..1000);

        let table = format!("test_table_{}", test_num);
        sqlx::query(
            format!(
                "create table if not exists vectorize_test.{} (id serial primary key, content text, updated_at timestamptz);",
                table
            )
            .as_str(),
        )
        .execute(&pool)
        .await
        .expect("unable to create test table");

        for record in ["pizza", "pencil", "airplane"] {
            sqlx::query(
                format!(
                    "insert into vectorize_test.{} (content, updated_at) values ('{}', now());",
                    table, record
                )
                .as_str(),
            )
            .execute(&pool)
            .await
            .expect("unable to insert test data");
        }

        table
    }
}
