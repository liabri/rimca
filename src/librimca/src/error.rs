use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("launch error: `{0}`")]
    LaunchError(#[from] LaunchError),
}

#[derive(Error, Debug)]
pub enum LaunchError {
    // #[error("pussy anyhow")]
    // Temporary(#[from] anyhow::Error),
    #[error("`{0}` arguments were not found")]
    ArgumentsNotFound(LaunchArguments),
    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("account error: {0}")]
    AccountError(#[from] AccountError),
    #[error("state error: {0}")]
    StateError(#[from] StateError)
}

#[derive(Error, Debug)]
pub enum LaunchArguments {
    Game,
    Java
}

impl std::fmt::Display for LaunchArguments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Game => write!(f, "game"),
            Self::Java => write!(f, "java")
        }
    }
}

#[derive(Error, Debug)]
pub enum StateError {
    #[error("scenario could not be found")]
    ScenarioDoesNotExist,
    #[error("the launch_options file cannot be found for instance: `{0}`")]
    CannotFind(String),
    #[error("cannot find component: `{0}` in launch_options")]
    ComponentNotFound(String),
    #[error("cannot find field: `{0}` in component: `{1}` in launch_options")]
    FieldNotFound(String, String),
    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("serde_json error: {0}")]
    SerdeJsonError(#[from] serde_json::Error),
}


//So, id like to move all serde_json and io errors into Request if possible, abstracting ftw!

#[derive(Error, Debug)]
pub enum AccountError {
    #[error("could not find xui user-hash")]
    CannotFindXUI,
    //NEED TO ABSTRACT REQWEST TO REQUEST STILL HERE
    #[error("reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("could not get authorisation code from microsoft services")]
    AuthorisationCodeFailure,
    //acccountsssss
    #[error("could not find account `{0}`")]
    CannotFindAccount(String),
    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("serde_json error: {0}")]
    SerdeJsonError(#[from] serde_json::Error),
    // #[error("io error: {0}")]
    // OpenerError(#[from] opener::OpenError),
    // #[error("io error: {0}")]
    // UrlParseError(#[from] url::ParseError),
}

#[derive(Error, Debug)]
pub enum DownloadError {
    // #[error("pussy anyhow")]
    // Temporary(#[from] anyhow::Error),
    #[error("instance: `{0}` already exists")]
    InstanceExists(String),
    #[error("library: `{0}` has no classifiers")]
    LibraryNoClassifiers(String),
    #[error("game version: `{0}` not found")]
    GameVersionNotFound(String),
    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("launch options error: {0}")]
    StateError(#[from] StateError),
    #[error("serde_json error: {0}")]
    SerdeJsonError(#[from] serde_json::Error),
    #[error("reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("api error: {0}")]
    ApiError(#[from] ApiError),
    #[error("nizziel error: {0}")]
    NizzielError(#[from] nizziel::Error),
    #[error("temp")]
    Temp,
}

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("loader does not exist for game version: `{0}`")]
    LoaderDoesNotExistForGameVer(String),
    #[error("cannot find latest version")]
    CannotFindLatestVersion,
    #[error("cannot find version `{0}`")]
    CannotFindVersion(String),
    #[error("io error: {0}")]
    SerdeJsonError(#[from] serde_json::Error),
    #[error("nizziel error: {0}")]
    NizzielError(#[from] nizziel::Error),
    #[error("reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),
}