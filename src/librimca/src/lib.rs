use std::path::PathBuf;

pub struct Instance {
	name: String,
	path: PathBuf,
	// options: 
}

pub enum InstanceType {
	Vanilla(Option<String>),
	Fabric(Option<String>, Option<String>),
}

impl Instance {
	fn delete(&self) -> std::io::Result<()> {
		std::fs::remove_dir_all(&self.path)?;
		Ok(())
	}

	pub fn launch(&self, username: &str) {}
	pub fn download(&self, instance_type: &InstanceType) {}
}