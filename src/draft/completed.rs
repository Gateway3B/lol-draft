use chrono::Local;
use codee::string::JsonSerdeCodec;
use csv::WriterBuilder;
use leptos::prelude::*;
use leptos_router::hooks::{use_navigate, use_params};
use leptos_router::params::Params;
use leptos_use::storage::{use_local_storage_with_options, UseStorageOptions};
use serde::{Deserialize, Serialize};
use thaw::*;
use web_sys::js_sys;
use crate::{entity::draft, Draft, Turn};

#[server]
pub async fn save_draft(draft: Draft) -> Result<(), ServerFnError> {
    use sea_orm::*;
    let db = use_context::<crate::AppState>().ok_or_else(|| ServerFnError::new("Database connection missing."))?.db;
    let draft: draft::Model = draft.into();
    let draft: draft::ActiveModel = draft.into();
    draft.insert(&db).await?;
    Ok(())
}

#[server(CompletedDraft, "/api", "Url", "completed_draft")]
pub async fn completed_draft(draft_id: String) -> Result<draft::Model, ServerFnError> {
    use sea_orm::*;
    let db = use_context::<crate::AppState>().ok_or_else(|| ServerFnError::new("Database connection missing."))?.db;
    let draft: Option<draft::Model> = draft::Entity::find_by_id(draft_id).one(&db).await?;
    draft.ok_or(ServerFnError::new("Draft not found."))
}

#[derive(Serialize, Deserialize, Params, PartialEq, Debug)]
struct CompletedParams {
    draft_id: Option<String>
}

fn create_csv(draft: draft::Model) -> String {
    let mut writer = WriterBuilder::new().from_writer(vec![]);
    let _ = writer.serialize(draft);
    let csv = String::from_utf8(writer.into_inner().unwrap_or_default()).unwrap_or_default();
    csv
}

#[component]
pub fn CompletedDraft() -> impl IntoView {
    let navigate = use_navigate();
    let params = use_params::<CompletedParams>();

    let draft_id = move || {
        params
            .read()
            .as_ref()
            .ok()
            .and_then(|params| params.draft_id.clone())
            .unwrap_or_default()
    };

    let draft_resource = Resource::new(|| (), move |_| completed_draft(draft_id()));

    let draft = RwSignal::new(draft::Model::default());

    let (drafts_ls, _, _) = use_local_storage_with_options::<Vec<draft::Model>, JsonSerdeCodec>("Drafts", UseStorageOptions::default());

    let drafts = RwSignal::new(Vec::<draft::Model>::new());

    Effect::new(move || {
        let mut drafts_ls = drafts_ls.get();
        drafts_ls.sort_by(|a, b| b.date_completed.cmp(&a.date_completed));
        drafts.set(drafts_ls);
    });

    Effect::new(move || {
        match draft_resource.get() {
            Some(model) => match model {
                Ok(model) => draft.set(model),
                Err(_) => (),
            },
            None => (),
        };
    });

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

    let pick_image = move |turn: Turn| Some(draft.get().get_pick_image(turn));

    view! {
        <Grid cols=5>
            <GridItem><div></div></GridItem>
            <GridItem class="max-h-screen overflow-hidden flex flex-col items-center blueborders">
                <video autoplay loop muted class="rotate-180 h-4">
                    <source src="https://raw.communitydragon.org/pbe/plugins/rcp-fe-lol-static-assets/global/default/videos/long-progress-bar-main-loop.webm" type="video/webm"/>
                </video>
                <div class="justify-evenly h-[5%] flex mt-1">
                    <Image class="w-fit no-drag aspect-square" class:blue=move || draft.get().blue_ban_1.is_none() src=MaybeProp::derive(move || pick_image(Turn::BlueBan1)) fit=ImageFit::Fill shape=ImageShape::Rounded/>
                    <Image class="w-fit no-drag aspect-square" class:blue=move || draft.get().blue_ban_2.is_none() src=MaybeProp::derive(move || pick_image(Turn::BlueBan2)) fit=ImageFit::Fill shape=ImageShape::Rounded/>
                    <Image class="w-fit no-drag aspect-square" class:blue=move || draft.get().blue_ban_3.is_none() src=MaybeProp::derive(move || pick_image(Turn::BlueBan3)) fit=ImageFit::Fill shape=ImageShape::Rounded/>
                </div>
                <Image class="mt-1 mb-1 w-fit no-drag h-[16.5%] aspect-video" class:blue=move || draft.get().blue_pick_1.is_none() src=MaybeProp::derive(move || pick_image(Turn::BluePick1)) fit=ImageFit::Fill shape=ImageShape::Circular/>
                <Image class="mt-1 mb-1 w-fit no-drag h-[16.5%] aspect-video" class:blue=move || draft.get().blue_pick_2.is_none() src=MaybeProp::derive(move || pick_image(Turn::BluePick2)) fit=ImageFit::Fill shape=ImageShape::Circular/>
                <Image class="mt-1 mb-1 w-fit no-drag h-[16.5%] aspect-video" class:blue=move || draft.get().blue_pick_3.is_none() src=MaybeProp::derive(move || pick_image(Turn::BluePick3)) fit=ImageFit::Fill shape=ImageShape::Circular/>
                <div class="justify-evenly h-[5%] flex">
                    <Image class="w-fit no-drag aspect-square" class:blue=move || draft.get().blue_ban_4.is_none() src=MaybeProp::derive(move || pick_image(Turn::BlueBan4)) fit=ImageFit::Fill shape=ImageShape::Rounded/>
                    <Image class="w-fit no-drag aspect-square" class:blue=move || draft.get().blue_ban_5.is_none() src=MaybeProp::derive(move || pick_image(Turn::BlueBan5)) fit=ImageFit::Fill shape=ImageShape::Rounded/>
                </div>
                <Image class="mt-1 mb-1 w-fit no-drag h-[16.5%] aspect-video" class:blue=move || draft.get().blue_pick_4.is_none() src=MaybeProp::derive(move || pick_image(Turn::BluePick4)) fit=ImageFit::Fill shape=ImageShape::Circular/>
                <Image class="mt-1 mb-1 w-fit no-drag h-[16.5%] aspect-video" class:blue=move || draft.get().blue_pick_5.is_none() src=MaybeProp::derive(move || pick_image(Turn::BluePick5)) fit=ImageFit::Fill shape=ImageShape::Circular/>
            </GridItem>
            
            <GridItem column=1 class="max-h-screen overflow-scroll">
                <Flex vertical=true justify=FlexJustify::Center align=FlexAlign::Center class="h-full">
                    <Button
                        appearance=ButtonAppearance::Secondary
                        size=ButtonSize::Large
                        class="!cursor-default"
                        on:click=move |_| download_csv()
                    >"Download CSV"</Button>
                    <Button
                        appearance=ButtonAppearance::Secondary
                        size=ButtonSize::Large
                        class="!cursor-default"
                        on:click=move |_| navigate("/", Default::default())
                    >"New Draft"</Button>
                    <Show
                        when=move || {
                            let draft_id = draft.get().draft_id;
                            !drafts.get().iter().any(|draft| draft.draft_id == draft_id)
                        }
                        fallback=|| view! {}
                    >
                        <Button
                            appearance=ButtonAppearance::Primary
                            size=ButtonSize::Large
                            class="!cursor-default"
                        >{ move || draft.get().date_completed.format("%Y-%m-%d %H:%M").to_string() }</Button>
                    </Show>
                    <For
                        // a function that returns the items we're iterating over; a signal is fine
                        each=move || drafts.get()
                        // a unique key for each item
                        key=|draft| draft.draft_id.clone()
                        // renders each item to a view
                        let:saved_draft
                    >
                        {
                            let navigate = use_navigate();
                            let draft_id = saved_draft.draft_id.clone();
                            view! {
                                <Button
                                    appearance=Signal::derive(move || if draft_id == draft.get().draft_id { ButtonAppearance::Primary } else { ButtonAppearance::Secondary })
                                    size=ButtonSize::Large
                                    class="!cursor-default"
                                    on:click=move |_| navigate(&format!("/completed/{}", saved_draft.draft_id), Default::default())
                                >{ saved_draft.date_completed.format("%Y-%m-%d %H:%M").to_string() }</Button>
                            }
                        }
                    </For>
                </Flex>
            </GridItem>
            
            <GridItem class="max-h-screen overflow-hidden flex flex-col items-center redborders">
                <video autoplay loop muted class="red h-4">
                    <source src="https://raw.communitydragon.org/pbe/plugins/rcp-fe-lol-static-assets/global/default/videos/long-progress-bar-main-loop.webm" type="video/webm"/>
                </video>
                <div class="justify-evenly h-[5%] flex mt-1">
                    <Image class="w-fit no-drag aspect-square" src=MaybeProp::derive(move || pick_image(Turn::RedBan1)) fit=ImageFit::Fill shape=ImageShape::Rounded/>
                    <Image class="w-fit no-drag aspect-square" src=MaybeProp::derive(move || pick_image(Turn::RedBan2)) fit=ImageFit::Fill shape=ImageShape::Rounded/>
                    <Image class="w-fit no-drag aspect-square" src=MaybeProp::derive(move || pick_image(Turn::RedBan3)) fit=ImageFit::Fill shape=ImageShape::Rounded/>
                </div>
                <Image class="mt-1 mb-1 w-fit no-drag h-[16.5%] aspect-video" src=MaybeProp::derive(move || pick_image(Turn::RedPick1)) fit=ImageFit::Fill shape=ImageShape::Circular/>
                <Image class="mt-1 mb-1 w-fit no-drag h-[16.5%] aspect-video" src=MaybeProp::derive(move || pick_image(Turn::RedPick2)) fit=ImageFit::Fill shape=ImageShape::Circular/>
                <Image class="mt-1 mb-1 w-fit no-drag h-[16.5%] aspect-video" src=MaybeProp::derive(move || pick_image(Turn::RedPick3)) fit=ImageFit::Fill shape=ImageShape::Circular/>
                <div class="justify-evenly h-[5%] flex">
                    <Image class="w-fit no-drag aspect-square" src=MaybeProp::derive(move || pick_image(Turn::RedBan4)) fit=ImageFit::Fill shape=ImageShape::Rounded/>
                    <Image class="w-fit no-drag aspect-square" src=MaybeProp::derive(move || pick_image(Turn::RedBan5)) fit=ImageFit::Fill shape=ImageShape::Rounded/>
                </div>
                <Image class="mt-1 mb-1 w-fit no-drag h-[16.5%] aspect-video" src=MaybeProp::derive(move || pick_image(Turn::RedPick4)) fit=ImageFit::Fill shape=ImageShape::Circular/>
                <Image class="mt-1 mb-1 w-fit no-drag h-[16.5%] aspect-video" src=MaybeProp::derive(move || pick_image(Turn::RedPick5)) fit=ImageFit::Fill shape=ImageShape::Circular/>
            </GridItem>
            <GridItem><div></div></GridItem>
        </Grid>
    }
}

impl From<Draft> for draft::Model {
    fn from(value: Draft) -> Self {
        draft::Model {
            draft_id: value.draft_id.to_string(),
            blue_ban_1: value.blue_ban_1,
            blue_ban_2: value.blue_ban_2,
            blue_ban_3: value.blue_ban_3,
            blue_ban_4: value.blue_ban_4,
            blue_ban_5: value.blue_ban_5,
            red_ban_1: value.red_ban_1,
            red_ban_2: value.red_ban_2,
            red_ban_3: value.red_ban_3,
            red_ban_4: value.red_ban_4,
            red_ban_5: value.red_ban_5,
            blue_pick_1: value.blue_pick_1,
            blue_pick_2: value.blue_pick_2,
            blue_pick_3: value.blue_pick_3,
            blue_pick_4: value.blue_pick_4,
            blue_pick_5: value.blue_pick_5,
            red_pick_1: value.red_pick_1,
            red_pick_2: value.red_pick_2,
            red_pick_3: value.red_pick_3,
            red_pick_4: value.red_pick_4,
            red_pick_5: value.red_pick_5,
            date_completed: Local::now().naive_local()
        }
    }
}

impl draft::Model {
    fn get_pick_image(&self, turn: Turn) -> String {
        if let Some(pick_id) = self.get_pick(&turn) {
            if turn.is_ban() {
                return format!("https://raw.communitydragon.org/latest/plugins/rcp-be-lol-game-data/global/default/v1/champion-icons/{}.png", pick_id)
            } else {
                return format!("https://cdn.communitydragon.org/latest/champion/{}/splash-art/centered/skin/0.jpg", pick_id);
            }
        }
        String::from("https://raw.communitydragon.org/latest/game/assets/ux/loadingscreen/srworlds2023loadscreen.png")
    }
    
    fn get_pick(&self, turn: &Turn) -> Option<u32> {
        match turn {
            Turn::PreDraft => None,
            Turn::BlueBan1 => self.blue_ban_1,
            Turn::BlueBan2 => self.blue_ban_2,
            Turn::BlueBan3 => self.blue_ban_3,
            Turn::BlueBan4 => self.blue_ban_4,
            Turn::BlueBan5 => self.blue_ban_5,
            
            Turn::RedBan1 => self.red_ban_1,
            Turn::RedBan2 => self.red_ban_2,
            Turn::RedBan3 => self.red_ban_3,
            Turn::RedBan4 => self.red_ban_4,
            Turn::RedBan5 => self.red_ban_5,
    
            Turn::BluePick1 => self.blue_pick_1,
            Turn::BluePick2 => self.blue_pick_2,
            Turn::BluePick3 => self.blue_pick_3,
            Turn::BluePick4 => self.blue_pick_4,
            Turn::BluePick5 => self.blue_pick_5,
    
            Turn::RedPick1 => self.red_pick_1,
            Turn::RedPick2 => self.red_pick_2,
            Turn::RedPick3 => self.red_pick_3,
            Turn::RedPick4 => self.red_pick_4,
            Turn::RedPick5 => self.red_pick_5,
            Turn::PostDraft => None
        }
    }
}
