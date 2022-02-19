mod download;

mod error;
use error::Error;

use std::path::PathBuf;

pub struct Instance {
	name: String,
	path: PathBuf,
	// options: 
}

pub enum InstanceType {
	Vanilla(Option<String>), 					//game_version
	Fabric(Option<String>, Option<String>), 	//game_version, loader_version
}

impl Instance {
	fn delete(&self) -> Result<(), Error> {
		std::fs::remove_dir_all(&self.path)?;
		Ok(())
	}

	pub fn launch(&self, username: &str) -> Result<(), Error> {
		Ok(())
	}

	pub fn download(&self, instance_type: &InstanceType) -> Result<(), Error> {
		Ok(())
	}
}