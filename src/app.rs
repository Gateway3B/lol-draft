use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};
use thaw::{Button, ButtonSize, ConfigProvider, Flex};
use crate::draft::draft::Draft;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body class="max-h-screen overflow-hidden">
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/lol-draft.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>
        
        // content for this welcome page
        <ConfigProvider>
            <Router>
                <main>
                    <Routes fallback=|| "Page not found.".into_view()>
                        <Route path=StaticSegment("") view=HomePage/>
                        <Route path=StaticSegment("draft") view=Draft/>
                    </Routes>
                </main>
            </Router>
        </ConfigProvider>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let count = RwSignal::new(1);
    let on_click = move |_| *count.write() += 1;

    view! {
        <div class="flex items-center flex-col">
            <h1>"Welcome to Leptos!"</h1>
            <Button size=ButtonSize::Medium on_click=on_click>"Click Me: " {count}</Button>
            <div class="flex flex-wrap gap-2">
                <Flex>
                    <Button size=ButtonSize::Medium><a href="../draft">{"To Draft"}</a></Button>
                </Flex>
            </div>
        </div>
    }
}
