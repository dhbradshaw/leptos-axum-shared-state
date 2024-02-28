use axum::extract::FromRef;
use leptos::LeptosOptions;

#[derive(FromRef, Debug, Clone)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub db: sqlx::PgPool,
}
