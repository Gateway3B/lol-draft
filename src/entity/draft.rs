//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
#[cfg_attr(
    feature = "ssr",
    derive(sea_orm::DeriveEntityModel),
    sea_orm(table_name = "draft")
)]
pub struct Model {
    #[cfg_attr(feature = "ssr", sea_orm(primary_key))]
    pub draft_id: String,
    
    pub blue_ban_1: Option<u32>,
    pub blue_ban_2: Option<u32>,
    pub blue_ban_3: Option<u32>,
    pub blue_ban_4: Option<u32>,
    pub blue_ban_5: Option<u32>,

    pub red_ban_1: Option<u32>,
    pub red_ban_2: Option<u32>,
    pub red_ban_3: Option<u32>,
    pub red_ban_4: Option<u32>,
    pub red_ban_5: Option<u32>,
    
    pub blue_pick_1: Option<u32>,
    pub blue_pick_2: Option<u32>,
    pub blue_pick_3: Option<u32>,
    pub blue_pick_4: Option<u32>,
    pub blue_pick_5: Option<u32>,

    pub red_pick_1: Option<u32>,
    pub red_pick_2: Option<u32>,
    pub red_pick_3: Option<u32>,
    pub red_pick_4: Option<u32>,
    pub red_pick_5: Option<u32>,

    pub date_completed: chrono::NaiveDateTime,
}

use cfg_if::cfg_if;

cfg_if! { if #[cfg(feature = "ssr")] {
    use sea_orm::entity::prelude::*;

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}
    
    impl ActiveModelBehavior for ActiveModel {}
}}
