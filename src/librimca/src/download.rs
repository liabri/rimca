use crate::error::Error;
use crate::Instance;

pub struct Download<T> {
	pub instance: Instance,
	pub inner: T,
}

pub trait DownloadSequence {
	fn commence(&self) -> Result<(), Error>;
	// fn collect_urls(&self) -> Result<request::Downloads, DownloadError>;
	fn spawn_thread(&self) -> Result<(), Error>;
}