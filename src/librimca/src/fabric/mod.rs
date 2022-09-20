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
use crate::Paths;

use std::process::Command;
use std::io::BufReader;
use nizziel::{ Download, Downloads };

pub struct Fabric {
    pub version: String,
    pub vanilla: Instance<Vanilla>,
    pub meta: Meta
}

impl Fabric {
    pub fn new(paths: &Paths, version: Option<String>, vanilla: Instance<Vanilla>) -> Result<Self, DownloadError> {
        let version = api::best_version(&vanilla.inner.version.id)?;

        let meta = {
            let path = paths.get("meta")?.join("net.fabricmc").join(format!("{}.json", &vanilla.inner.version.id));
            let file = std::fs::File::open(&path)?;
            let reader = BufReader::new(file);
            if let Ok(meta) = serde_json::from_reader(reader) {
                meta 
            } else {
                todo!();
                // let meta_str = nizziel::blocking::download(&version.url, &path, false)?;
                // serde_json::from_slice::<Meta>(&meta_str)?
            }
        };

        Ok(Self {
            version,
            vanilla,
            meta
        })
    }
}

impl DownloadSequence for Instance<Fabric> {
    fn collect_urls(&mut self) -> Result<Downloads, DownloadError> {
        let mut dls = self.inner.vanilla.collect_urls()?;

        let loader_version = api::best_version(&self.inner.vanilla.inner.version.id)?;

        let meta_str = nizziel::blocking::download(
            &api::META
                .replace("{game_version}", &self.inner.vanilla.inner.version.id)
                .replace("{loader_version}", &self.inner.version), 
            &self.paths.get("meta")?.join("net.fabricmc").join(&format!("{}.json", &self.inner.version)), false)?;
        let meta: Meta = serde_json::from_slice(&meta_str).unwrap();

        for lib in meta.libraries {
            let split = lib.name.split(':').collect::<Vec<&str>>();
            let local_path = format!("{}/{}/{}/{}-{}.jar", 
                split[0].to_string().replace('.', "/"), split[1], split[2], split[1], split[2]
            );

            let path = self.paths.get("libraries")?.join(&local_path);
            if !path.exists() {
                dls.downloads.push(Download {
                    url: format!("{}{}", lib.url, local_path),
                    path,
                    unzip: false
                });
            }
        }

        self.create_state(String::new())?;

        Ok(dls)
    }

    fn create_state(&mut self, _: String) -> Result<(), DownloadError> {
        self.state = self.inner.vanilla.state.clone();

        let game = Component::GameComponent { 
            asset_index: None, 
            version: self.inner.version.clone()
        };

        self.state.components.insert("net.fabricmc".to_string(), game);
        self.state.write(self.paths.get("instance")?)?;
        Ok(())
    }
}

impl LaunchSequence for Instance<Fabric> {
    fn get_main_class(&self) -> Result<String, LaunchError> {
        Ok(self.inner.meta.main_class.clone())
    }
    
    fn get_game_options(&self, username: &str) -> Result<Vec<String>, LaunchError> { 
        self.inner.vanilla.get_game_options(username)
    }

    fn get_classpath(&self) -> Result<String, LaunchError> { 
        let mut classpath = self.inner.vanilla.get_classpath()?;

        let dir_name = self.paths.get("libraries")?;
        classpath.push(':');
        for lib in &self.inner.meta.libraries {
            let split = lib.name.split(':').collect::<Vec<&str>>();
            let path = format!("/{}/{}/{}/{}-{}.jar", 
                split[0].to_string().replace('.', "/"), split[1], split[2], split[1], split[2]
            );

            classpath.push_str(dir_name.to_str().unwrap());
            classpath.push_str(&path);
            classpath.push(':');
        }

        classpath.pop();

        Ok(classpath)
    }
    
    fn get_jvm_arguments(&self, classpath: &str) -> Result<Vec<String>, LaunchError> { 
        self.inner.vanilla.get_jvm_arguments(classpath)
    }
}