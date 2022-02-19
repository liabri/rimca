mod download;
use download::DownloadSequence;

mod vanilla;
use vanilla::Vanilla;

mod error;
use error::Error;

use std::path::PathBuf;

pub struct Instance<T> {
	name: String,
	path: PathBuf,
	inner: T,
	// options: 
}

impl<T: DownloadSequence> Instance<T> {
	fn delete(&self) -> Result<(), Error> {
		std::fs::remove_dir_all(&self.path)?;
		Ok(())
	}

	pub fn launch(&self, username: &str) -> Result<(), Error> {
		Ok(())
	}

	pub fn download(&self) -> Result<(), Error> {
		self.inner.download()
	}
}

// pub trait InstanceType {}
// impl InstanceType for Vanilla {}