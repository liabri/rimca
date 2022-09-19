use serde::{ Serialize, Deserialize };
use crate::error::ApiError;

const VERSION_MANIFEST_URL: &str = "http://launchermeta.mojang.com/mc/game/version_manifest.json";

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Version {
    pub id: String,
    pub r#type: String,
    pub url: String,
    pub time: String,
    pub release_time: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Manifest {
    latest: Latest,
    versions: Vec<Version>
}

pub fn versions(snapshots: bool) -> Result<Vec<Version>, ApiError> {
    let manifest = reqwest::blocking::get(VERSION_MANIFEST_URL)?
        .json::<Manifest>().unwrap();

    Ok(manifest.versions
        .into_iter()
        .filter(|v| !(v.r#type.eq("snapshot")==true && !snapshots))
        .collect())
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VanillaLatest {
    pub latest: Latest,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Latest {
    pub release: String,
    pub snapshot: String
}


pub fn latest(snapshot: bool) -> Result<Version, ApiError> {
    let van = reqwest::blocking::get(VERSION_MANIFEST_URL)?
        .json::<VanillaLatest>()?;

    versions(snapshot)?
        .into_iter()
        .find(|v| v.id.eq(&van.latest.release) || v.id.eq(&van.latest.snapshot))
        .ok_or(ApiError::CannotFindLatestVersion)
}