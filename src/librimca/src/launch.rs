use std::process::{Command, Stdio};
use crate::error::LaunchError;

pub trait LaunchSequence {
	fn launch(&self) -> Result<(), LaunchError> {
		let jvm_args = self.get_jvm_arguments(&self.get_classpath()?)?;
		let main_class = self.get_main_class()?;
		let game_opts = self.get_game_options()?;

		self.execute(jvm_args, main_class, game_opts)?;
		Ok(())
	}

	fn get_main_class(&self) -> Result<&str, LaunchError>;
	fn execute(&self, jvm_args: Vec<String>, main_class: &str, game_opts: Vec<String>) -> Result<(), LaunchError>;
	fn get_jvm_arguments(&self, classpath: &str) -> Result<Vec<String>, LaunchError>;
	fn get_game_options(&self) -> Result<Vec<String>, LaunchError>;
	fn get_classpath(&self) -> Result<String, LaunchError>;
}