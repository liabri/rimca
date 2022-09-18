use crate::error::LaunchError;
use crate::vanilla::models::Meta;

pub trait LaunchSequence {
    fn launch(&self, username: &str) -> Result<(), LaunchError> {
        let meta = self.get_meta()?;
        let game_opts = self.get_game_options(username, &meta)?;
        let classpath = self.get_classpath(&meta)?;
        let jvm_args = self.get_jvm_arguments(&classpath, &meta)?;
        let main_class = meta.main_class;

        self.execute(jvm_args, &main_class, game_opts)?;
        Ok(())
    }

    fn get_meta(&self) -> Result<Meta, LaunchError>;
    fn get_game_options(&self, username: &str, meta: &Meta) -> Result<Vec<String>, LaunchError>;
    fn get_classpath(&self, meta: &Meta) -> Result<String, LaunchError>;
    fn get_jvm_arguments(&self, classpath: &str, meta: &Meta) -> Result<Vec<String>, LaunchError>;
    fn execute(&self, jvm_args: Vec<String>, main_class: &str, game_opts: Vec<String>) -> Result<(), LaunchError>;
}