use crate::error::Error;
use crate::Instance;

pub trait DownloadSequence {
	fn download(&self) -> Result<(), Error>;
	// fn collect_urls(&self) -> Result<request::Downloads, DownloadError>;
	fn spawn_thread(&self) -> Result<(), Error>;
}