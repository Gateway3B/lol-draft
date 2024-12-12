use serde::{Serialize, Deserialize};
use strum_macros::{Display, EnumIs, EnumIter, EnumString};
use leptos::prelude::*;
use cfg_if::cfg_if;
use crate::entity::champion;

#[derive(Default, Display, EnumIter, EnumString, PartialEq, Eq, EnumIs, Copy, Clone, Debug, Serialize, Deserialize)]
pub enum Role {
    #[default]
    All,
    Top,
    Jungle,
    Middle,
    Bottom,
    Support
}

cfg_if! { if #[cfg(feature = "ssr")] {
    use std::collections::HashMap;

    #[derive(Deserialize, Debug)]
    struct Champions {
        data: HashMap<String, Champion>
    }

    #[derive(Deserialize, Debug)]
    struct Champion {
        key: String,
        name: String
    }
}}

#[server(UpdateChampions, "/api", "Url", "update_champions")]
pub async fn update_champions() -> Result<Vec<champion::Model>, ServerFnError> {
    use crate::entity::champion;
    use strum::IntoEnumIterator;
    use std::string::ToString;

    let versions: Vec<String> = reqwest::get("https://ddragon.leagueoflegends.com/api/versions.json")
        .await?
        .json()
        .await?;

    let version = versions.first().ok_or(ServerFnError::new("Versions don't exist."))?;

    let champions: Champions = reqwest::get(format!("https://ddragon.leagueoflegends.com/cdn/{version}/data/en_US/champion.json"))
        .await?
        .json()
        .await?;
    
    let roles_text = reqwest::get(format!("https://raw.communitydragon.org/latest/plugins/rcp-fe-lol-champion-statistics/global/default/rcp-fe-lol-champion-statistics.js"))
        .await?
        .text()
        .await?;

    let json_regex = regex::Regex::new(r#"JSON.parse\('(\S+)'\)}"#)?;
    let roles_json = &json_regex
        .captures(&roles_text)
        .ok_or(ServerFnError::new("Couldn't extract roles json."))?
        .get(1)
        .ok_or(ServerFnError::new("Couldn't get roles json matching group."))?
        .as_str();

    let roles_raw: HashMap<String, HashMap<String, serde_json::Number>> = serde_json::from_str(roles_json)?;
    let mut roles = HashMap::<String, champion::Roles>::new();
    let _ = Role::iter().filter(|role| !role.is_all()).try_for_each(|role_name| {
        roles_raw.get(&role_name.to_string().to_uppercase()).ok_or(())?.into_iter().for_each(|(champion_id, _)| {
            roles
                .entry(champion_id.clone())
                .and_modify(|roles| roles.roles.push(role_name))
                .or_insert(champion::Roles { roles: vec![role_name] });
        });
        Ok::<(), ()>(())
    });

    use sea_orm::*;
    let db = use_context::<crate::AppState>().ok_or_else(|| ServerFnError::new("Database connection missing."))?.db;

    let champion_models: Vec<champion::ActiveModel> = champions.data.into_iter().filter_map(|(_, champion)| {
        Some(champion::ActiveModel {
            name: Set(champion.name.clone()),
            id: Set(champion.key.parse().ok()?),
            roles: Set(roles.get(&champion.key)?.clone()),
            ..Default::default()
        })
    }).collect();

    let _ = champion::Entity::insert_many(champion_models.clone())
        .on_conflict(
            sea_query::OnConflict::column(champion::Column::Id)
                .update_column(champion::Column::Roles)
                .to_owned()
        )
        .exec(&db)
        .await
        .map_err(|db_err| ServerFnError::new(db_err.to_string()))?;

    let champion_models = champion_models
        .into_iter()
        .map(|champion_active_model| {
            champion_active_model.try_into_model()
        })
        .collect::<Result<Vec<champion::Model>, DbErr>>()
        .map_err(|db_err| ServerFnError::new(db_err.to_string()))?;
    
    Ok(champion_models)
}