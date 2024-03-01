use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {


        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos-axum-with-state.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);

    // On click, increment the count
    let on_click = move |_| set_count.update(|count| *count += 1);

    // Create a resource that will call the server function when the count changes
    let resource = create_resource(count, |c| async move { db_add_100(c).await });

    // Create a closure that will be called when the resource is updated
    // and returns a string including the value of the resource.
    let resource_report = move || {
        resource
            .get()
            .map(|value| format!("Server returned {value:?}"))
            // This loading state will only show before the first load
            .unwrap_or_else(|| "No adding yet".into())
    };

    // Create a closure to track the loading state of the resource.
    let loading = resource.loading();
    let is_loading = move || {
        if loading() {
            "Loading..."
        } else {
            "Idle.."
        }
    };

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
        <p>{resource_report}</p>
        <p>{is_loading}</p>
    }
}

#[server]
pub async fn db_add_100(count: i32) -> Result<i32, ServerFnError> {
    use crate::state::AppState;
    use sqlx::Row;

    let state: AppState = use_context::<AppState>().expect("Can't get app state from context");
    let row = sqlx::query("SELECT 100 + $1")
        .bind(count)
        .fetch_one(&state.db)
        .await?;
    Ok(row.get(0))
}
