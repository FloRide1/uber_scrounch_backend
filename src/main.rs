pub mod migration;
pub mod models;
pub mod routes;
pub mod schema;
pub mod state;

use std::error::Error;

use crate::routes::command::{close_command, get_all_commands, get_command, post_command};
use crate::routes::delivery::{
    get_all_delivery_list, get_delivery, get_next_delivery, post_delivery,
};
use crate::routes::location::{get_all_location_list, get_location, get_location_ids};
use crate::routes::product::{get_all_product_list, get_product, get_product_ids};
use crate::state::AppState;
use axum::routing::{get, post, put};
use diesel::PgConnection;
use tower_http::trace::TraceLayer;

use crate::routes::oauth::{
    login::{login, login_authorized},
    microsoft::microsoft_auth,
};
use crate::routes::user::{get_user, me};

use tracing_subscriber::filter;
use tracing_subscriber::prelude::*;

#[macro_use]
extern crate log;

pub type DbConnection = PgConnection;

#[tokio::main]
async fn main() {
    // Update env
    dotenvy::dotenv().ok();

    // Launch tracing
    let filter = filter::Targets::new()
        .with_target("tower_http::trace::on_response", tracing::Level::TRACE)
        .with_target("tower_http::trace::on_request", tracing::Level::TRACE)
        .with_target("tower_http::trace::make_span", tracing::Level::DEBUG)
        .with_default(tracing::Level::INFO);

    let tracing_layer = tracing_subscriber::fmt::layer();

    tracing_subscriber::registry()
        .with(tracing_layer)
        .with(filter)
        .init();

    // Create DB Pool
    let url = std::env::var("DATABASE_URL").unwrap();
    let state = AppState::new(&url);

    // Run migrations
    state
        .pool
        .get()
        .await
        .unwrap()
        .interact(run_migrations)
        .await
        .ok();

    let app = app(state);

    let addr = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "0.0.0.0:3000".to_string());

    info!("Server started on: {}", addr);
    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn app(state: AppState) -> axum::Router {
    axum::Router::new()
        .route("/status", get(status))
        .route("/login", get(login))
        // .route("/logout", get(logout)) TODO
        .route("/auth/microsoft", get(microsoft_auth))
        .route("/auth/authorize", get(login_authorized))
        .route("/user/me", get(me))
        .route("/user/:id", get(get_user))
        .route("/product", get(get_product_ids))
        .route("/product/:id", get(get_product))
        .route("/product/all", get(get_all_product_list))
        .route("/command/close/:id", put(close_command))
        .route("/command/:id", get(get_command))
        .route("/command", get(get_all_commands))
        .route("/command", post(post_command))
        .route("/location", get(get_location_ids))
        .route("/location/:id", get(get_location))
        .route("/location/all", get(get_all_location_list))
        .route("/delivery", get(get_next_delivery))
        .route("/delivery/:id", get(get_delivery))
        .route("/delivery/all", get(get_all_delivery_list))
        .route("/delivery", post(post_delivery))
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

async fn status() -> &'static str {
    "UP"
}

fn run_migrations(
    connection: &mut impl diesel_migrations::MigrationHarness<diesel::pg::Pg>,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    use diesel_migrations::{embed_migrations, EmbeddedMigrations};

    const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");
    let res = connection.run_pending_migrations(MIGRATIONS)?;
    if res.is_empty() {
        info!("Running Migrations: {:?}", res);
    }

    let profile = std::env::var("PROFILE").unwrap_or_else(|_| "dev".to_string());

    match profile.as_str() {
        "dev" => migration::dev::run(connection),
        "test" => migration::test::run(connection),
        "prod" => migration::prod::run(connection),
        _ => panic!("Unknow profile"),
    }
}
