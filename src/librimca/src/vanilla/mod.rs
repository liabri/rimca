mod api;

mod models;
use models::{ Meta, Library, Assets };

use crate::Instance;
use crate::state::State;
use crate::download::DownloadSequence;
use crate::launch::LaunchSequence;
use crate::error::{ Error, LaunchError, DownloadError };
use crate::state::Component;
use crate::verify::is_file_valid;

use std::collections::HashMap;
use std::path::PathBuf;
use nizziel::{ Download, Downloads };

pub struct Vanilla {
	pub version: Option<String>,
	// meta: Meta,
}

impl Vanilla {
	pub fn new() -> Self {
		Self {
			version: None
		}
	}

	pub fn from_version(version: Option<&str>) -> Self {
		if let Some(version) = version {
			return Self {
				version: Some(version.to_string())
			}
		} else {
			return Self {
				version: Some(String::from("get latest_version"))
			}
		}
	}
}

impl DownloadSequence for Instance<Vanilla> {
	fn download(&self) -> Result<(), DownloadError> {
		todo!()
	}

	fn collect_urls(&self) -> Result<Downloads, DownloadError> {
		let mut dls = Downloads::default();

		let version = match &self.inner.version {
			Some(ver) => {
				api::versions(true)?
					.into_iter()
					.filter(|v| v.id.eq(ver))
					.next()
					.ok_or(DownloadError::GameVersionNotFound(ver.to_string()))?
			},

			None => api::latest(false)?
		};

		// impl default for 	
		dls.retries = 5;

		//check if exists locally before making 
		let meta_str = nizziel::blocking::download(&version.url, &self.paths.get("meta").ok_or(DownloadError::Temp)?.join("net.minecraft").join(format!("{}.json", &version.id)), false)?;
		let meta: Meta = serde_json::from_slice(&meta_str)?;

		//version_jar
		let path = self.paths.get("libraries").ok_or(DownloadError::Temp)?.join("com").join("mojang").join("minecraft").join(&version.id).join(format!("minecraft-{}-client.jar", &version.id));
		// let path = LIBRARIES_DIR.join("com").join("mojang").join("minecraft").join(&version.id).join(format!("minecraft-{}-client.jar", &version.id));
		if !path.exists() || !is_file_valid(&path, &meta.downloads.client.sha1)? {
			dls.downloads.push(Download {
				url: meta.downloads.client.url,
				path: path,
				unzip: false
			});
		}

		// let natives_dir = paths::natives(&self.instance());
		let natives_dir = self.paths.get("natives").ok_or(DownloadError::Temp)?;
		for lib in meta.libraries {
			if let Some(artifact) = lib.downloads.artifact {
				let path = self.paths.get("libraries").ok_or(DownloadError::Temp)?.join(artifact.path);
				// let path = LIBRARIES_DIR.join(artifact.path);
				if !path.exists() || !is_file_valid(&path, &artifact.sha1)? {
					dls.downloads.push(Download{
						url: artifact.url,
						path: path,
						unzip: false
					});
				}
			}

			//verify this
			// if !self.verify {
				if let Some(key) = lib.natives.and_then(|n| n.linux) {
					if let Some(url) = &lib.downloads.classifiers.ok_or(DownloadError::LibraryNoClassifiers(lib.name))?.get(&key) {
						dls.downloads.push(Download {
							url: url.url.to_string(),
							path: natives_dir.clone(),
							unzip: true
						});	
					}			
				}
			// }
		}

		let asset_id = meta.asset_index.id;
		let url = meta.asset_index.url;
		let path = self.paths.get("assets").ok_or(DownloadError::Temp)?.join("indexes").join(format!("{}.json", asset_id));
		// let path = ASSETS_DIR.join("indexes").join(format!("{}.json", asset_id));

		let ajson_resp = nizziel::blocking::download(&url, &path, false)?;
		let assets: Assets = serde_json::from_slice(&ajson_resp)?;

		if asset_id.eq("pre-1.6") || asset_id.eq("legacy") {
			for (key, hash) in assets.objects {
				let hash_head = &hash.hash[0..2];
				let path = self.paths.get("instance").ok_or(DownloadError::Temp)?.join("resources").join(key);
				// let path = paths::instance(&self.instance()).join("resources").join(key);

				if !path.exists() && is_file_valid(&path, &hash.hash)? {
					dls.downloads.push(Download{
						url: format!("https://resources.download.minecraft.net/{}/{}", hash_head, hash.hash),
						path: path,
						unzip: false
					});
				}
			}
		} else {
			let objects_dir = self.paths.get("assets").ok_or(DownloadError::Temp)?.join("objects");
			// let objects_dir = ASSETS_DIR.join("objects");
			for hash in assets.objects.values() {
				let hash_head = &hash.hash[0..2];
				let path = objects_dir.join(&hash_head).join(&hash.hash);

				if !path.exists() { //&& is_file_valid(&path, &hash.hash) {
					dls.downloads.push(Download{
						url: format!("https://resources.download.minecraft.net/{}/{}", hash_head, hash.hash),
						path: path,
						unzip: false
					});
				}
			}
		}

		let mut state = State {
			scenario: "vanilla".to_string(),
			components: HashMap::new(),
			wrapper: None,
			prelaunch_cmds: None,
		};

		state.components.insert("java".to_string(), Component::JavaComponent { path: "java".to_string(), arguments: None });
		state.components.insert("net.minecraft".to_string(), Component::GameComponent { asset_index: Some(asset_id), version: version.id });
		state.write(&self.paths.get("instance").ok_or(DownloadError::Temp)?)?;


		return Ok(dls)
	}

	fn spawn_thread(&self) -> Result<(), DownloadError> {
		Ok(())
	}
}

impl LaunchSequence for Instance<Vanilla> {
	fn get_main_class(&self) -> Result<&str, LaunchError> { todo!() }
	fn execute(&self, jvm_args: Vec<String>, main_class: &str, game_opts: Vec<String>) -> Result<(), LaunchError> { todo!() }
	fn get_jvm_arguments(&self, classpath: &str) -> Result<Vec<String>, LaunchError> { todo!() }
	fn get_game_options(&self) -> Result<Vec<String>, LaunchError> { todo!() }
	fn get_classpath(&self) -> Result<String, LaunchError> { todo!() }
}