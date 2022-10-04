use crate::{ Instance, Paths };
use crate::error::{ LaunchError, StateError };
use crate::state::{ State, Component };
use std::process::{ Command, Stdio };

pub trait LaunchHelper {
    fn state(&self) -> &State;
    fn paths(&self) -> &Paths;
    fn output(&self) -> bool;
}

impl <T> LaunchHelper for Instance<T> {
    fn state(&self) -> &State {
        &self.state
    }

    fn paths(&self) -> &Paths {
        &self.paths
    } 

    fn output(&self) -> bool {
        self.output
    }
}

pub trait LaunchSequence: LaunchHelper {
    fn launch(&self, username: &str) -> Result<(), LaunchError> {
        let game_opts = self.get_game_options(username)?;
        log::debug!("Game Options: {:?}", game_opts);

        let classpath = self.get_classpath()?;
        log::debug!("Classpath: {}", classpath);

        let jvm_args = self.get_jvm_arguments(&classpath)?;
        log::debug!("Jvm Arguments: {:?}", jvm_args);

        let main_class = self.get_main_class()?;
        log::debug!("Main Class: {}", main_class);

        self.execute(jvm_args, &main_class, game_opts)?;
        Ok(())
    }

    fn get_main_class(&self) -> Result<String, LaunchError>;
    fn get_game_options(&self, username: &str) -> Result<Vec<String>, LaunchError>;
    fn get_classpath(&self) -> Result<String, LaunchError>;
    fn get_jvm_arguments(&self, classpath: &str) -> Result<Vec<String>, LaunchError>;

    fn execute(&self, jvm_args: Vec<String>, main_class: &str, game_opts: Vec<String>) -> Result<(), LaunchError> {
        if let Ok(Component::JavaComponent { path, .. }) = self.state().get_component("java") {
            let (exe, args) = match &self.state().wrapper {
                Some(wrapper) => (wrapper.as_str(), &["java"][..]),
                None => (path.as_str(), &[][..]),
            };

            let mut command = Command::new(exe);
            command.args(args);
            command.current_dir(self.paths().get("instance")?)
                .args(jvm_args)
                .arg(main_class)
                .args(game_opts);

            if self.output() {
                log::info!("JVM output disabled");
                command.stdout(Stdio::null()).stderr(Stdio::null());
            }

            log::debug!("Spawning command: {:?}", command);
            command.spawn()?;

            return Ok(())
        }

        Err(LaunchError::StateError(StateError::ComponentNotFound(String::from("java"))))
    }
}