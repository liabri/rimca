use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    pub id: String,
    pub inherits_from: String,
    pub release_time: String,
    pub time: String,
    pub r#type: String,
    pub main_class: String,
    pub libraries: Vec<Library>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Libraries {
    pub common: Vec<Library>,
    pub client: Vec<Library>,
    pub server: Vec<Library>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Library {
    pub name: String,
    pub url: String
}