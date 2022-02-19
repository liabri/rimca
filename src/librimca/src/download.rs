use crate::error::DownloadError;
use crate::Instance;

use nizziel::Downloads;

pub trait DownloadSequence {
	fn download(&self) -> Result<(), DownloadError>;
	fn collect_urls(&self) -> Result<Downloads, DownloadError>;
	fn spawn_thread(&self) -> Result<(), DownloadError>;
}