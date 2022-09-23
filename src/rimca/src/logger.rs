use simplelog::{ LevelFilter, LevelPadding, TerminalMode, ColorChoice, CombinedLogger, WriteLogger, TermLogger, Config, ConfigBuilder };
use std::time::{ SystemTime, UNIX_EPOCH };
use std::path::{ Path, PathBuf };

fn config() -> Config {
    ConfigBuilder::new()
        .set_level_padding(LevelPadding::Left)
        .set_time_level(LevelFilter::Trace)
        .build()
}

pub fn init(log_level: &str) -> Result<(), ()> {
    let level_filter = match log_level {
        "warn" => LevelFilter::Warn,
        "error" => LevelFilter::Error,
        "info" => LevelFilter::Info,
        "debug" => LevelFilter::Debug,
        "trace" => LevelFilter::Trace,
        _ => LevelFilter::Off
    };

    init_in(level_filter).map_err(|err| eprintln!("logger failed to initialise: {:?}", err))
}

fn init_in(level: LevelFilter) -> Result<(), ()> {
    let unix_time = format!("{:.0?}.log", SystemTime::now().duration_since(UNIX_EPOCH).unwrap()).replace("s", "");
    Ok(CombinedLogger::init(vec![
        TermLogger::new(level, config(), TerminalMode::Mixed, ColorChoice::Auto),
        WriteLogger::new(LevelFilter::Debug, Config::default(), create_file(&PathBuf::from("poop").join(&unix_time), true, true).unwrap())
    ])
    .unwrap())
}

use std::fs::{File, create_dir_all};
fn create_file(path: &Path, read: bool, write: bool) -> std::io::Result<File> {
    let mut file = std::fs::OpenOptions::new()
        .read(read)
        .write(write)
        .open(path);

    if let Err(_) = file {
        create_dir_all(&path.parent().expect("couldn't get parent path")).unwrap();

        file = std::fs::OpenOptions::new()
            .read(read)
            .write(write)
            .create(true)
            .open(path);
    }

    Ok(file?)
}