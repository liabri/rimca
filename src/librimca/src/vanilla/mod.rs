mod api;

use crate::download::{ DownloadSequence, Download };
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

impl DownloadSequence for Download<Vanilla> {
	fn commence(&self) -> Result<(), Error> {
		Ok(())
	}

	fn spawn_thread(&self) -> Result<(), Error> {
		Ok(())
	}
}