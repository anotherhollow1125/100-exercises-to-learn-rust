use crate::db::{SqliteImpl, SqliteImplParameters};
use crate::store::StoreImpl;
use anyhow::anyhow;
use anyhow::Result;
use axum::routing::{get, post};
use axum::Router;
use shaku::module;
use sqlx::sqlite::SqlitePool;
use std::sync::Arc;
use tokio::net::TcpListener;

mod end_points;
use end_points::{create_ticket, get_all_ticket, get_ticket_by_id, update_ticket};

module! {
    StoreModule {
        components = [StoreImpl, SqliteImpl],
        providers = []
    }
}

pub struct Config {
    pub database_url: String,
}

pub async fn serve(Config { database_url }: Config) -> Result<()> {
    let pool = SqlitePool::connect(&database_url)
        .await
        .map_err(|e| anyhow!("[@{} in {}] {:?}", line!(), file!(), e))?;

    let store = StoreModule::builder()
        .with_component_parameters::<SqliteImpl>(SqliteImplParameters { pool })
        .build();

    let app = Router::new()
        .route("/tickets", get(get_all_ticket))
        .route("/tickets/id/:ticket_id", get(get_ticket_by_id))
        .route("/tickets/create", post(create_ticket))
        .route("/tickets/update", post(update_ticket))
        .with_state(Arc::new(store));

    let listener = TcpListener::bind("0.0.0.0:3000")
        .await
        .map_err(|e| anyhow!("[@{} in {}] {:?}", line!(), file!(), e))?;

    axum::serve(listener, app)
        .await
        .map_err(|e| anyhow!("[@{} in {}] {:?}", line!(), file!(), e))?;

    Ok(())
}
