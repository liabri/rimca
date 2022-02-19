use std::io::{ BufReader, BufWriter, Seek, SeekFrom };
use std::fs::File;
use serde::{ Serialize, Deserialize };
use std::collections::HashMap;
use std::path::Path;
use crate::error::StateError;

#[derive(Serialize, Deserialize)]
pub struct State {
	pub scenario: String,
	pub components: HashMap<String, Component>,
	pub wrapper: Option<String>,
	pub prelaunch_cmds: Option<Vec<String>>
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Component {
    GameComponent { 
    	asset_index: Option<String>, 
    	version: String 
    },

    JavaComponent { 
    	path: String, 
    	arguments: Option<String> 
    }
}

impl State {
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
        let file = File::create(&path)?;
        let reader = BufReader::new(file);
        let options: Self = serde_json::from_reader(reader)?;
        Ok(options)
    }
}