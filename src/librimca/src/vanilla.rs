pub struct Vanilla {
	version: String,
}

impl Vanilla {
	pub fn new(version: Option<&str>) -> Self {
		if let None = version {
			//get latest_version
		} else {
			Self {
				version
			}
		}

		todo!()
	}
}