use serde::{ Serialize, Deserialize, Deserializer };
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Assets {
    pub objects: HashMap<String, Hash>
}

#[derive(Serialize, Deserialize)]
pub struct Hash {
	pub hash: String,
}

//----------------

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase", rename = "Version")]
pub struct Meta {
	#[serde(deserialize_with = "arguments_deserialiser", alias = "minecraftArguments")]
    pub arguments: HashMap<String, Vec<String>>,	
    pub asset_index: File,
    pub assets: String,
    pub downloads: Downloads,
    pub id: String,
    pub libraries: Vec<Library>,
    pub main_class: String,
    pub r#type: String
}

#[derive(Serialize, Deserialize)]
pub struct Downloads {
	pub client: Download,
	pub server: Option<Download>,
}

#[derive(Serialize, Deserialize)]
pub struct Download {
	pub url: String,
	pub sha1: String
}

#[derive(Serialize, Deserialize)]
pub struct File {
	pub id: String,
	pub url: String
}

//-----------------

#[derive(Serialize, Deserialize)]
pub struct Library {
	pub name: String,
	pub downloads: LibraryDownload,
	pub natives: Option<Natives>,
	pub rules: Option<Vec<Rule>>, 
}

#[derive(Serialize, Deserialize)]
pub struct LibraryDownload {
	pub artifact: Option<Artifact>,
	pub classifiers: Option<HashMap<String, Artifact>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Artifact {
	pub url: String,
	pub path: String,
	pub sha1: String
}

#[derive(Serialize, Deserialize)]
pub struct Natives {
	pub linux: Option<String>,
	// pub macos: Option<Artifact>,
	// pub windows: Artifact
}

#[derive(Serialize, Deserialize)]
pub struct Rule {
	pub action: String,
	pub os: Option<Os>,
}

#[derive(Serialize, Deserialize)]
pub struct Os {
	pub name: Option<String>,
	// pub arch: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Native {
	pub natives_linux: Option<Artifact>,
}

pub fn arguments_deserialiser<'de, D: Deserializer<'de>>(d: D) -> Result<HashMap<String, Vec<String>>, D::Error> {
    let value: serde_json::Value = serde::Deserialize::deserialize(d)?;

    if let Some(value) = value.as_str() {
		let mut map = HashMap::with_capacity(value.len());
		let arguments: Vec<String> = value.split_whitespace().map(|s| s.to_string()).collect();

		map.insert("game".to_string(), arguments);
    	return Ok(map)

    } else if let Some(value) = value.as_object() {
    	let map = value.into_iter()
		     .map(|(key, vec)| (
		     	key.to_string(), 
		     	vec
		     		.as_array()
		     		.unwrap()
		     		.into_iter()
		        	.filter_map(|value| {
		        		value.as_str().map(|x| x.to_string())
	                }).collect()
		        )
		     ).collect();
		
		return Ok(map)
	}

      panic!()
}