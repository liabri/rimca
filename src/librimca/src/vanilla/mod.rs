mod api;

use crate::Instance;
use crate::state::State;
use crate::download::DownloadSequence;
use crate::launch::LaunchSequence;
use crate::error::{ Error, LaunchError };

use std::path::PathBuf;

pub struct Vanilla {
	pub version: Option<String>,
	// meta: Meta,
}

impl Vanilla {
	pub fn new() -> Self {
		Self {
			version: None
		}
	}

	pub fn from_version(version: Option<&str>) -> Self {
		if let Some(version) = version {
			return Self {
				version: Some(version.to_string())
			}
		} else {
			return Self {
				version: Some(String::from("get latest_version"))
			}
		}
	}
}

impl Instance<Vanilla> {
	pub fn new(path: PathBuf, state: State, name: String) -> Self {
		Self { name, path, inner: Vanilla::new(), state }
	}

	pub fn with_version(name: String, version: Option<&str>) -> Result<Self, Error> {
		let path = PathBuf::new().join(&name);
		let state = State::read(&path)?;

		Ok(Self {
			name,
			path,
			state,
			inner: Vanilla::from_version(version),
		})
	}
}

impl DownloadSequence for Instance<Vanilla> {
	fn download(&self) -> Result<(), Error> {
		Ok(())
	}

	fn spawn_thread(&self) -> Result<(), Error> {
		Ok(())
	}
}

impl LaunchSequence for Instance<Vanilla> {
	fn get_main_class(&self) -> Result<&str, LaunchError> { todo!() }
	fn execute(&self, jvm_args: Vec<String>, main_class: &str, game_opts: Vec<String>) -> Result<(), LaunchError> { todo!() }
	fn get_jvm_arguments(&self, classpath: &str) -> Result<Vec<String>, LaunchError> { todo!() }
	fn get_game_options(&self) -> Result<Vec<String>, LaunchError> { todo!() }
	fn get_classpath(&self) -> Result<String, LaunchError> { todo!() }
}