use crate::error::LaunchError;
use crate::vanilla::models::Meta;

pub trait LaunchSequence {
    fn launch(&self, username: &str) -> Result<(), LaunchError> {
        let game_opts = self.get_game_options(username)?;
        let classpath = self.get_classpath()?;
        let jvm_args = self.get_jvm_arguments(&classpath)?;
        let main_class = self.get_main_class()?;

        self.execute(jvm_args, &main_class, game_opts)?;
        Ok(())
    }

    fn get_main_class(&self) -> Result<String, LaunchError>;
    // fn get_meta(&self) -> Result<Meta, LaunchError>;
    fn get_game_options(&self, username: &str) -> Result<Vec<String>, LaunchError>;
    fn get_classpath(&self) -> Result<String, LaunchError>;
    fn get_jvm_arguments(&self, classpath: &str) -> Result<Vec<String>, LaunchError>;
    fn execute(&self, jvm_args: Vec<String>, main_class: &str, game_opts: Vec<String>) -> Result<(), LaunchError>;
}