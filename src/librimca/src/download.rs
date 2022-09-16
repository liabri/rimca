use crate::error::DownloadError;

use nizziel::{ download, Downloads };

pub trait DownloadSequence {
	fn download(&self) -> Result<(), DownloadError>;
	fn collect_urls(&self) -> Result<Downloads, DownloadError>;
	fn spawn_thread(&self, dls: Downloads) -> Result<(), DownloadError> {
				// if self.verify {
		// 	log::info!("Verified integrity of game files")
		// } else {
			// log::info!("Downloading files");
		// }

		println!("Downloading!!");

		let before = std::time::Instant::now();
		let rt = tokio::runtime::Builder::new_multi_thread()
			.worker_threads(10)
			.enable_io()
			.enable_time()
			.build()?;

		rt.block_on(
			async move {
				download(dls).await.unwrap();
			}
		);


		println!("Time taken: {:.2?}", before.elapsed());
		// log::info!("Time taken: {:.2?}", before.elapsed());
		Ok(())		
	}
}