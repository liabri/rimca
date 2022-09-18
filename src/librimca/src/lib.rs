mod state;
use state::State;

mod download;
pub use download::DownloadSequence;

mod launch;
pub use launch::LaunchSequence;

pub mod vanilla;
pub use vanilla::Vanilla;

mod error;
pub use error::{ Error, StateError };

mod verify;

mod paths;
use paths::Paths;

use std::path::PathBuf;

pub struct Instance<T> {
	paths: Paths,
	state: State,
	inner: T,
}

pub trait InstanceTrait: LaunchSequence + DownloadSequence {}
impl<T> InstanceTrait for T where T: LaunchSequence + DownloadSequence {}

impl<T> Instance<T> {
	fn get(state: State, paths: Paths, version: Option<String>) -> Result<Box<dyn InstanceTrait>, Error> {
		match state.scenario.as_ref() { 
			"vanilla" => Ok(Box::new(Instance::<Vanilla> { paths, state, inner: Vanilla::from(version) })),
		  	_ => Err(Error::StateError(StateError::ScenarioDoesNotExist(String::from(state.scenario))))
		}
	}	
}

pub fn download(instance: String, version: Option<String>, scenario: Option<String>) -> Result<(), Error> {
	let mut paths = Paths::new();
	let base_dir = PathBuf::from("/home/liabri/loghob/minecraft/rimca/");
	let instance_path = base_dir.join("instances").join(&instance);
	std::fs::create_dir_all(&instance_path)?;

	paths.0.insert("natives".to_string(), instance_path.join("natives")); 
	paths.0.insert("instance".to_string(), instance_path);
	paths.0.insert("meta".to_string(), base_dir.join("meta")); 
	paths.0.insert("assets".to_string(), base_dir.join("assets")); 
	paths.0.insert("libraries".to_string(), base_dir.join("libraries")); 

	let scenario = scenario.unwrap_or("vanilla".to_string());
	let state = State::from_scenario(scenario);

	Instance::<Box<dyn InstanceTrait>>::get(state, paths, None)?.download();

	Ok(())
}

pub fn launch(instance: &str, username: &str) -> Result<(), Error> {
	let mut paths = Paths::new();
	let base_dir = PathBuf::from("/home/liabri/loghob/minecraft/rimca/");
	let instance_path = base_dir.join("instances").join(instance);

	let state = State::read(&instance_path)?;
	paths.0.insert("resources".to_string(), instance_path.join("resources")); 
	paths.0.insert("natives".to_string(), instance_path.join("natives"));
	paths.0.insert("instance".to_string(), instance_path);
	paths.0.insert("meta".to_string(), base_dir.join("meta")); 
	paths.0.insert("assets".to_string(), base_dir.join("assets"));
	paths.0.insert("libraries".to_string(), base_dir.join("libraries")); 

	let state = State::read(&paths.get("instance")?)?;	

	Instance::<Box<dyn InstanceTrait>>::get(state, paths, None)?.launch(username);

	Ok(())
}