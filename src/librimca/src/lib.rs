mod state;
use state::State;

mod download;
pub use download::DownloadSequence;

mod launch;
pub use launch::LaunchSequence;

mod vanilla;
pub use vanilla::Vanilla;

mod error;
use error::Error;

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

	pub fn launch(&self, username: &str) -> Result<(), Error> {
		Ok(self.inner.launch()?)
	}

	pub fn download(&self) -> Result<(), Error> {
		Ok(self.inner.download()?)
	}
}

pub fn launch(name: &str, username: &str) -> Result<(), Error> {
	let mut paths = HashMap::new();
	let instance_path = PathBuf::new().join(name);

	let state = State::read(&instance_path)?;

	paths.insert("instance".to_string(), instance_path);

	let inner = match state.scenario.as_str() {
		// "fabric" => Instance::<Fabric>::new(path, state, name.to_string()).launch(),
		"vanilla" => Instance::<Vanilla> { 
			name: name.to_string(),
			paths, 
			state,
			inner: Vanilla::new()
		}.launch(),

		_ => return Err(Error::InstanceDoesNotExist)
	};

	Ok(())
}