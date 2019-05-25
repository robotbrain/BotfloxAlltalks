use serde::Deserialize;
use serde_repr::*;
use strum_macros::Display;

#[derive(Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct RawCharacter {
    pub avatar: String,
    #[serde(alias = "ID")]
    pub id: usize,
    pub name: String,
    pub server: String,
}

#[derive(Deserialize_repr, PartialEq, Debug, Display)]
#[repr(usize)]
pub enum Gender {
    Other = 0,
    Male = 1,
    Female = 2,
}

#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct WithIdName {
    #[serde(alias = "ID")]
    pub id: usize,
    pub name: String,
}

#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ActiveClassJob {
    pub class: WithIdName,
    pub level: usize,
    pub job: WithIdName,
}

#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Character {
    pub active_class_job: ActiveClassJob,
    pub avatar: String,
    #[serde(rename = "ID")]
    pub id: usize,
    pub guardian_deity: WithIdName,
    pub gender: Gender,
    pub portrait: String,
    pub race: WithIdName,
    pub server: String,
    #[serde(rename = "FreeCompanyId")]
    pub fc_id: Option<String>,
    pub title: WithIdName,
    pub town: WithIdName,
    pub tribe: WithIdName,
    pub nameday: String,
    pub name: String,
    #[serde(skip_deserializing)]
    pub fc: FreeCompany,
}

#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct LodestoneCharacterIdResult {
    pub character: Character,
}

#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Pagination {
    pub results_total: usize,
    pub page: usize,
    pub page_next: Option<usize>,
    pub page_prev: Option<usize>,
    pub page_total: usize,
}

#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct LodestoneSearchResult<T> {
    pub pagination: Pagination,
    pub results: Vec<T>,
}

#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct FreeCompany {
    pub name: String,
    #[serde(rename = "ID")]
    pub id: String,
    pub tag: String,
}

impl Default for FreeCompany {
    fn default() -> Self {
        FreeCompany {
            name: "".to_string(),
            id: "".to_string(),
            tag: "".to_string(),
        }
    }
}

#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct LodestoneFCIdResult {
    pub free_company: FreeCompany,
}

#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct PartyComp {
    pub healers_per_party: usize,
    pub melees_per_party: usize,
    pub ranged_per_party: usize,
    pub tanks_per_party: usize,
}

#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct DutyInfo {
    #[serde(rename = "ID")]
    pub id: usize,
    pub class_job_level_sync: usize,
    pub class_job_level_required: usize,
    pub item_level_sync: usize,
    pub itel_level_required: usize,
    pub content_type: WithIdName,
    pub content_member_type: PartyComp,
    pub name: String,
}

#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct DutyResult {
    pub content_finder_condition: DutyInfo,
}
