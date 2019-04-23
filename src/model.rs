use serde::Deserialize;
use serde_repr::*;

#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct RawCharacter {
    pub avatar: String,
    #[serde(alias = "ID")]
    pub id: usize,
    pub name: String,
    pub server: String
}

#[derive(Deserialize_repr, PartialEq, Debug)]
#[repr(usize)]
pub enum Gender {
    Other = 0,
    Male = 1,
    Female = 2,
}

#[derive(Deserialize_repr, PartialEq, Debug)]
#[repr(usize)]
pub enum Town {
    Limsa = 1,
    Gridania = 2,
    Uldah = 3
}

#[derive(Deserialize_repr, PartialEq, Debug)]
#[repr(usize)]
pub enum GuardianDiety {
    Halone = 1,
    Menphina = 2,
    Thaliak = 3,
    Nymeia = 4,
    Llymlaen = 5,
    Oschon = 6,
    Byregot = 7,
    Rhalgr = 8,
    Azeyma = 9,
    Naldthal = 10,
    Nophica = 11,
    Althyk = 12
}

#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Character {
    pub avatar: String,
    #[serde(rename = "ID")]
    pub id: usize,
    pub guardian_diety: GuardianDiety,
    pub gender: Gender,
    pub portrait: String,
    pub race: u8,
    pub server: String,
    #[serde(rename = "FreeCompanyId")]
    pub fc: usize,
    pub title: usize,
    pub town: Town,
    pub nameday: String
}

#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct LodestoneIdResult {
    pub character: Character
}