use directories_next::BaseDirs;
use serde::{ Serialize, Deserialize };
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
	pub base_dir: PathBuf
}

impl Default for Config {
	fn default() -> Self {
		Config {
			base_dir: BaseDirs::new().unwrap().home_dir().join(".minecraft")
		}
	}
}