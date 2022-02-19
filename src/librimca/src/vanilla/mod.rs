mod api;

use crate::Instance;
use crate::download::DownloadSequence;
use crate::launch::LaunchSequence;
use crate::error::{ Error, LaunchError };

pub struct Vanilla {
	version: String,
	// meta: Meta,
}

impl Vanilla {
	pub fn new(version: Option<&str>) -> Self {
		if let Some(version) = version {
			return Self {
				version: version.to_string()
			}
		} else {
			return Self {
				version: String::from("get latest_version")
			}
		}
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