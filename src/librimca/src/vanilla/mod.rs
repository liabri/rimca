mod api;

use crate::Instance;
use crate::download::DownloadSequence;
use crate::error::Error;

pub struct Vanilla {
	version: String,
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