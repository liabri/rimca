use crate::error::Error;
use crate::Instance;

// pub struct Download<T> {
// 	pub instance: Instance<T>,
// }

// impl<T> From<T> for Download<T> {
// 	fn from(inst: &dyn InstanceType) -> Self {
// 		todo!()
// 	}
// }

pub trait DownloadSequence {
	fn download(&self) -> Result<(), Error>;
	// fn collect_urls(&self) -> Result<request::Downloads, DownloadError>;
	fn spawn_thread(&self) -> Result<(), Error>;
}