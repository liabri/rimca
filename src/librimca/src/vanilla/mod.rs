pub mod api;

pub mod models;
pub use models::{ Meta, Assets };

use crate::Instance;
use crate::download::DownloadSequence;
use crate::launch::LaunchSequence;
use crate::error::{ LaunchError, LaunchArguments, DownloadError, StateError };
use crate::state::Component;
use crate::verify::is_file_valid;

use std::process::Command;
use std::io::BufReader;
use nizziel::{ Download, Downloads };

pub struct Vanilla {
    pub version: Option<String>,
}

impl From<Option<String>> for Vanilla {
    fn from(version: Option<String>) -> Self {
        Self {
            version
        }
    }
}

impl DownloadSequence for Instance<Vanilla> {
    fn collect_urls(&mut self) -> Result<Downloads, DownloadError> {
        let version = match &self.inner.version {
            Some(ver) => {
                api::versions(true)?.into_iter().find(|v| v.id.eq(ver))
                    .ok_or_else(|| DownloadError::GameVersionNotFound(ver.to_string()))?
            },

            None => api::latest(false)?
        };

        let mut dls = Downloads { retries: 5, ..Default::default() };

        // get meta file, used to get libraries, natives & assets -- in that order
        let meta_str = nizziel::blocking::download(&version.url, &self.paths.get("meta")?.join("net.minecraft").join(format!("{}.json", &version.id)), false)?;
        let meta: Meta = serde_json::from_slice(&meta_str)?;

        // version.jar
        let path = self.paths.get("libraries")?.join("com").join("mojang").join("minecraft").join(&version.id).join(format!("minecraft-{}-client.jar", &version.id));
        if !path.exists() || !is_file_valid(&path, &meta.downloads.client.sha1)? {
            dls.downloads.push(Download {
                url: meta.downloads.client.url,
                path,
                unzip: false
            });
        }

        let natives_dir = self.paths.get("natives")?;
        for lib in meta.libraries {
            // libraries
            if let Some(artifact) = lib.downloads.artifact {
                let path = self.paths.get("libraries")?.join(artifact.path);
                if !path.exists() || !is_file_valid(&path, &artifact.sha1)? {
                    dls.downloads.push(Download{
                        url: artifact.url,
                        path,
                        unzip: false
                    });
                }
            }

            // natives (pre 1.19)
            if let Some(key) = lib.natives.and_then(|n| n.linux) {
                if let Some(url) = &lib.downloads.classifiers.ok_or(DownloadError::LibraryNoClassifiers(lib.name))?.get(&key) {
                    dls.downloads.push(Download {
                        url: url.url.to_string(),
                        path: natives_dir.clone(),
                        unzip: true
                    }); 
                }           
            }
        }

        // assets
        let asset_id = meta.asset_index.id;
        let url = meta.asset_index.url;
        let path = self.paths.get("assets")?.join("indexes").join(format!("{}.json", asset_id));

        let assets_str = nizziel::blocking::download(&url, &path, false)?;
        let assets: Assets = serde_json::from_slice(&assets_str)?;

        if asset_id.eq("pre-1.6") || asset_id.eq("legacy") {
            for (key, hash) in assets.objects {
                let hash_head = &hash.hash[0..2];
                let path = self.paths.get("instance")?.join("resources").join(key);

                if !path.exists() && is_file_valid(&path, &hash.hash)? {
                    dls.downloads.push(Download {
                        url: format!("https://resources.download.minecraft.net/{}/{}", hash_head, hash.hash),
                        path,
                        unzip: false
                    });
                }
            }
        } else {
            let objects_dir = self.paths.get("assets")?.join("objects");
            for hash in assets.objects.values() {
                let hash_head = &hash.hash[0..2];
                let path = objects_dir.join(hash_head).join(&hash.hash);

                if !path.exists() {
                    dls.downloads.push(Download{
                        url: format!("https://resources.download.minecraft.net/{}/{}", hash_head, hash.hash),
                        path,
                        unzip: false
                    });
                }
            }
        }

        self.inner.version = Some(version.id);
        self.create_state(asset_id)?;

        Ok(dls)
    }

    fn create_state(&mut self, asset_id: String) -> Result<(), DownloadError> {
        let java = Component::JavaComponent { 
            path: "java".to_string(), 
            arguments: None 
        };

        let game = Component::GameComponent { 
            asset_index: Some(asset_id), 
            version: self.inner.version.as_ref().ok_or(DownloadError::VersionNotSpecified)?.to_string()
        };

        self.state.components.insert("java".to_string(), java);
        self.state.components.insert("net.minecraft".to_string(), game);
        self.state.write(self.paths.get("instance")?)?;
        Ok(())
    }
}

impl LaunchSequence for Instance<Vanilla> {
    fn get_main_class(&self, meta: &Meta) -> Result<String, LaunchError> {
        Ok(meta.main_class.clone())
    }

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
            let asset_index = asset_index.as_ref().ok_or_else(|| StateError::FieldNotFound("asset_index".to_string(), "net.minecraft".to_string()))?;   
            let game_assets = self.paths.get("resources")?;

            let arguments = meta.arguments.get("game").ok_or(LaunchError::ArgumentsNotFound(LaunchArguments::Game))?;
            // let account = crate::auth::Accounts::get()?.get_account(self.username()).unwrap_or(auth::Account::default());

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
                        .replace("${natives_directory}", natives_directory.to_str().unwrap())
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

        if let Ok(Component::JavaComponent { arguments, .. }) = &self.state.get_component("java") {
            if let Some(args) = arguments {
                jvm_arguments.extend(args.split_whitespace().map(|s| s.to_string()));
            } 

            return Ok(jvm_arguments);
        }

        Err(LaunchError::StateError(StateError::ComponentNotFound(String::from("java"))))
    }

    fn execute(&self, jvm_args: Vec<String>, main_class: &str, game_opts: Vec<String>) -> Result<(), LaunchError> { 
        if let Ok(Component::JavaComponent { path, .. }) = self.state.get_component("java") {
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
            //  log::debug!("No jvm output enabled");
            //  command.stdout(Stdio::null())
            //  .stderr(Stdio::null());
            // }

            println!("Spawning command: {:?}", command);
            command.spawn()?;

            return Ok(())
        }

        Err(LaunchError::StateError(StateError::ComponentNotFound(String::from("java"))))
    }
}