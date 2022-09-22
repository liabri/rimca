pub mod api;

mod models;
use models::Meta;

use crate::Instance;
use crate::download::DownloadSequence;
use crate::vanilla::Vanilla;
use crate::launch::LaunchSequence;
use crate::error::{ LaunchError, DownloadError };
use crate::state::Component;
use crate::Paths;

use std::io::BufReader;
use nizziel::{ Download, Downloads };

pub struct Fabric {
    pub version: String,
    pub meta: Meta,
    pub vanilla: Instance<Vanilla>
}

impl Fabric {
    pub fn new(paths: &Paths, vanilla: Instance<Vanilla>) -> Result<Self, DownloadError> {
        let version = api::best_version(&vanilla.inner.version.id)?;

        let meta = {
            let path = paths.get("meta")?.join("net.fabricmc").join(format!("{}.json", &vanilla.inner.version.id));
            if let Ok(file) = std::fs::File::open(&path) {
                let reader = BufReader::new(file); 
                serde_json::from_reader(reader)?               
            } else {
                let meta_str = nizziel::blocking::download(
                    &api::META
                        .replace("{game_version}", &vanilla.inner.version.id)
                        .replace("{loader_version}", &version), 
                    &paths.get("meta")?.join("net.fabricmc").join(&format!("{}.json", &version)), false)?;
                serde_json::from_slice(&meta_str)?
            }
        };

        Ok(Self {
            version,
            meta,
            vanilla
        })
    }
}

impl DownloadSequence for Instance<Fabric> {
    fn collect_urls(&mut self) -> Result<Downloads, DownloadError> {
        let mut dls = self.inner.vanilla.collect_urls()?;

        for lib in &self.inner.meta.libraries {
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

        self.write_state()?;

        Ok(dls)
    }

    fn write_state(&mut self) -> Result<(), DownloadError> {
        self.state = self.inner.vanilla.state.clone();
        self.state.components.insert(
            "net.fabricmc".to_string(), 
            Component::GameComponent { 
                version: self.inner.version.clone()
            }
        );
        
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