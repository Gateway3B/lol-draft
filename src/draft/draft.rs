use leptos::prelude::*;
use thaw::*;
use strum_macros::{Display, EnumIs, EnumIter, EnumString};
use strum::IntoEnumIterator;
use std::{str::FromStr, string::ToString};
use crate::entity::champion;
use crate::api::Role;

#[server(GetChampions)]
pub async fn get_champions() -> Result<Vec<champion::Model>, ServerFnError> {
    use sea_orm::*;
    let db = use_context::<crate::AppState>().ok_or_else(|| ServerFnError::new("Database connection missing."))?.db;

    champion::Entity::find().all(&db).await.map_err(|err| ServerFnError::new(err.to_string()))
}

#[component]
pub fn Draft() -> impl IntoView {
    let selected_role = RwSignal::new(Role::default().to_string());
    
    let champions = Resource::new(|| (), move |_| async move {
        if let Ok(champs) = get_champions().await {
            // let image_requests = champs.iter().map(|champion| {
            //     reqwest::get(format!("https://raw.communitydragon.org/latest/plugins/rcp-be-lol-game-data/global/default/v1/champion-icons/{}.png", champion.id))
            // });

            // let _ = futures::future::join_all(image_requests).await;
            champs
        } else {
            vec![]
        }
    });

    // let champions_by_role = move || {
    //     let role: Role = Role::from_str(&selected_role.get()).unwrap_or_default();
    //     if let Some(champions) = champions.get() {
    //         champions.into_iter().filter(|champion| champion.roles.roles.contains(&role) || role.is_all()).collect::<Vec<champion::Model>>()
    //     } else {
    //         vec![]
    //     }
    // };

    view! {
        <Grid cols=4>
            <GridItem>
                <Image src="https://ddragon.leagueoflegends.com/cdn/img/champion/splash/Aatrox_0.jpg" fit=ImageFit::Fill/>
                <Image src="https://ddragon.leagueoflegends.com/cdn/img/champion/splash/Aatrox_0.jpg" fit=ImageFit::Fill/>
                <Image src="https://ddragon.leagueoflegends.com/cdn/img/champion/splash/Aatrox_0.jpg" fit=ImageFit::Fill/>
                <Image src="https://ddragon.leagueoflegends.com/cdn/img/champion/splash/Aatrox_0.jpg" fit=ImageFit::Fill/>
                <Image src="https://ddragon.leagueoflegends.com/cdn/img/champion/splash/Aatrox_0.jpg" fit=ImageFit::Fill/>
            </GridItem>
            <GridItem column=2 class="max-h-screen overflow-hidden">
                <Flex>
                    <TabList selected_value=selected_role>
                        {
                            Role::iter().map(|role| {
                                view! {
                                    <Tab value=role.to_string() >
                                        { role.to_string() }
                                    </Tab>
                                }
                            }).collect_view()
                        }
                    </TabList>
                    <Input placeholder="Search"/>
                </Flex>
                <Scrollbar class="m-h-[100%]">
                    <div class="flex flex-wrap justify-center">
                        <Suspense
                            fallback=move || view! {
                                <Skeleton class="flex flex-wrap">
                                    {
                                        (0..169).into_iter().map(|_| {
                                            view! {
                                                <SkeletonItem class="m-2 !w-[75px] !h-[75px]"/>
                                            }
                                        }).collect_view()
                                    }
                                </Skeleton>
                            }
                        >
                            {
                                move || match champions.get() {
                                    None => view! {}.into_any(),
                                    Some(champion) => champion.into_iter().map(|champion| {
                                        let role: Role = Role::from_str(&selected_role.get()).unwrap_or_default();
                                        view! {
                                            <Image
                                                style:display=move || if champion.roles.roles.contains(&role) || role.is_all() { "block" } else { "none" }
                                                class="m-2 !w-[75px] !h-[75px] !rounded-xl"
                                                src=format!("https://raw.communitydragon.org/latest/plugins/rcp-be-lol-game-data/global/default/v1/champion-icons/{}.png", champion.id)
                                                fit=ImageFit::Fill
                                            />
                                        }
                                    }).collect_view().into_any()
                                }
                            }
                        </Suspense>
                    </div>
                </Scrollbar>
            </GridItem>
            <GridItem>
                <Image src="https://cdn.communitydragon.org/latest/champion/{}/splash-art/centered/skin/0" fit=ImageFit::Fill/>
                <Image src="https://ddragon.leagueoflegends.com/cdn/img/champion/splash/Aatrox_0.jpg" fit=ImageFit::Fill/>
                <Image src="https://ddragon.leagueoflegends.com/cdn/img/champion/splash/Aatrox_0.jpg" fit=ImageFit::Fill/>
                <Image src="https://ddragon.leagueoflegends.com/cdn/img/champion/splash/Aatrox_0.jpg" fit=ImageFit::Fill/>
                <Image src="https://ddragon.leagueoflegends.com/cdn/img/champion/splash/Aatrox_0.jpg" fit=ImageFit::Fill/>
            </GridItem>
            
        </Grid>
    }
}