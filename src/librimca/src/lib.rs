mod state;
use state::State;

mod download;
pub use download::DownloadSequence;

mod launch;
pub use launch::LaunchSequence;

mod vanilla;
pub use vanilla::Vanilla;

mod error;
pub use error::Error;

mod verify;

use std::collections::HashMap;
use std::path::PathBuf;

pub struct Instance<T> {
	name: String,
	paths: Paths,
	state: State,
	inner: T,
}

impl<T: LaunchSequence + DownloadSequence> Instance<T> {
	fn delete(&self) -> Result<(), Error> {
		Ok(std::fs::remove_dir_all(&self.paths.get("instance")?)?)
	}

	fn launch(&self, username: &str) -> Result<(), Error> {
		Ok(self.inner.launch(username)?)
	}

	fn download(&self) -> Result<(), Error> {
		Ok(self.inner.download()?)
	}
}

pub fn download(instance: &str/*, version: Option<&str>*/) -> Result<(), Error> {
	let mut paths = Paths::new();
	let base_dir = PathBuf::from("/home/liabri/loghob/minecraft/rimca/");
	let instance_path = base_dir.join("instances").join(instance);

	std::fs::create_dir_all(&instance_path);

	paths.0.insert("natives".to_string(), instance_path.join("natives")); 
	paths.0.insert("instance".to_string(), instance_path);
	paths.0.insert("meta".to_string(), base_dir.join("meta")); 
	paths.0.insert("assets".to_string(), base_dir.join("assets")); 
	paths.0.insert("libraries".to_string(), base_dir.join("libraries")); 

	// kinda shady to make an empty one just to write a completely different one later.
	let state = State::from_scenario(String::from("vanilla"));

	let inner = match state.scenario.as_str() {
		"vanilla" => Instance::<Vanilla> { 
			name: instance.to_string(),
			paths, 
			state,
			inner: Vanilla::new()
		}.download()?,

		_ => return Err(Error::InstanceDoesNotExist)
	};

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


	let inner = match state.scenario.as_str() {
		// "fabric" => Instance::<Fabric>::new(path, state, name.to_string()).launch(),
		"vanilla" => Instance::<Vanilla> { 
			name: instance.to_string(),
			paths, 
			state,
			inner: Vanilla::new()
		}.launch(username)?,

		_ => return Err(Error::InstanceDoesNotExist)
	};

	Ok(())
}

use crate::error::PathError;
pub struct Paths(HashMap<String, PathBuf>);

impl Paths {
	pub fn get(&self, key: &str) -> Result<&PathBuf, PathError> {
        self.0.get(key).ok_or(PathError::NotFound(String::from(key)))
	}

	pub fn new() -> Self {
		Paths(HashMap::new())
	}
}