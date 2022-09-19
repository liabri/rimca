mod state;
use state::State;

mod download;
pub use download::DownloadSequence;

mod launch;
pub use launch::LaunchSequence;

pub mod vanilla;
pub use vanilla::Vanilla;

pub mod fabric;
pub use fabric::Fabric;

mod error;
pub use error::{ Error, StateError };

mod verify;

mod paths;
use paths::Paths;

use std::path::Path;

pub struct Instance<T> {
    paths: Paths,
    state: State,
    inner: T,
}

impl<T> Instance<T> {
    fn get(state: State, paths: Paths, version: Option<String>) -> Result<Box<dyn InstanceTrait>, Error> {
        let vanilla = Instance::<Vanilla> { 
            paths: paths.clone(), 
            state: state.clone(), 
            inner: Vanilla::new(&paths, version)?
        };

        match state.scenario.as_ref() { 
            "vanilla" => Ok(Box::new(vanilla)),
            "fabric" => Ok(Box::new(Instance::<Fabric> { paths, state, inner: Fabric::from(vanilla) 
            })),
            _ => Err(Error::StateError(StateError::ScenarioDoesNotExist(state.scenario)))
        }
    }   
}

pub trait InstanceTrait: LaunchSequence + DownloadSequence {}
impl<T> InstanceTrait for T where T: LaunchSequence + DownloadSequence {}

pub fn download(instance: &str, version: Option<String>, scenario: Option<String>, base_dir: &Path) -> Result<(), Error> {
    let mut paths = Paths::new();
    let instance_path = base_dir.join("instances").join(instance);
    std::fs::create_dir_all(&instance_path)?;

    paths.0.insert("natives".to_string(), instance_path.join("natives")); 
    paths.0.insert("instance".to_string(), instance_path);
    paths.0.insert("meta".to_string(), base_dir.join("meta")); 
    paths.0.insert("assets".to_string(), base_dir.join("assets")); 
    paths.0.insert("libraries".to_string(), base_dir.join("libraries")); 

    let scenario = scenario.unwrap_or_else(|| "vanilla".to_string());
    let state = State::from_scenario(scenario);

    Instance::<Box<dyn InstanceTrait>>::get(state, paths, version)?.download()?;

    Ok(())
}

pub fn launch(instance: &str, username: &str, base_dir: &Path) -> Result<(), Error> {
    let mut paths = Paths::new();
    let instance_path = base_dir.join("instances").join(instance);

    paths.0.insert("resources".to_string(), instance_path.join("resources")); 
    paths.0.insert("natives".to_string(), instance_path.join("natives"));
    paths.0.insert("instance".to_string(), instance_path);
    paths.0.insert("meta".to_string(), base_dir.join("meta")); 
    paths.0.insert("assets".to_string(), base_dir.join("assets"));
    paths.0.insert("libraries".to_string(), base_dir.join("libraries")); 

    let state = State::read(paths.get("instance")?)?;  

    Instance::<Box<dyn InstanceTrait>>::get(state, paths, None)?.launch(username)?;

    Ok(())
}

pub fn list_instances(base_dir: &Path) -> std::io::Result<Vec<String>> {
    let instance_path = base_dir.join("instances");
    Ok(std::fs::read_dir(instance_path)?
        .map(|x| x.unwrap().file_name().into_string().unwrap())
        .collect::<Vec<String>>()
    )
}