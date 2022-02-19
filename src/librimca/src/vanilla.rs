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