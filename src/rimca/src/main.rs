mod config;
use config::Config;

mod logger;

use structopt::StructOpt;
use structopt::clap::AppSettings;
use std::path::PathBuf;
use std::str::FromStr;
use structopt::clap::Error;

pub fn main() {
    logger::init("debug").map_err(|err| eprintln!("logger failed to initialise: {:?}", err)).unwrap();
    let cfg: Config = confy::load("rimca", "config").unwrap();

	match Arguments::from_args().command {
		// Command::Login => rimca::auth::Accounts::get().unwrap().new_account().unwrap(),
		// Command::Delete{ instance } => Instance::get(&instance).delete().unwrap(),
        
        Command::Download(dl) => {
			if let Some(fabric) = dl.fabric {
                rimca::download(&dl.instance, dl.version, Some(String::from("fabric")), &cfg.base_dir).unwrap()
			} else {
                rimca::download(&dl.instance, dl.version, Some(String::from("vanilla")), &cfg.base_dir).unwrap()
			}
        },

        // SOSH
        // SOSH
        // SOSH

        Command::Launch(l) => {
        	rimca::launch(&l.instance, &l.username, l.game_output, &cfg.base_dir).unwrap()
        },

        Command::List(list) => {
            if let Some(remote) = list.loader {
                match remote {
                    Remote::Vanilla => {
   //       for mut version in rimca::vanilla::api::versions(v.snapshot).unwrap().into_iter().rev() {
            //  version.release_time.truncate(version.release_time.find("T").unwrap());
            //  println!("{0: <20} {1: <15} {2: <25}", version.id, version.r#type, version.release_time);
            // }  
                    },

                    Remote::Fabric => {
        //  for version in rimca::fabric::api::loaders().unwrap().into_iter().rev() {
        //      println!("{0: <20} {1: <15}", version.version, version.stable)
        //  }      
                    }
                }
                //list remote stuff...
            } else {
                for instance in rimca::list_instances(&cfg.base_dir).unwrap() {
                    println!("{}", instance.as_str());
                }    
            }
        }

        _ => {}
    }
}


#[derive(StructOpt)]
pub struct Arguments {
    #[structopt(subcommand)]
    pub command: Command,
}

#[derive(StructOpt)]
// #[structopt(setting = AppSettings::InferSubcommands)]
pub enum Command {
    #[structopt(alias = "dl", no_version, global_settings = &[AppSettings::DisableVersion])]
    ///Download minecraft version as an instance
    Download(Download),

    #[structopt(alias = "del", no_version, global_settings = &[AppSettings::DisableVersion])]
    ///Download minecraft version as an instance
    Delete { instance: String },

    #[structopt(alias = "l", no_version, global_settings = &[AppSettings::DisableVersion])]
    ///Launch minecraft instance
    Launch(Launch),

    #[structopt(alias = "ls", no_version, global_settings = &[AppSettings::DisableVersion])]
    ///List installed minecraft instances
    List(List),

    #[structopt(no_version, global_settings = &[AppSettings::DisableVersion])]
    ///Verify integrity of game files of instance
    Verify { instance: String },

    #[structopt(no_version, global_settings = &[AppSettings::DisableVersion])]
    ///Login
    Login,

    #[structopt(no_version, global_settings = &[AppSettings::DisableVersion])]
    ///Login
    Logout { username: String },
}

#[derive(Debug)]
#[derive(StructOpt)]
pub struct Download {
    pub instance: String,
    ///Vanilla version [default: latest_version]
    pub version: Option<String>,
    #[structopt(long, conflicts_with="fabric", value_name="version", require_equals=true)]
    ///Include forge [default version: latest_stable_version]
    pub forge: Option<Option<String>>,
    #[structopt(long, conflicts_with="forge", value_name="version", require_equals=true)]
    ///Include fabric [default version: latest_stable_version]
    pub fabric: Option<Option<String>>
}

#[derive(StructOpt)]
pub struct Launch {
    pub instance: String,
    pub username: String,
    #[structopt(short = "q", long)]
    ///Print output of game to terminal
    pub game_output: bool,
}

#[derive(StructOpt)]
pub struct List {
    #[structopt(short = "r", long= "--remote")]
    ///Print output of game to terminal
    pub loader: Option<Remote>,
    #[structopt(short = "s", long)]
    ///List snapshot/unstable versions
    pub snapshot: bool,
}

impl FromStr for Remote {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "vanilla" => Ok(Remote::Vanilla),
            "fabric" => Ok(Remote::Fabric),
            _ => todo!()
        }
    }
}

#[derive(StructOpt)]
pub enum Remote {
    ///List vanilla versions
    Vanilla,
    ///List fabric versions
    Fabric,
}