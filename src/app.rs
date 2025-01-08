#[allow(unused_imports)]
use std::time::Duration;
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Link, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes}, hooks::use_navigate, path
};
use thaw::{ConfigProvider, Theme, ToasterProvider};

use crate::draft::{completed::CompletedDraft, draft::Draft};

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
                <Link rel="icon" type_="image/x-icon" href="https://raw.communitydragon.org/latest/plugins/rcp-fe-lol-shared-components/global/default/emerald.png"/>
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

    let theme = RwSignal::new(Theme::dark()); 

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/lol-draft.css"/>

        // sets the document title
        <Title text="Lol Draft"/>

        <a href="https://g3tech.net" target="_blank">
            <video
                style:cursor="pointer"
                class="absolute right-0 bottom-0 w-10 rounded"
                autoplay muted loop
            >
                <source src="/G3.mp4" type="video/mp4"/>
            </video>
        </a>
        
        // content for this welcome page
        <ConfigProvider theme class="!bg-transparent">
            <ToasterProvider>
                <Router>
                    <main>
                        <Routes fallback=|| "Page not found.".into_view()>
                            <Route path=path!("/") view=HomePage/>
                            <Route path=path!("/draft/:draft_id") view=Draft/>
                            <Route path=path!("/draft/:draft_id/:team_id") view=Draft/>
                            <Route path=path!("/completed/:draft_id") view=CompletedDraft/>
                        </Routes>
                    </main>
                </Router>
            </ToasterProvider>
        </ConfigProvider>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    let navigate = use_navigate();

    let redirect = OnceResource::new(create_draft());

    Effect::new(move |_| {
        let Some(redirect) = redirect.get() else { return; };
        let Ok(redirect) = redirect else { return; };
        navigate(&redirect, Default::default());
    });

    view! {}
}

#[server]
pub async fn create_draft() -> Result<String, ServerFnError> {
    use uuid::Uuid;
    use crate::Draft;
    use leptos_ws::ServerSignal;

    let draft_id = Uuid::new_v4();
    let blue_id = Uuid::new_v4();
    let red_id = Uuid::new_v4();
    let mut draft = Draft::default();
    draft.draft_id = draft_id.clone();
    draft.blue_id = blue_id;
    draft.red_id = red_id;

    let draft_signal = ServerSignal::new(draft_id.to_string(), draft.clone()).unwrap();
    let draft_timer_signal = ServerSignal::new(format!("{draft_id}timer"), 30).unwrap();

    draft_signal.update(move |value| *value = draft);

    let mut draft_subscription = draft_signal.subscribe();
    tokio::spawn(async move {
        loop {
            draft_subscription.recv().await.unwrap();
            let draft = draft_signal.get();

            if draft.blue_ready && draft.red_ready {
                break;
            }
        }

        loop {
            let draft = draft_signal.get();
            let draft_timer = draft_timer_signal.get();

            if !draft.blue_ready || !draft.red_ready {
                continue;
            }

            if draft.turn.is_post_draft() {
                break;
            }

            if draft_timer < -3 {
                draft_signal.update(move |value| value.next_turn());
                draft_timer_signal.update(move |value| *value = 30);
                continue;
            }

            let _ = tokio::time::sleep(Duration::from_secs(1)).await;
            draft_timer_signal.update(move |value| *value -= 1);
        }
    });

    let redirect = format!("/draft/{}", draft_id);
    Ok(redirect)
}
