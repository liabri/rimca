#![feature(error_generic_member_access)]
#![feature(provide_any)]

mod state;
use state::{ State, Component };

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

mod auth;
use auth::Accounts;

mod paths;
use paths::Paths;

use std::path::Path;

pub struct Instance<T> {
    paths: Paths,
    output: bool,
    state: State,
    inner: T,
}

impl<T> Instance<T> {
    fn get(state: State, paths: Paths, output: bool, version: Option<String>) -> Result<Box<dyn InstanceTrait>, Error> {
        let vanilla = Instance::<Vanilla> { 
            paths: paths.clone(), 
            state: state.clone(),
            output, 
            inner: Vanilla::new(&paths, version)?
        };

        match state.scenario.as_ref() { 
            "vanilla" => Ok(Box::new(vanilla)),
            "fabric" => Ok(Box::new(
                Instance::<Fabric> {
                    inner: Fabric::new(&paths, vanilla)?,  
                    paths, 
                    output,
                    state, 
            })),
            _ => Err(Error::StateError(StateError::ScenarioDoesNotExist(state.scenario)))
        }
    } 
}

pub trait InstanceTrait: LaunchSequence + DownloadSequence {}
impl<T> InstanceTrait for T where T: LaunchSequence + DownloadSequence {}

pub fn download(instance: &str, version: Option<String>, scenario: Option<String>, base_dir: &Path) -> Result<(), Error> {
    let mut paths = Paths::default();
    let instance_path = base_dir.join("instances").join(instance);
    std::fs::create_dir_all(&instance_path)?;

    paths.0.insert("natives".to_string(), instance_path.join("natives")); 
    paths.0.insert("instance".to_string(), instance_path);
    paths.0.insert("meta".to_string(), base_dir.join("meta")); 
    paths.0.insert("assets".to_string(), base_dir.join("assets")); 
    paths.0.insert("libraries".to_string(), base_dir.join("libraries")); 

    let scenario = scenario.unwrap_or_else(|| "vanilla".to_string());
    let state = State::from_scenario(scenario);

    Instance::<Box<dyn InstanceTrait>>::get(state, paths, true, version)?.download()?;

    Ok(())
}

pub fn launch(instance: &str, username: &str, output: bool, base_dir: &Path) -> Result<(), Error> {
    let mut paths = Paths::default();
    let instance_path = base_dir.join("instances").join(instance);

    paths.0.insert("resources".to_string(), instance_path.join("resources")); 
    paths.0.insert("natives".to_string(), instance_path.join("natives"));
    paths.0.insert("instance".to_string(), instance_path);
    paths.0.insert("meta".to_string(), base_dir.join("meta")); 
    paths.0.insert("assets".to_string(), base_dir.join("assets"));
    paths.0.insert("libraries".to_string(), base_dir.join("libraries")); 
    paths.0.insert("accounts".to_string(), base_dir.join("accounts").with_extension("json"));

    let state = State::read(paths.get("instance")?)?; 

    //maybe move this to state.get_version(); 
    let version = {
        if let Component::GameComponent { version, .. } = state.get_component("net.minecraft")? {
            version.to_string()
        } else {
            return Err(Error::StateError(StateError::ComponentNotFound(String::from("net.minecraft"))));
        }
    };

    Instance::<Box<dyn InstanceTrait>>::get(state, paths, output, Some(version))?.launch(username)?;

    Ok(())
}

pub fn login(base_dir: &Path) -> Result<(), Error> {
    let path = base_dir.join("accounts").with_extension("json");

    Accounts::get(&path)?.new_account()?;
    Ok(())
}

pub fn list_instances(base_dir: &Path) -> std::io::Result<Vec<String>> {
    let instance_path = base_dir.join("instances");
    Ok(std::fs::read_dir(instance_path)?
        .map(|x| x.unwrap().file_name().into_string().unwrap())
        .collect::<Vec<String>>()
    )
}