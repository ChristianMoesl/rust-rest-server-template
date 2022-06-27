use axum::{Extension, Router, Server};
use server::fallback::fallback;
use server::open_api::open_api;
use server::todo_controller::todo;
use server::todo_repo::PostgresTodoRepo;
use server::todo_service::{TodoService, TodoServiceImpl};
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let db_connection_str =
        std::env::var("DATABASE_URL").expect("DATABASE_URL should be set as environment variable");
    let port = std::env::var("PORT")
        .unwrap_or("3000".to_string())
        .parse::<u16>()
        .expect("PORT should have a small value");

    // setup connection pool
    let pool = Arc::new(
        PgPoolOptions::new()
            .max_connections(5)
            .connect_timeout(Duration::from_secs(3))
            .connect(&db_connection_str)
            .await
            .expect("can connect to database"),
    );

    let todo_repo = Arc::new(PostgresTodoRepo::new(pool.clone()));
    let todo_service: Arc<dyn TodoService> = Arc::new(TodoServiceImpl::new(todo_repo.clone()));

    // build our application with a route
    let app = Router::new()
        .merge(fallback())
        .merge(open_api())
        .merge(todo())
        .layer(Extension(todo_service));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::debug!("listening on {addr}");
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
