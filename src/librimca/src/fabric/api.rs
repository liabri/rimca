use crate::error::ApiError;

const BASE_URL: &str = "https://meta.fabricmc.net"; //concatp!()
pub(crate) const MANIFEST: &str = "https://meta.fabricmc.net/v2/versions/loader";
pub(crate) const META: &str = "https://meta.fabricmc.net/v2/versions/loader/{game_version}/{loader_version}/profile/json";

pub fn best_version(ver_id: &str) -> Result<String, ApiError> {
    let url = format!("{}/{}", MANIFEST, ver_id);
    let fjson = reqwest::blocking::get(&url)?.json::<serde_json::Value>().unwrap();

    let loader = fjson
        .as_array().ok_or_else(|| ApiError::LoaderDoesNotExistForGameVer(ver_id.to_string()))?
        .first().ok_or_else(|| ApiError::LoaderDoesNotExistForGameVer(ver_id.to_string()))?;
        
    Ok(loader["loader"]["version"].as_str().unwrap().to_string())
}