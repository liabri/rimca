use std::io::{ BufReader, BufWriter, Seek, SeekFrom };
use std::fs::File;
use serde::{ Serialize, Deserialize };
use std::collections::HashMap;
use std::path::Path;
use crate::error::StateError;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct State {
    pub scenario: String,
    pub components: HashMap<String, Component>,
    pub wrapper: Option<String>,
    pub prelaunch_cmds: Option<Vec<String>>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Component {
    GameComponent { 
        version: String 
    },

    JavaComponent { 
        path: String, 
        arguments: Option<String> 
    }
}

impl State {
    pub fn from_scenario(scenario: String) -> Self {
        State {
            scenario,
            components: HashMap::new(),
            wrapper: None,
            prelaunch_cmds: None
        }
    }

    pub fn get_component(&self, key: &str) -> Result<&Component, StateError> {
        self.components.get(key).ok_or_else(|| StateError::ComponentNotFound(String::from(key)))
    }

    pub fn write(&self, instance_path: &Path) -> Result<(), StateError> {
        let path = instance_path.join("state.json");
        let file = File::create(&path)?;
        let reader: BufReader<File> = BufReader::new(file);
        let mut file = reader.into_inner();
        file.seek(SeekFrom::Start(0))?;
        let mut writer = BufWriter::new(file);

        serde_json::to_writer_pretty(&mut writer, &self)?;
        Ok(())
    }

    pub fn read(instance_path: &Path) -> Result<Self, StateError> {
        let path = instance_path.join("state.json");
        let file = File::open(&path)?;
        let reader = BufReader::new(file);
        let options: Self = serde_json::from_reader(reader)?;
        Ok(options)
    }
}