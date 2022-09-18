pub mod api;

mod models;
use models::Meta;

use crate::Instance;
use crate::download::DownloadSequence;
use crate::vanilla::Vanilla;
use crate::vanilla;
use crate::launch::LaunchSequence;
use crate::error::{ LaunchError, LaunchArguments, DownloadError, StateError };
use crate::state::Component;
use crate::verify::is_file_valid;

use std::process::Command;
use std::io::BufReader;
use nizziel::{ Download, Downloads };

pub struct Fabric {
    pub version: Option<String>,
    pub vanilla: Instance<Vanilla>
}

impl From<Instance<Vanilla>> for Fabric {
    fn from(vanilla: Instance<Vanilla>) -> Self {
        Self {
            version: None,
            vanilla
        }
    }
}

impl DownloadSequence for Instance<Fabric> {
    fn collect_urls(&mut self) -> Result<Downloads, DownloadError> {
        let mut dls = self.inner.vanilla.collect_urls()?;

        let loader_version = api::best_version(self.inner.vanilla.inner.version.as_ref().unwrap())?;

        let meta_str = nizziel::blocking::download(
            &api::META
                .replace("{game_version}", self.inner.vanilla.inner.version.as_ref().unwrap())
                .replace("{loader_version}", &loader_version), 
            &self.paths.get("meta")?.join("net.fabricmc").join(&format!("{}.json", &loader_version)), false)?;
        let meta: Meta = serde_json::from_slice(&meta_str).unwrap();

        for lib in meta.libraries {
            let split = lib.name.split(":").collect::<Vec<&str>>();
            let local_path = format!("{}/{}/{}/{}-{}.jar", 
                split[0].to_string().replace(".", "/"), split[1], split[2], split[1], split[2]
            );

            let path = self.paths.get("libraries")?.join(&local_path);
            if !path.exists() {
                dls.downloads.push(Download {
                    url: format!("{}{}", lib.url, local_path),
                    path: path,
                    unzip: false
                });
            }
        }

        self.inner.version = Some(loader_version);
        self.create_state(String::new())?;

        return Ok(dls)
    }

    fn create_state(&mut self, _: String) -> Result<(), DownloadError> {
        self.state = self.inner.vanilla.state.clone();

        let game = Component::GameComponent { 
            asset_index: None, 
            version: self.inner.version.as_ref().ok_or(DownloadError::VersionNotSpecified)?.to_string()
        };

        self.state.components.insert("net.fabricmc".to_string(), game);
        self.state.write(&self.paths.get("instance")?)?;
        Ok(())
    }
}

impl LaunchSequence for Instance<Fabric> {
    fn get_main_class(&self, meta: &vanilla::Meta) -> Result<String, LaunchError> {
        let version = {
            match self.state.get_component("net.fabricmc")? {
                Component::GameComponent { version, .. } => version,
                _ => return Err(LaunchError::StateError(StateError::FieldNotFound("version".to_string(), "net.minecraft".to_string())))
            }
        };

        let path = self.paths.get("meta")?.join("net.fabricmc").join(format!("{}.json", version));
        let file = std::fs::File::open(&path)?;
        let reader = BufReader::new(file);
        let fabric_meta: Meta = serde_json::from_reader(reader)?; 

        Ok(fabric_meta.main_class)
    }

    fn get_meta(&self) -> Result<vanilla::Meta, LaunchError> {
        Ok(self.inner.vanilla.get_meta()?)
    }
    
    fn get_game_options(&self, username: &str, meta: &vanilla::Meta) -> Result<Vec<String>, LaunchError> { 
        Ok(self.inner.vanilla.get_game_options(username, meta)?)
    }

    fn get_classpath(&self, meta: &vanilla::Meta) -> Result<String, LaunchError> { 
        let mut classpath = self.inner.vanilla.get_classpath(meta)?;

        let version = {
            match self.state.get_component("net.fabricmc")? {
                Component::GameComponent { version, .. } => version,
                _ => return Err(LaunchError::StateError(StateError::FieldNotFound("version".to_string(), "net.minecraft".to_string())))
            }
        };

        let path = self.paths.get("meta")?.join("net.fabricmc").join(format!("{}.json", version));
        let file = std::fs::File::open(&path)?;
        let reader = BufReader::new(file);
        let fabric_meta: Meta = serde_json::from_reader(reader)?; 

        let dir_name = self.paths.get("libraries")?;
        classpath.push(':');
        for lib in &fabric_meta.libraries {
            let split = lib.name.split(":").collect::<Vec<&str>>();
            let path = format!("/{}/{}/{}/{}-{}.jar", 
                split[0].to_string().replace(".", "/"), split[1], split[2], split[1], split[2]
            );

            println!("dir name: {}", &path);
            classpath.push_str(dir_name.to_str().unwrap());
            classpath.push_str(&path);
            classpath.push(':');
        }

        println!("REAL CLASSPATH!!!: {}", classpath);

        classpath.pop();

        Ok(classpath)
    }
    
    fn get_jvm_arguments(&self, classpath: &str, meta: &vanilla::Meta) -> Result<Vec<String>, LaunchError> { 
        Ok(self.inner.vanilla.get_jvm_arguments(classpath, meta)?)
    }

    fn execute(&self, jvm_args: Vec<String>, main_class: &str, game_opts: Vec<String>) -> Result<(), LaunchError> { 
        Ok(self.inner.vanilla.execute(jvm_args, main_class, game_opts)?)
    }
}