use structopt::StructOpt;
use structopt::clap::AppSettings;

pub fn main() {
	match Arguments::from_args().command {
		// Command::Login => rimca::auth::Accounts::get().unwrap().new_account().unwrap(),
		// Command::Delete{ instance } => Instance::get(&instance).delete().unwrap(),
        Command::Download(dl) => {
			// if let Some(fabric) = dl.fabric {
			// 	// rimca::Instance::<Fabric>::new(dl.version, fabric).download().unwrap();
			// } else {
                rimca::download(dl.instance, dl.version).unwrap()
			// 	rimca::Instance::<Vanilla>::download(dl.instance/*, dl.version.as_ref().map(|x| &**x)*/).unwrap().download().unwrap();
			// }
        },


        // SOSH
        // SOSH
        // SOSH

        Command::Launch(l) => {
        	rimca::launch(&l.instance, &l.username).unwrap()
        },

        Command::List(List::Remote(Remote::Vanilla(v))) => {
        	for mut version in rimca::vanilla::api::versions(v.snapshot).unwrap().into_iter().rev() {
				version.release_time.truncate(version.release_time.find("T").unwrap());
				println!("{0: <20} {1: <15} {2: <25}", version.id, version.r#type, version.release_time);
			}
		},

  //       Command::List(List::Remote(Remote::Fabric(_))) => {
		// 	for version in rimca::fabric::api::loaders().unwrap().into_iter().rev() {
		// 		println!("{0: <20} {1: <15}", version.version, version.stable)
		// 	}
		// },

        Command::List(List::Local) => {
		 //    for instance in rimca::list_instances().unwrap() {
			// 	println!("{}", instance.as_str());
			// }	
		},

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
    #[structopt(short = "q" , long)]
    ///Print output of game to terminal
    pub game_output: bool,
}

#[derive(StructOpt)]
pub enum List {
    ///Remote objects
    Remote(Remote),
    ///Local objects
    Local,
}

#[derive(StructOpt)]
pub enum Remote {
    ///List vanilla versions
    Vanilla(VanillaList),
    ///List fabric versions
    Fabric(FabricList),
}

#[derive(StructOpt)]
pub struct VanillaList {
    #[structopt(short = "s", long)]
    ///List snapshot versions
    pub snapshot: bool
}

#[derive(StructOpt)]
pub struct FabricList {
    #[structopt(short = "u", long)]
    ///List unstable versions
    pub unstable: bool,

    #[structopt(value_name="fabric-version", long)]
    ///List vanilla versions comptable with the fabric version specified
    pub version: Option<String>,

    #[structopt(value_name="vanilla-version", long)]
    ///List fabric versions comptable with the vanilla version specified
    pub game_version: Option<String>
}