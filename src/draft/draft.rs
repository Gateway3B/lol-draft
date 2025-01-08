use codee::string::JsonSerdeCodec;
use csv::WriterBuilder;
use gloo_timers::callback::Timeout;
use leptos::prelude::*;
use leptos::Params;
use leptos_router::hooks::use_navigate;
use leptos_router::params::Params;
use leptos_router::hooks::use_params;
use leptos::task::spawn_local;
use leptos_use::storage::use_local_storage;
use leptos_ws::{provide_websocket, ServerSignal};
use serde::{Deserialize, Serialize};
use strum_macros::Display;
use strum_macros::EnumIs;
use thaw::*;
use strum::IntoEnumIterator;
use uuid::Uuid;
use web_sys::js_sys;
use std::{str::FromStr, string::ToString};
use crate::draft::completed::save_draft;
use crate::entity::champion;
use crate::api::Role;
use crate::{Draft, Turn};

#[derive(Serialize, Deserialize, Params, PartialEq, Debug)]
struct DraftParams {
    draft_id: Option<String>,
    team_id: Option<String>,
}

#[derive(Display, EnumIs)]
enum Team {
    Blue,
    Red,
    Spectator,
}

#[component]
pub fn Draft() -> impl IntoView {
    let params = use_params::<DraftParams>();

    let draft_id = move || {
        params
            .read()
            .as_ref()
            .ok()
            .and_then(|params| params.draft_id.clone())
            .unwrap_or_default()
    };

    let team_id = move || {
        params
            .read()
            .as_ref()
            .ok()
            .and_then(|params| params.team_id.clone())
    };

    cfg_if::cfg_if! { if #[cfg(feature = "hydrate")] {
        let host = window().location().host().unwrap_or(String::from("localhost:3000"));
        let websocket_url = match window().location().protocol().unwrap_or(String::from("http:")).as_str() {
            "https:" => format!("wss://{}/ws", host),
            "http:" => format!("ws://{}/ws", host),
            _ => String::from("ws://localhost:3000/ws")
        };
    } else {
        let websocket_url = String::from("ws://localhost:3000/ws");
    }};
    provide_websocket(&websocket_url);

    let navigate = use_navigate();
    let draft_exists = OnceResource::new(check_for_draft(draft_id()));
    Effect::new(move |_| {
        let Some(draft_exists) = draft_exists.get() else { return; };
        let Ok(draft_exists) = draft_exists else { return; };
        if draft_exists { return; }
        navigate("/", Default::default());
    });

    let selected_role = RwSignal::new(Role::default().to_string());
    let delay = RwSignal::new(false);
    let search = RwSignal::new(String::new());
    
    let server_draft = ServerSignal::new(draft_id(), Draft::default()).unwrap();
    let draft = RwSignal::new(Draft::default());

    let server_draft_timer = ServerSignal::new(format!("{}timer", draft_id()), 30).unwrap();
    let draft_timer = RwSignal::new(30);

    let navigate = use_navigate();
    let done = RwSignal::new(false);
    Effect::new(move |_| {
        let draft = draft.get();
        if draft.turn.is_post_draft() {
            spawn_local(async move {
                let _ = save_draft(draft).await;
                done.set(true);
            });
        }
    });

    let (_, drafts_set, _) = use_local_storage::<Vec<crate::entity::draft::Model>, JsonSerdeCodec>("Drafts");
    Effect::new(move |_| {
        if done.get() {
            drafts_set.update(|drafts| {
                let draft: crate::entity::draft::Model = draft.get_untracked().into();
                
                drafts.push(draft);
            });
            navigate(&format!("/completed/{}", draft_id()), Default::default());
        }
    });

    Effect::new(move |_| {
        draft_timer.set(server_draft_timer.get());
    });

    Effect::new(move |_| {
        draft.set(server_draft.get());
    });

    cfg_if::cfg_if! { if #[cfg(feature = "hydrate")] {
        let origin = window().location().origin().unwrap_or(String::from("http://localhost:3000"));
    } else {
        let origin = String::from("http://localhost:3000");
    }};
    let spectator_url = Signal::derive(move || format!("{}/draft/{}", origin.clone(), draft.get().draft_id));
    let blue_url = Signal::derive(move || format!("{}/{}", spectator_url.get(), draft.get().blue_id));
    let red_url = Signal::derive(move || format!("{}/{}", spectator_url.get(), draft.get().red_id));
    
    let champions = Resource::new(|| (), move |_| async move {
        if let Ok(champs) = get_champions().await {
            champs
        } else {
            vec![]
        }
    });

    Effect::new(move |_| {
        let _ = champions.get();
        let timeout = Timeout::new(1_000, move || {
            delay.set(true);
        });
        timeout.forget();
    });

    let pick_image = move |turn: Turn| Some(draft.get().get_pick_image(turn));

    let team = move || {
        let team_id = match team_id() {
            Some(team_id) => team_id,
            None => {
                return Team::Spectator;
            }
        };
        let team_id = match Uuid::from_str(&team_id) {
            Ok(team_id) => team_id,
            Err(_) => {
                return Team::Spectator;
            }
        };

        let blue = team_id == draft.get().blue_id;
        let red = team_id == draft.get().red_id;

        match (blue, red) {
            (true, false) => Team::Blue,
            (false, true) => Team::Red,
            (_, _) => Team::Spectator,

        }
    };

    let link_copied = RwSignal::new(false);
    Effect::new(move |_| {
        if draft.get().draft_id.to_string() == "00000000-0000-0000-0000-000000000000" ||
            !draft.get().turn.is_pre_draft() ||
            !team().is_spectator() ||
            link_copied.get()
        {
            return;
        }

        cfg_if::cfg_if! { if #[cfg(feature = "hydrate")] {
            let _ = window().navigator().clipboard().write_text(&format!("Blue: {}\nSpectator: {}\nRed: {}", blue_url.get(), spectator_url.get(), red_url.get()));
    
            let toaster = ToasterInjection::expect_context();
            toaster.dispatch_toast(move || view! {
                <Toast>
                    <ToastTitle>"Links Copied To Clipboard"</ToastTitle>
                </Toast>
            }, ToastOptions::default().with_position(ToastPosition::Bottom));

            link_copied.set(true);
        }}
    });

    let is_turn = move || {
        team().is_blue() && draft.get().turn.is_blue() ||
        team().is_red() && draft.get().turn.is_red()
    };

    let is_ready = move || {
        team().is_blue() && draft.get().blue_ready ||
        team().is_red() && draft.get().red_ready
    };

    let download_csv = move || {
        let csv_data = create_csv(draft.get_untracked());
        let uint8_array = js_sys::Uint8Array::from(csv_data.as_bytes());
        let blob = web_sys::Blob::new_with_u8_array_sequence(&js_sys::Array::of1(&uint8_array)).unwrap();
        let url = web_sys::Url::create_object_url_with_blob(&blob).unwrap();
        let a = window().document().unwrap().create_element("a").unwrap();
        use leptos::wasm_bindgen::JsCast;
        let a: web_sys::HtmlAnchorElement = a.dyn_into().unwrap();
        a.set_href(&url);
        a.set_download("draft.csv");
        a.click();
    };

    view! {
        <Grid cols=4>
            <GridItem class="max-h-screen overflow-hidden flex flex-col items-center blueborders">
                <video autoplay loop muted class="rotate-180 h-4" class:transparent=move || !(draft.get().turn.is_blue() || (draft.get().turn.is_pre_draft() && draft.get().blue_ready) || draft.get().turn.is_post_draft())>
                    <source src="https://raw.communitydragon.org/pbe/plugins/rcp-fe-lol-static-assets/global/default/videos/long-progress-bar-main-loop.webm" type="video/webm"/>
                </video>
                <div class="justify-evenly h-[5%] flex mt-1">
                    <Image class="w-fit no-drag aspect-square" class:selected=move || draft.get().turn.is_blue_ban_1() class:blue=move || draft.get().blue_ban_1.is_none() src=MaybeProp::derive(move || pick_image(Turn::BlueBan1)) fit=ImageFit::Fill shape=ImageShape::Rounded/>
                    <Image class="w-fit no-drag aspect-square" class:selected=move || draft.get().turn.is_blue_ban_2() class:blue=move || draft.get().blue_ban_2.is_none() src=MaybeProp::derive(move || pick_image(Turn::BlueBan2)) fit=ImageFit::Fill shape=ImageShape::Rounded/>
                    <Image class="w-fit no-drag aspect-square" class:selected=move || draft.get().turn.is_blue_ban_3() class:blue=move || draft.get().blue_ban_3.is_none() src=MaybeProp::derive(move || pick_image(Turn::BlueBan3)) fit=ImageFit::Fill shape=ImageShape::Rounded/>
                </div>
                <Image class="mt-1 mb-1 w-fit no-drag h-[16.5%] aspect-video" class:selected=move || draft.get().turn.is_blue_pick_1() class:blue=move || draft.get().blue_pick_1.is_none() src=MaybeProp::derive(move || pick_image(Turn::BluePick1)) fit=ImageFit::Fill shape=ImageShape::Circular/>
                <Image class="mt-1 mb-1 w-fit no-drag h-[16.5%] aspect-video" class:selected=move || draft.get().turn.is_blue_pick_2() class:blue=move || draft.get().blue_pick_2.is_none() src=MaybeProp::derive(move || pick_image(Turn::BluePick2)) fit=ImageFit::Fill shape=ImageShape::Circular/>
                <Image class="mt-1 mb-1 w-fit no-drag h-[16.5%] aspect-video" class:selected=move || draft.get().turn.is_blue_pick_3() class:blue=move || draft.get().blue_pick_3.is_none() src=MaybeProp::derive(move || pick_image(Turn::BluePick3)) fit=ImageFit::Fill shape=ImageShape::Circular/>
                <div class="justify-evenly h-[5%] flex">
                    <Image class="w-fit no-drag aspect-square" class:selected=move || draft.get().turn.is_blue_ban_4() class:blue=move || draft.get().blue_ban_4.is_none() src=MaybeProp::derive(move || pick_image(Turn::BlueBan4)) fit=ImageFit::Fill shape=ImageShape::Rounded/>
                    <Image class="w-fit no-drag aspect-square" class:selected=move || draft.get().turn.is_blue_ban_5() class:blue=move || draft.get().blue_ban_5.is_none() src=MaybeProp::derive(move || pick_image(Turn::BlueBan5)) fit=ImageFit::Fill shape=ImageShape::Rounded/>
                </div>
                <Image class="mt-1 mb-1 w-fit no-drag h-[16.5%] aspect-video" class:selected=move || draft.get().turn.is_blue_pick_4() class:blue=move || draft.get().blue_pick_4.is_none() src=MaybeProp::derive(move || pick_image(Turn::BluePick4)) fit=ImageFit::Fill shape=ImageShape::Circular/>
                <Image class="mt-1 mb-1 w-fit no-drag h-[16.5%] aspect-video" class:selected=move || draft.get().turn.is_blue_pick_5() class:blue=move || draft.get().blue_pick_5.is_none() src=MaybeProp::derive(move || pick_image(Turn::BluePick5)) fit=ImageFit::Fill shape=ImageShape::Circular/>
            </GridItem>
            <GridItem column=2 class="max-h-screen overflow-hidden">
                <Flex justify=FlexJustify::Center class="pt-4 pb-4 h-[5%]">
                    <TabList selected_value=selected_role>
                        {
                            Role::iter().map(|role| {
                                view! {
                                    <Tab value=role.to_string()>
                                        {role.to_string()}
                                    </Tab>
                                }
                            }).collect_view()
                        }
                    </TabList>
                    <Input value=search placeholder="Search"/>
                </Flex>
                <Scrollbar class="!h-[90%] fade">
                    <div class="flex flex-wrap justify-center" class:redborders=move || draft.get().turn.is_red() class:blueborders=move || draft.get().turn.is_blue()>
                        <Suspense
                            fallback=|| skeleton_view()
                        >
                            {
                                move || if !delay.get() {
                                    skeleton_view().into_any()
                                } else {
                                    view! {}.into_any()
                                }
                            }
                            {
                                move || match champions.get() {
                                    None => skeleton_view().into_any(),
                                    Some(champion) => champion.into_iter().map(|champion| {
                                        let role: Role = Role::from_str(&selected_role.get()).unwrap_or_default();
                                        let show = (champion.roles.roles.contains(&role) || role.is_all()) &&
                                            delay.get() &&
                                            champion.name.to_lowercase().contains(&search.get().to_lowercase()) &&
                                            !draft.get().is_champ_chosen(champion.id);
                                        let is_hovered = draft.get().current_pick().is_some_and(|id| champion.id == id);
                                        view! {
                                            <Image
                                                style:display=move || if show { "block" } else { "none" }
                                                class:selected=move || is_hovered
                                                on:click=move |_| if !is_turn() { return; } else { spawn_local(async move { let _ = select_pick(draft_id(), team_id().unwrap_or_default(), champion.id).await; })}
                                                class="m-2 !w-[75px] !h-[75px] hover:border-4 no-drag !cursor-default"
                                                src=format!("https://raw.communitydragon.org/latest/plugins/rcp-be-lol-game-data/global/default/v1/champion-icons/{}.png", champion.id)
                                                fit=ImageFit::Fill shape=ImageShape::Rounded
                                            />
                                        }
                                    }).collect_view().into_any()
                                }
                            }
                        </Suspense>
                    </div>
                </Scrollbar>
                {
                    move || match (team().is_spectator(), is_turn(), is_ready(), draft.get().turn) {
                        (true, _, _, Turn::PreDraft) => view! {
                            <Flex justify=FlexJustify::Center align=FlexAlign::Center class="!h-[5%]">
                                <Button
                                    appearance=ButtonAppearance::Secondary
                                    on:click=move |_| { let _ = window().navigator().clipboard().write_text(&blue_url.get()); }
                                    size=ButtonSize::Large
                                >"Copy Blue Link"</Button>
                                <Button
                                    appearance=ButtonAppearance::Secondary
                                    on:click=move |_| { let _ = window().navigator().clipboard().write_text(&spectator_url.get()); }
                                    size=ButtonSize::Large
                                >"Copy Spectator Link"</Button>
                                <Button
                                    appearance=ButtonAppearance::Secondary
                                    on:click=move |_| { let _ = window().navigator().clipboard().write_text(&red_url.get()); }
                                    size=ButtonSize::Large
                                >"Copy Red Link"</Button>
                            </Flex>
                        }.into_any(),
                        (false, _, false, Turn::PreDraft) => view! {
                            <Flex justify=FlexJustify::Center align=FlexAlign::Center class="!h-[5%]">
                                <Button
                                    appearance=ButtonAppearance::Primary
                                    on:click=move |_| spawn_local(async move { let _ = ready(draft_id(), team_id().unwrap_or_default()).await; })
                                    size=ButtonSize::Large
                                >{ move || if team().is_blue() { "Ready Blue" } else { "Ready Red" } }</Button>
                            </Flex>
                        }.into_any(),
                        (false, _, true, Turn::PreDraft) => view! {
                            <Flex justify=FlexJustify::Center align=FlexAlign::Center class="!h-[5%]">
                                <Button
                                    appearance=ButtonAppearance::Secondary
                                    size=ButtonSize::Large
                                    disabled=true
                                    class="!cursor-default"
                                >{ move || if team().is_blue() { "Waiting On Red" } else { "Waiting On Blue" } }</Button>
                            </Flex>
                        }.into_any(),
                        (false, true, _, _) => view! {
                            <Flex justify=FlexJustify::SpaceEvenly align=FlexAlign::Center class="!h-[5%]">
                                <Button
                                    appearance=ButtonAppearance::Secondary shape=ButtonShape::Circular disabled=true size=ButtonSize::Large
                                    class="!bg-blue-500 !cursor-default"
                                ><b style:color="black">{ move || if draft.get().turn.is_blue() { draft_timer.get().clamp(0, 30).to_string() } else { "".to_string() } }</b></Button>
                                <Button
                                    appearance=ButtonAppearance::Primary
                                    disabled=Signal::derive(move || draft.get().current_pick().is_none())
                                    on:click=move |_| spawn_local(async move { let _ = next_turn(draft_id(), team_id().unwrap_or_default()).await; })
                                    size=ButtonSize::Large
                                >"Confirm"</Button>
                                <Button
                                    appearance=ButtonAppearance::Secondary shape=ButtonShape::Circular disabled=true size=ButtonSize::Large
                                    class="!bg-red-500 !cursor-default"
                                ><b style:color="black">{ move || if draft.get().turn.is_red() { draft_timer.get().clamp(0, 30).to_string() } else { "".to_string() } }</b></Button>
                            </Flex>
                        }.into_any(),
                        (_, _, _, Turn::PostDraft) => view! {
                            <Flex justify=FlexJustify::Center align=FlexAlign::Center class="!h-[5%]">
                                <Button
                                    appearance=ButtonAppearance::Secondary
                                    size=ButtonSize::Large
                                    class="!cursor-default"
                                    on:click=move |_| download_csv()
                                >"Download CSV"</Button>
                            </Flex>
                        }.into_any(),
                        (_, _, _, _) => view! {
                            <Flex justify=FlexJustify::SpaceEvenly align=FlexAlign::Center class="!h-[5%]">
                                <Button
                                    appearance=ButtonAppearance::Secondary shape=ButtonShape::Circular disabled=true size=ButtonSize::Large
                                    class="!bg-blue-500 !cursor-default"
                                    ><b style:color="black">{ move || if draft.get().turn.is_blue() { draft_timer.get().clamp(0, 30).to_string() } else { "".to_string() } }</b></Button>
                                <Button
                                    appearance=ButtonAppearance::Primary class="!cursor-default" disabled=true
                                >{ move || if draft.get().turn.is_blue() { "Blue Turn" } else { "Red Turn" } }</Button>
                                <Button
                                    appearance=ButtonAppearance::Secondary shape=ButtonShape::Circular disabled=true size=ButtonSize::Large
                                    class="!bg-red-500 !cursor-default"
                                ><b style:color="black">{ move || if draft.get().turn.is_red() { draft_timer.get().clamp(0, 30).to_string() } else { "".to_string() } }</b></Button>
                            </Flex>
                        }.into_any(),
                    }
                }
            </GridItem>
            <GridItem class="max-h-screen overflow-hidden flex flex-col items-center redborders">
                <video autoplay loop muted class="red h-4" class:transparent=move || !(draft.get().turn.is_red() || (draft.get().turn.is_pre_draft() && draft.get().red_ready) || draft.get().turn.is_post_draft())>
                    <source src="https://raw.communitydragon.org/pbe/plugins/rcp-fe-lol-static-assets/global/default/videos/long-progress-bar-main-loop.webm" type="video/webm"/>
                </video>
                <div class="justify-evenly h-[5%] flex mt-1">
                    <Image class="w-fit no-drag aspect-square" class:selected=move || draft.get().turn.is_red_ban_1() src=MaybeProp::derive(move || pick_image(Turn::RedBan1)) fit=ImageFit::Fill shape=ImageShape::Rounded/>
                    <Image class="w-fit no-drag aspect-square" class:selected=move || draft.get().turn.is_red_ban_2() src=MaybeProp::derive(move || pick_image(Turn::RedBan2)) fit=ImageFit::Fill shape=ImageShape::Rounded/>
                    <Image class="w-fit no-drag aspect-square" class:selected=move || draft.get().turn.is_red_ban_3() src=MaybeProp::derive(move || pick_image(Turn::RedBan3)) fit=ImageFit::Fill shape=ImageShape::Rounded/>
                </div>
                <Image class="mt-1 mb-1 w-fit no-drag h-[16.5%] aspect-video" class:selected=move || draft.get().turn.is_red_pick_1() src=MaybeProp::derive(move || pick_image(Turn::RedPick1)) fit=ImageFit::Fill shape=ImageShape::Circular/>
                <Image class="mt-1 mb-1 w-fit no-drag h-[16.5%] aspect-video" class:selected=move || draft.get().turn.is_red_pick_2() src=MaybeProp::derive(move || pick_image(Turn::RedPick2)) fit=ImageFit::Fill shape=ImageShape::Circular/>
                <Image class="mt-1 mb-1 w-fit no-drag h-[16.5%] aspect-video" class:selected=move || draft.get().turn.is_red_pick_3() src=MaybeProp::derive(move || pick_image(Turn::RedPick3)) fit=ImageFit::Fill shape=ImageShape::Circular/>
                <div class="justify-evenly h-[5%] flex">
                    <Image class="w-fit no-drag aspect-square" class:selected=move || draft.get().turn.is_red_ban_4() src=MaybeProp::derive(move || pick_image(Turn::RedBan4)) fit=ImageFit::Fill shape=ImageShape::Rounded/>
                    <Image class="w-fit no-drag aspect-square" class:selected=move || draft.get().turn.is_red_ban_5() src=MaybeProp::derive(move || pick_image(Turn::RedBan5)) fit=ImageFit::Fill shape=ImageShape::Rounded/>
                </div>
                <Image class="mt-1 mb-1 w-fit no-drag h-[16.5%] aspect-video" class:selected=move || draft.get().turn.is_red_pick_4() src=MaybeProp::derive(move || pick_image(Turn::RedPick4)) fit=ImageFit::Fill shape=ImageShape::Circular/>
                <Image class="mt-1 mb-1 w-fit no-drag h-[16.5%] aspect-video" class:selected=move || draft.get().turn.is_red_pick_5() src=MaybeProp::derive(move || pick_image(Turn::RedPick5)) fit=ImageFit::Fill shape=ImageShape::Circular/>
            </GridItem>
        </Grid>
    }
}

fn create_csv(draft: Draft) -> String {
    let mut writer = WriterBuilder::new().from_writer(vec![]);
    let _ = writer.serialize(draft);
    let csv = String::from_utf8(writer.into_inner().unwrap_or_default()).unwrap_or_default();
    csv
}

fn skeleton_view() -> impl IntoView {
    view! {
        <Skeleton class="flex flex-wrap justify-center">
            {
                (0..169).into_iter().map(|_| {
                    view! {
                        <SkeletonItem class="m-2 !w-[75px] !h-[75px]"/>
                    }
                }).collect_view()
            }
        </Skeleton>
    }
}

#[server(GetChampions)]
pub async fn get_champions() -> Result<Vec<champion::Model>, ServerFnError> {
    use sea_orm::*;
    let db = use_context::<crate::AppState>().ok_or_else(|| ServerFnError::new("Database connection missing."))?.db;

    champion::Entity::find().all(&db).await.map_err(|err| ServerFnError::new(err.to_string()))
}

#[server]
async fn check_for_draft(draft_id: String) -> Result<bool, ServerFnError> {
    let ss: leptos_ws::server_signals::ServerSignals = use_context::<crate::AppState>().ok_or_else(|| ServerFnError::new("Database connection missing."))?.server_signals;
    if !ss.contains(&draft_id).await {
        return Ok(false);
    }

    use leptos_ws::ServerSignal;
    let draft: ServerSignal<Draft> = ServerSignal::new(draft_id.clone(), Draft::default()).unwrap();
    let draft_value = draft.get();

    if &draft_value.draft_id.to_string() == "00000000-0000-0000-0000-000000000000" {
        return Ok(false);
    }

    Ok(true)
}

#[server]
async fn ready(draft_id: String, team_id: String) -> Result<(), ServerFnError> {
    use leptos_ws::ServerSignal;

    let draft: ServerSignal<Draft> = ServerSignal::new(draft_id, Draft::default()).unwrap();
    let draft_value = draft.get();

    let Ok(team_id) = Uuid::from_str(&team_id) else { return Err(ServerFnError::new("Invalid team_id.")); };

    if !(draft_value.blue_id == team_id || draft_value.red_id == team_id) {
        return Err(ServerFnError::new("Does not have the correct team uuid for the selection."));
    }

    if draft_value.blue_id == team_id {
        draft.update(move |value| value.blue_ready = true);
    }

    if draft_value.red_id == team_id {
        draft.update(move |value| value.red_ready = true);
    }

    let draft_value = draft.get();
    if draft_value.blue_ready && draft_value.red_ready {
        draft.update(move |value| value.next_turn());
    }

    Ok(())
}

#[server]
async fn select_pick(draft_id: String, team_id: String, pick: u32) -> Result<(), ServerFnError> {
    use leptos_ws::ServerSignal;

    let draft: ServerSignal<Draft> = ServerSignal::new(draft_id, Draft::default()).unwrap();

    let Ok(team_id) = Uuid::from_str(&team_id) else { return Err(ServerFnError::new("Invalid team_id.")); };
    let draft_value = draft.get();
    if !((draft_value.turn.is_blue() && draft_value.blue_id == team_id) || (draft_value.turn.is_red() && draft_value.red_id == team_id)) {
        return Err(ServerFnError::new("Does not have the correct team uuid for the selection."));
    }

    draft.update(move |value| value.select_pick(pick));
    Ok(())
}

#[server]
async fn next_turn(draft_id: String, team_id: String) -> Result<(), ServerFnError> {
    use leptos_ws::ServerSignal;

    let draft: ServerSignal<Draft> = ServerSignal::new(draft_id.clone(), Draft::default()).unwrap();
    let draft_timer: ServerSignal<i32> = ServerSignal::new(format!("{draft_id}timer"), 30).unwrap();

    let Ok(team_id) = Uuid::from_str(&team_id) else { return Err(ServerFnError::new("Invalid team_id.")); };
    let draft_value = draft.get();
    if !((draft_value.turn.is_blue() && draft_value.blue_id == team_id) || (draft_value.turn.is_red() && draft_value.red_id == team_id)) {
        return Err(ServerFnError::new("Does not have the correct team uuid for the selection."));
    }
    
    draft.update(move |value| value.next_turn());
    draft_timer.update(move |value| *value = 30);
    Ok(())
}