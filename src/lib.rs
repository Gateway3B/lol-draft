use serde::{Deserialize, Serialize};
use strum_macros::EnumIs;
use uuid::Uuid;

pub mod draft;
pub mod app;
pub mod entity;
pub mod api;

#[cfg(feature = "ssr")]
#[derive(Clone)]
pub struct AppState {
    pub db: sea_orm::DatabaseConnection,
    pub server_signals: leptos_ws::server_signals::ServerSignals,
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, EnumIs)]
pub enum Turn {
    #[default]
    PreDraft,
    BlueBan1,
    BlueBan2,
    BlueBan3,
    BlueBan4,
    BlueBan5,
    RedBan1,
    RedBan2,
    RedBan3,
    RedBan4,
    RedBan5,
    BluePick1,
    BluePick2,
    BluePick3,
    BluePick4,
    BluePick5,
    RedPick1,
    RedPick2,
    RedPick3,
    RedPick4,
    RedPick5,
    PostDraft,
}

impl Turn {
    fn is_ban(&self) -> bool {
        self.is_blue_ban_1() ||
        self.is_blue_ban_2() ||
        self.is_blue_ban_3() ||
        self.is_blue_ban_4() ||
        self.is_blue_ban_5() ||
        self.is_red_ban_1() ||
        self.is_red_ban_2() ||
        self.is_red_ban_3() ||
        self.is_red_ban_4() ||
        self.is_red_ban_5()
    }

    fn is_blue(&self) -> bool {
        self.is_blue_ban_1() ||
        self.is_blue_ban_2() ||
        self.is_blue_ban_3() ||
        self.is_blue_ban_4() ||
        self.is_blue_ban_5() ||
        self.is_blue_pick_1() ||
        self.is_blue_pick_2() ||
        self.is_blue_pick_3() ||
        self.is_blue_pick_4() ||
        self.is_blue_pick_5()
    }

    fn is_red(&self) -> bool {
        self.is_red_ban_1() ||
        self.is_red_ban_2() ||
        self.is_red_ban_3() ||
        self.is_red_ban_4() ||
        self.is_red_ban_5() ||
        self.is_red_pick_1() ||
        self.is_red_pick_2() ||
        self.is_red_pick_3() ||
        self.is_red_pick_4() ||
        self.is_red_pick_5()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Draft {
    draft_id: Uuid,
    blue_id: Uuid,
    red_id: Uuid,

    blue_ready: bool,
    red_ready: bool,
    
    blue_ban_1: Option<u32>,
    blue_ban_2: Option<u32>,
    blue_ban_3: Option<u32>,
    blue_ban_4: Option<u32>,
    blue_ban_5: Option<u32>,

    red_ban_1: Option<u32>,
    red_ban_2: Option<u32>,
    red_ban_3: Option<u32>,
    red_ban_4: Option<u32>,
    red_ban_5: Option<u32>,
    
    blue_pick_1: Option<u32>,
    blue_pick_2: Option<u32>,
    blue_pick_3: Option<u32>,
    blue_pick_4: Option<u32>,
    blue_pick_5: Option<u32>,

    red_pick_1: Option<u32>,
    red_pick_2: Option<u32>,
    red_pick_3: Option<u32>,
    red_pick_4: Option<u32>,
    red_pick_5: Option<u32>,

    turn: Turn,
}

impl Draft {
    fn is_champ_chosen(&self, id: u32) -> bool {
        self.blue_ban_1.is_some_and(|selection_id| selection_id == id) && !self.turn.is_blue_ban_1() ||
        self.blue_ban_2.is_some_and(|selection_id| selection_id == id) && !self.turn.is_blue_ban_2() ||
        self.blue_ban_3.is_some_and(|selection_id| selection_id == id) && !self.turn.is_blue_ban_3() ||
        self.blue_ban_4.is_some_and(|selection_id| selection_id == id) && !self.turn.is_blue_ban_4() ||
        self.blue_ban_5.is_some_and(|selection_id| selection_id == id) && !self.turn.is_blue_ban_5() ||
        self.blue_pick_1.is_some_and(|selection_id| selection_id == id) && !self.turn.is_blue_pick_1() ||
        self.blue_pick_2.is_some_and(|selection_id| selection_id == id) && !self.turn.is_blue_pick_2() ||
        self.blue_pick_3.is_some_and(|selection_id| selection_id == id) && !self.turn.is_blue_pick_3() ||
        self.blue_pick_4.is_some_and(|selection_id| selection_id == id) && !self.turn.is_blue_pick_4() ||
        self.blue_pick_5.is_some_and(|selection_id| selection_id == id) && !self.turn.is_blue_pick_5() ||
        self.red_ban_1.is_some_and(|selection_id| selection_id == id) && !self.turn.is_red_ban_1() ||
        self.red_ban_2.is_some_and(|selection_id| selection_id == id) && !self.turn.is_red_ban_2() ||
        self.red_ban_3.is_some_and(|selection_id| selection_id == id) && !self.turn.is_red_ban_3() ||
        self.red_ban_4.is_some_and(|selection_id| selection_id == id) && !self.turn.is_red_ban_4() ||
        self.red_ban_5.is_some_and(|selection_id| selection_id == id) && !self.turn.is_red_ban_5() ||
        self.red_pick_1.is_some_and(|selection_id| selection_id == id) && !self.turn.is_red_pick_1() ||
        self.red_pick_2.is_some_and(|selection_id| selection_id == id) && !self.turn.is_red_pick_2() ||
        self.red_pick_3.is_some_and(|selection_id| selection_id == id) && !self.turn.is_red_pick_3() ||
        self.red_pick_4.is_some_and(|selection_id| selection_id == id) && !self.turn.is_red_pick_4() ||
        self.red_pick_5.is_some_and(|selection_id| selection_id == id) && !self.turn.is_red_pick_5()
    }

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

    fn current_pick(&self) -> Option<u32> {
        self.get_pick(&self.turn)
    }

    #[allow(dead_code)]
    fn select_pick(&mut self, pick: u32) {
        let pick = Some(pick);
        match self.turn {
            Turn::BlueBan1 => self.blue_ban_1 = pick,
            Turn::BlueBan2 => self.blue_ban_2 = pick,
            Turn::BlueBan3 => self.blue_ban_3 = pick,
            Turn::BlueBan4 => self.blue_ban_4 = pick,
            Turn::BlueBan5 => self.blue_ban_5 = pick,
            
            Turn::RedBan1 => self.red_ban_1 = pick,
            Turn::RedBan2 => self.red_ban_2 = pick,
            Turn::RedBan3 => self.red_ban_3 = pick,
            Turn::RedBan4 => self.red_ban_4 = pick,
            Turn::RedBan5 => self.red_ban_5 = pick,

            Turn::BluePick1 => self.blue_pick_1 = pick,
            Turn::BluePick2 => self.blue_pick_2 = pick,
            Turn::BluePick3 => self.blue_pick_3 = pick,
            Turn::BluePick4 => self.blue_pick_4 = pick,
            Turn::BluePick5 => self.blue_pick_5 = pick,

            Turn::RedPick1 => self.red_pick_1 = pick,
            Turn::RedPick2 => self.red_pick_2 = pick,
            Turn::RedPick3 => self.red_pick_3 = pick,
            Turn::RedPick4 => self.red_pick_4 = pick,
            Turn::RedPick5 => self.red_pick_5 = pick,
            _ => ()
        }
    } 

    #[allow(dead_code)]
    fn next_turn(&mut self) {
        self.turn = match self.turn {
            Turn::PreDraft => Turn::BlueBan1,
            Turn::BlueBan1 => Turn::RedBan1,
            Turn::RedBan1 => Turn::BlueBan2,
            Turn::BlueBan2 => Turn::RedBan2,
            Turn::RedBan2 => Turn::BlueBan3,
            Turn::BlueBan3 => Turn::RedBan3,
            Turn::RedBan3 => Turn::BluePick1,
            
            Turn::BluePick1 => Turn::RedPick1,
            Turn::RedPick1 => Turn::RedPick2,
            Turn::RedPick2 => Turn::BluePick2,
            Turn::BluePick2 => Turn::BluePick3,
            Turn::BluePick3 => Turn::RedPick3,
            Turn::RedPick3 => Turn::RedBan4,

            Turn::RedBan4 => Turn::BlueBan4,
            Turn::BlueBan4 => Turn::RedBan5,
            Turn::RedBan5 => Turn::BlueBan5,
            Turn::BlueBan5 => Turn::RedPick4,

            Turn::RedPick4 => Turn::BluePick4,
            Turn::BluePick4 => Turn::BluePick5,
            Turn::BluePick5 => Turn::RedPick5,
            Turn::RedPick5 => Turn::PostDraft,
            Turn::PostDraft => Turn::PostDraft
        };
    }

}