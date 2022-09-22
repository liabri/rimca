use crate::error::DownloadError;

use nizziel::{ download, Downloads };

pub trait DownloadSequence {
    fn collect_urls(&mut self) -> Result<Downloads, DownloadError>;
    fn create_state(&mut self) -> Result<(), DownloadError>;

    fn download(&mut self) -> Result<(), DownloadError> {
        self.create_state()?;
        let urls = self.collect_urls()?;
        self.spawn_thread(urls)
    }

    fn spawn_thread(&mut self, dls: Downloads) -> Result<(), DownloadError> {
        log::info!("Downloading!");

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

        log::info!("Time taken: {:.2?}", before.elapsed());
        Ok(())      
    }
}