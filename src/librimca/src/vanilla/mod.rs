mod api;

pub mod models;
use models::{ Meta, Library, Assets };

use crate::Instance;
use crate::state::State;
use crate::download::DownloadSequence;
use crate::launch::LaunchSequence;
use crate::error::{ Error, LaunchError, LaunchArguments, DownloadError, StateError };
use crate::state::Component;
use crate::verify::is_file_valid;

use std::process::Command;
use std::io::BufReader;
use std::collections::HashMap;
use std::path::PathBuf;
use nizziel::{ Download, Downloads };

pub struct Vanilla {
	pub version: Option<String>,
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
		self.spawn_thread(self.collect_urls()?)
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
		let meta_str = nizziel::blocking::download(&version.url, &self.paths.get("meta")?.join("net.minecraft").join(format!("{}.json", &version.id)), false)?;
		let meta: Meta = serde_json::from_slice(&meta_str)?;

		//version_jar
		let path = self.paths.get("libraries")?.join("com").join("mojang").join("minecraft").join(&version.id).join(format!("minecraft-{}-client.jar", &version.id));
		if !path.exists() || !is_file_valid(&path, &meta.downloads.client.sha1)? {
			dls.downloads.push(Download {
				url: meta.downloads.client.url,
				path: path,
				unzip: false
			});
		}

		// let natives_dir = paths::natives(&self.instance());
		let natives_dir = self.paths.get("natives")?;
		for lib in meta.libraries {
			if let Some(artifact) = lib.downloads.artifact {
				let path = self.paths.get("libraries")?.join(artifact.path);
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
		let path = self.paths.get("assets")?.join("indexes").join(format!("{}.json", asset_id));
		// let path = ASSETS_DIR.join("indexes").join(format!("{}.json", asset_id));

		let ajson_resp = nizziel::blocking::download(&url, &path, false)?;
		let assets: Assets = serde_json::from_slice(&ajson_resp)?;

		if asset_id.eq("pre-1.6") || asset_id.eq("legacy") {
			for (key, hash) in assets.objects {
				let hash_head = &hash.hash[0..2];
				let path = self.paths.get("instance")?.join("resources").join(key);
				// let path = paths::instance(&self.instance()).join("resources").join(key);

				if !path.exists() && is_file_valid(&path, &hash.hash)? {
					dls.downloads.push(Download {
						url: format!("https://resources.download.minecraft.net/{}/{}", hash_head, hash.hash),
						path: path,
						unzip: false
					});
				}
			}
		} else {
			let objects_dir = self.paths.get("assets")?.join("objects");
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

		// create state.json
		let mut state = State {
			scenario: "vanilla".to_string(),
			components: HashMap::new(),
			wrapper: None,
			prelaunch_cmds: None,
		};

		state.components.insert("java".to_string(), Component::JavaComponent { path: "java".to_string(), arguments: None });
		state.components.insert("net.minecraft".to_string(), Component::GameComponent { asset_index: Some(asset_id), version: version.id });
		state.write(&self.paths.get("instance")?)?;

		return Ok(dls)
	}
}

impl LaunchSequence for Instance<Vanilla> {
	fn get_meta(&self) -> Result<Meta, LaunchError> {
		let version = {
			match self.state.get_component("net.minecraft")? {
				Component::GameComponent { version, .. } => version,
				_ => return Err(LaunchError::StateError(StateError::FieldNotFound("version".to_string(), "net.minecraft".to_string())))
			}
		};


		let meta_path = self.paths.get("meta")?
			.join("net.minecraft").join(format!("{}.json", &version));
		let meta_file = std::fs::File::open(&meta_path)?;
    	let reader = BufReader::new(meta_file);
		Ok(serde_json::from_reader(reader)?)
 	}
	
	fn get_game_options(&self, username: &str, meta: &Meta) -> Result<Vec<String>, LaunchError> { 
		if let Component::GameComponent { asset_index, version } = self.state.get_component("net.minecraft")? {
			let asset_index = asset_index.as_ref().ok_or(StateError::FieldNotFound("asset_index".to_string(), "net.minecraft".to_string()))?;	
			let game_assets = self.paths.get("resources")?;

			let arguments = meta.arguments.get("game").ok_or(LaunchError::ArgumentsNotFound(LaunchArguments::Game))?;
			// let account = crate::auth::Accounts::get()?.get_account(self.username()).unwrap_or(auth::Account::default());

			// //make nicer
			return Ok(arguments.iter().map(|x| x
					.replace("${auth_player_name}", username)
					.replace("${version_name}", version)
					.replace("${game_directory}", ".")
					.replace("${assets_root}", self.paths.get("assets").expect("assets").to_str().unwrap())
					.replace("${assets_index_name}", asset_index)
					.replace("${auth_uuid}", "null")//&account.uuid)
					.replace("${auth_access_token}", "null")//&account.access_token)
					.replace("${user_type}", "mojang")
					.replace("${version_type}", &meta.r#type)
					.replace("${user_properties}", "{}")
					// .replace("${resolution_width}", "1920")
					// .replace("${resolution_height}", "1080")
					.replace("${game_assets}", game_assets.to_str().unwrap())
					.replace("${auth_session}", "{}")
			).collect());
		}

		Err(LaunchError::StateError(StateError::ComponentNotFound(String::from("net.minecraft"))))
	}

	fn get_classpath(&self, meta: &Meta) -> Result<String, LaunchError> { 
		let libraries = self.paths.get("libraries")?;

		let mut classpath = String::with_capacity((libraries.to_str().unwrap().len() * meta.libraries.len())
		    + (meta.libraries.len() * 2)
		    + meta.libraries.iter().map(|lib| lib.downloads.artifact.as_ref().map_or(0, |a| a.path.len())).sum::<usize>()
		);

		'outer: for lib in &meta.libraries {
			if let Some(rules) = &lib.rules {
				for rule in rules {
					if let Some(os) = &rule.os {
						if let Some(name) = &os.name {
							if rule.action.eq("allow") && name.ne("linux") || 
							rule.action.eq("disallow") && name.eq("linux") {
								continue 'outer
							}
						}
					}
				}
			}

			if let Some(artifact) = &lib.downloads.artifact { 
				classpath.push_str(libraries.to_str().unwrap());
				classpath.push('/');
				classpath.push_str(&artifact.path);
				classpath.push(':');
			}
		}

		let jar_name = format!("minecraft-{}-client.jar", meta.id);
		let jar_path = libraries.join("com").join("mojang").join("minecraft").join(meta.id.clone()).join(jar_name);
		classpath.push_str(jar_path.to_str().unwrap());
		Ok(classpath)
	}
	
	fn get_jvm_arguments(&self, classpath: &str, meta: &Meta) -> Result<Vec<String>, LaunchError> { 
		let natives_directory = self.paths.get("natives")?;

		let mut jvm_arguments = {
			if let Some(arguments) = meta.arguments.get("jvm") {
				arguments.iter().map(|x| x
						.replace("${natives_directory}", &natives_directory.to_str().unwrap())
						.replace("${launcher_name}", "rimca")
						.replace("${launcher_version}", "3.0")
						.replace("${classpath}", classpath)
				).collect()
			} else {
				let mut jvm_arguments = Vec::with_capacity(3 + classpath.len());
				jvm_arguments.push(format!("-Djava.library.path={}", &natives_directory.to_str().unwrap()));
				jvm_arguments.push("-cp".to_string());
				jvm_arguments.push(classpath.to_string());
				jvm_arguments
			} 
		};

		if let Ok(component) = &self.state.get_component("java") {
			if let Component::JavaComponent { arguments, .. } = component {
				if let Some(args) = arguments {
					jvm_arguments.extend(args.split_whitespace().map(|s| s.to_string()));
				} 

				return Ok(jvm_arguments);
			}
		}

		Err(LaunchError::StateError(StateError::ComponentNotFound(String::from("java"))))
	}

	fn execute(&self, jvm_args: Vec<String>, main_class: &str, game_opts: Vec<String>) -> Result<(), LaunchError> { 
		if let Ok(component) = self.state.get_component("java") {
			if let Component::JavaComponent { path, .. } = component {
				let (exe, args) = match &self.state.wrapper {
				    Some(wrapper) => (wrapper.as_str(), &["java"][..]),
				    None => (path.as_str(), &[][..]),
				};

				let mut command = Command::new(exe);
				command.args(args);
				command.current_dir(self.paths.get("instance")?)
					.args(jvm_args)
					.arg(main_class)
					.args(game_opts);

				// if *self.no_output() {
				// 	log::debug!("No jvm output enabled");
				// 	command.stdout(Stdio::null())
				// 	.stderr(Stdio::null());
				// }

				println!("Spawning command: {:?}", command);
				command.spawn()?;

				return Ok(())
			}
		}

		Err(LaunchError::StateError(StateError::ComponentNotFound(String::from("java"))))
	}
}