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
	paths: HashMap<String, PathBuf>,
	state: State,
	inner: T,
}

impl<T: LaunchSequence + DownloadSequence> Instance<T> {
	fn delete(&self) -> Result<(), Error> {
		Ok(std::fs::remove_dir_all(&self.paths.get("instance").ok_or(Error::InstanceDoesNotExist)?)?)
	}

	fn launch(&self, username: &str) -> Result<(), Error> {
		Ok(self.inner.launch(username)?)
	}

	fn download(&self) -> Result<(), Error> {
		Ok(self.inner.download()?)
	}

	// fn get(name: &str) -> Result<Self, Error> {
	// 	let mut paths = HashMap::new();
	// 	let instance_path = PathBuf::new().join(name);
	// 	let state = State::read(&instance_path)?;
	// 	paths.insert("instance".to_string(), instance_path);

	// 	match state.scenario.as_str() {
	// 		"vanilla" => Ok(Instance::<Vanilla> { 
	// 			name: name.to_string(),
	// 			paths, 
	// 			state,
	// 			inner: Vanilla::new()
	// 		}),

	// 		_ => return Err(Error::InstanceDoesNotExist)
	// 	}
	// }
}

pub fn download(instance: &str/*, version: Option<&str>*/) -> Result<(), Error> {
	let mut paths = HashMap::new();
	let base_dir = PathBuf::from("/home/liabri/loghob/minecraft/rimca/");
	let instance_path = base_dir.join("instances").join(instance);

	std::fs::create_dir_all(&instance_path);

	paths.insert("natives".to_string(), instance_path.join("natives")); 
	paths.insert("instance".to_string(), instance_path);
	paths.insert("meta".to_string(), base_dir.join("meta")); 
	paths.insert("assets".to_string(), base_dir.join("assets")); 
	paths.insert("libraries".to_string(), base_dir.join("libraries")); 

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
	let mut paths = HashMap::new();
	let base_dir = PathBuf::from("/home/liabri/loghob/minecraft/rimca/");
	let instance_path = base_dir.join("instances").join(instance);

	let state = State::read(&instance_path)?;
	paths.insert("resources".to_string(), instance_path.join("resources")); 
	paths.insert("natives".to_string(), instance_path.join("natives"));
	paths.insert("instance".to_string(), instance_path);
	paths.insert("meta".to_string(), base_dir.join("meta")); 
	paths.insert("assets".to_string(), base_dir.join("assets"));

	let inner = match state.scenario.as_str() {
		// "fabric" => Instance::<Fabric>::new(path, state, name.to_string()).launch(),
		"vanilla" => Instance::<Vanilla> { 
			name: instance.to_string(),
			paths, 
			state,
			inner: Vanilla::new()
		}.launch(username),

		_ => return Err(Error::InstanceDoesNotExist)
	};

	Ok(())
}