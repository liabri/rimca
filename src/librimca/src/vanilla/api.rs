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

pub fn versions(snapshots: bool) -> Result<Vec<Version>, ApiError> {
	let resp = reqwest::blocking::get(VERSION_MANIFEST_URL)?;
	let versions: Vec<Version> = serde_json::from_slice(&resp)?;
	Ok(versions
		.into_iter()
		.filter(|v| !(v.r#type.eq("snapshot")==true && !snapshots))
		.collect())
}


// pub fn latest(snapshot: bool) -> Result<Version, ApiError> {
// 	let resp = reqwest::blocking::get(VERSION_MANIFEST_URL)?;
// 	let van: VanillaLatest = serde_json::from_slice(&resp)?;

// 	Ok(versions(snapshot)?
// 		.into_iter()
// 		.filter(|v| v.id.eq(&van.latest.release) || v.id.eq(&van.latest.snapshot))
// 		.next()
// 		.ok_or(ApiError::CannotFindLatestVersion)?)
// }