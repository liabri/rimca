mod download;

mod vanilla;

mod error;
use error::Error;

use std::path::PathBuf;

pub struct Instance {
	name: String,
	path: PathBuf,
	// options: 
}

impl Instance {
	fn delete(&self) -> Result<(), Error> {
		std::fs::remove_dir_all(&self.path)?;
		Ok(())
	}

	pub fn launch(&self, username: &str) -> Result<(), Error> {
		Ok(())
	}

	pub fn download(&self, instance_type: &dyn InstanceType) -> Result<(), Error> {
		Ok(())
	}
}

pub trait InstanceType {}
impl InstanceType for vanilla::Vanilla {}