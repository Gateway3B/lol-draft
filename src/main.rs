
use cfg_if::cfg_if;

cfg_if! { if #[cfg(feature = "ssr")] {
    use axum::Router;
    use leptos::{logging::log, prelude::*};
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use migration::{Migrator, MigratorTrait};
    use sea_orm::Database;
    use dotenv::dotenv;
    use std::env;
    use lol_draft::{app::*, AppState};

    #[tokio::main]
    async fn main() {
        dotenv().ok();
        let db = Database::connect(env::var("DATABASE_URL").expect("DATABASE_URL env var doesn't exist.")).await.expect("Couldn't connect to db.");
        Migrator::up(&db, None).await.expect("Couldn't run database migrations.");

        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .with_test_writer()
            .init();

        let conf = get_configuration(None).unwrap();
        let addr = conf.leptos_options.site_addr;
        let leptos_options = conf.leptos_options;
        // Generate the list of routes in your Leptos App
        let routes = generate_route_list(App);

        let app_state = AppState {
            db: db.clone()
        };            

        let app = Router::new()
            .leptos_routes_with_context(
                &leptos_options,
                routes,
                move || provide_context(app_state.clone()),
                {
                    let leptos_options = leptos_options.clone();
                    move || shell(leptos_options.clone())
                }
            )
            .fallback(leptos_axum::file_and_error_handler(shell))
            .with_state(leptos_options);

        // run our app with hyper
        // `axum::Server` is a re-export of `hyper::Server`
        log!("listening on http://{}", &addr);
        let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
        axum::serve(listener, app.into_make_service())
            .await
            .unwrap();
    }
} else {
    pub fn main() {
        // no client-side main function
        // unless we want this to work with e.g., Trunk for pure client-side testing
        // see lib.rs for hydration function instead
    }
}}
