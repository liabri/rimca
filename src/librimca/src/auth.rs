use serde::{ Serialize, Deserialize };
use std::io::{ BufReader, BufWriter, BufRead, Write };
use std::path::{ Path, PathBuf };
use oauth2::basic::BasicClient;
use std::fs::File;
use oauth2::{
    AuthUrl,
    ClientId,
    CsrfToken,
    RedirectUrl,
    Scope,
    ResponseType
};

use crate::error::AccountError;

//implement lazy_static and read from file, so if urls change no need to recompile
const CLIENT_ID: &str = "9963c094-1077-4c84-bf98-dcf47483272b";
// const CLIENT_SECRET: &str = "Dez7Q~Ku-.sEcIiUXlXvbEL4U6NKSY3khQ6tl";//"a0fc7a6b-c6e0-444e-a85b-4782c7741973";

const MICROSOFT_TOKEN_URL: &str = "https://login.live.com/oauth20_token.srf";

const LOGIN_URL: &str = "https://api.minecraftservices.com/authentication/login_with_xbox";
const PROFILE_URL: &str = "https://api.minecraftservices.com/minecraft/profile";

const XBOX_AUTH_REQUEST_TYPE: &str = "JWT";
const XSTS_AUTHENTICATION_URL: &str = "https://xsts.auth.xboxlive.com/xsts/authorize";
const XBOX_AUTH_RELYING_PARTY: &str = "http://auth.xboxlive.com";

const XBL_AUTHENTICATION_URL: &str = "https://user.auth.xboxlive.com/user/authenticate";
const XBL_AUTH_PROPERTIES_METHOD: &str = "RPS";
const XBL_AUTH_PROPERTIES_SITE: &str = "user.auth.xboxlive.com";

#[derive(Default, Debug, Serialize, Deserialize)]
// pub struct Accounts(Vec<Account>);
pub struct Accounts {
    inner: Vec<Account>,
    #[serde(skip)] 
    path: PathBuf
}

impl Accounts {
    pub fn get(path: &Path) -> Result<Self, AccountError> {
        match Self::read(path) {
            Ok(x) => return Ok(x),
            Err(_) => {
                //If file doesn't exist, create it
                std::fs::create_dir_all(&path.parent().unwrap())?;
                std::fs::OpenOptions::new().create(true);

                //If file is empty, return default self
                return Ok(Self {
                    path: path.to_path_buf(),
                    inner: Vec::default()
                })
            }
        }
    }

    fn read(path: &Path) -> Result<Self, AccountError> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        Ok(serde_json::from_reader(reader)?)
    }

    pub fn write(&self) -> Result<(), AccountError> {
        let file = std::fs::OpenOptions::new().write(true).open(&self.path)?;
        let mut writer = BufWriter::new(file);
        serde_json::to_writer_pretty(&mut writer, &self)?;
        writer.flush()?;
        Ok(())
    }

    pub fn get_account(self, username: &str) -> Result<Account, AccountError> {
        self.inner.into_iter().find(|x| x.name == username)
            .ok_or(AccountError::CannotFindAccount(username.to_string()))
    }

    pub fn new_account(&mut self) -> Result<(), AccountError> {
        self.inner.push(Account::new()?);
        self.write()
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Account {
    pub access_token: String,
    pub refresh_token: String,
    pub name: String,
    pub uuid: String,
}

// API
impl Account {
    fn new() -> Result<Self, AccountError> {
        let mut account: Self = Self::default();

        let authorisation_code = account.get_authorisation_code()?;
        account.get_tokens(&authorisation_code)?;
        
        let xauth = account.get_xauth_response()?;
        account.get_auth_response(xauth)?;
        account.get_user_profile()?;

        Ok(account)
    }

    fn get_authorisation_code(&mut self) -> Result<String, AccountError> {
        let client = BasicClient::new(
            ClientId::new(String::from(CLIENT_ID)),
            None,
            AuthUrl::new("https://login.live.com/oauth20_authorize.srf".to_string())?,
            None
        ).set_redirect_uri(RedirectUrl::new("http://localhost:8594".to_string())?);

        let (auth_url, _) = client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new("XboxLive.signin".to_string()))
            .add_scope(Scope::new("XboxLive.offline_access".to_string()))
            .set_response_type(&ResponseType::new("code".to_string()))
            .url();

        // Open url in system default browser
        opener::open(auth_url.as_str())?;

        // A very naive implementation of the redirect server.
        let listener = std::net::TcpListener::bind("127.0.0.1:8594").unwrap();
        for stream in listener.incoming() {
            if let Ok(stream) = stream { 

                let mut reader = std::io::BufReader::new(&stream);
                
                let mut request_line = String::new();
                reader.read_line(&mut request_line)?;

                let redirect_url = request_line.split_whitespace().nth(1).unwrap();
                let url = url::Url::parse(&format!("http://localhost{}", redirect_url)).unwrap();
                let mut pairs = url.query_pairs();

                return Ok(pairs.next().unwrap().1.to_string());
            }
        }

        Err(AccountError::AuthorisationCodeFailure) 
    }

    //exchange authorisation_code for tokens
    fn get_tokens(&mut self, authorisation_code: &str) -> Result<(), AccountError> {
        let token: Tokens = serde_json::from_slice(reqwest::blocking::Client::new()
            .post("https://login.live.com/oauth20_token.srf")
            .form(&[
                ("client_id", CLIENT_ID),
                // ("client_secret", CLIENT_SECRET),
                ("code", authorisation_code),
                ("grant_type", "authorization_code"),
                ("redirect_uri", "http://localhost:8594")
            ])
            .send()?
            .bytes()?
            .to_vec().as_slice())?;

        self.refresh_token = token.refresh_token;
        self.access_token = token.access_token;

        Ok(())
    }  

    fn get_xauth_response(&mut self) -> Result<XboxAuthResponse, AccountError> {
        //Xbl
        let xbl_body = serde_json::json!({
            "RelyingParty": XBOX_AUTH_RELYING_PARTY,
            "TokenType": XBOX_AUTH_REQUEST_TYPE,
            "Properties": {
                "AuthMethod": XBL_AUTH_PROPERTIES_METHOD,
                "SiteName": XBL_AUTH_PROPERTIES_SITE,
                "RpsTicket": format!("d={}", &self.access_token)
            }
        });

        let xbl_response = reqwest::blocking::Client::new()
            .post(XBL_AUTHENTICATION_URL)
            .json(&xbl_body)
            .send()?;
        let xbl: XboxAuthResponse = xbl_response.json()?;
        // let xbl = request::blocking::post(XBL_AUTHENTICATION_URL, serde_json::to_vec(&xbl_body)?)?;
        // let xbl: XboxAuthResponse = serde_json::from_slice(&xbl_response)?;

        //Xsts
        let (token, _) = xbl.extract_essential_information()?;
        let xsts_body = serde_json::json!({
            "RelyingParty": "rp://api.minecraftservices.com/",
            "TokenType": XBOX_AUTH_REQUEST_TYPE,
            "Properties": {
                "SandboxId": "RETAIL",
                "UserTokens": [token]
            },
        });

        let xsts_response = reqwest::blocking::Client::new()
            .post("https://xsts.auth.xboxlive.com/xsts/authorize")
            .json(&xsts_body)
            .send()?;
        let xsts: XboxAuthResponse = xsts_response.json()?;
        // let xbox_auth_response = request::blocking::post(XSTS_AUTHENTICATION_URL, serde_json::to_vec(&xbox_auth_request)?)?;
        // let xbox_auth: XboxAuthResponse = serde_json::from_slice(&xbox_auth_response)?;
        Ok(xsts)
    }

    fn get_auth_response(&mut self, xauth: XboxAuthResponse) -> Result<AuthResponse, AccountError> {
        let (token, user_hash) = xauth.extract_essential_information()?;
        let auth_body = serde_json::json!({
            "identityToken": format!("XBL3.0 x={};{}", user_hash, token)
        });

        let auth_response = reqwest::blocking::Client::new()
            .post("https://api.minecraftservices.com/authentication/login_with_xbox")
            .json(&auth_body)
            .send()?;
        let auth: AuthResponse = auth_response.json()?;

        self.access_token = auth.access_token.clone();
        Ok(auth)
        // let auth_response = request::blocking::post(LOGIN_URL, serde_json::to_vec(&body)?)?;
        // let auth: AuthResponse = serde_json::from_slice(&auth_response)?;
    }

    // fn check_for_ownership() {
         // println!("Checking for game ownership.");
        // // i don't know how to do signature verification, so we just have to assume the signatures are
        // // valid :)
        // let store: Store = client
        //     .get("https://api.minecraftservices.com/entitlements/mcstore")
        //     .bearer_auth(&access_token)
        //     .send()
        //     .await?
        //     .json()
        //     .await?;

        // anyhow::ensure!(
        //     store.items.contains(&Item::PRODUCT_MINECRAFT),
        //     "product_minecraft item doesn't exist. do you really own the game?"
        // );

        // anyhow::ensure!(
        //     store.items.contains(&Item::GAME_MINECRAFT),
        //     "game_minecraft item doesn't exist. do you really own the game?"
        // );
    // }

    // pub fn refresh_access_token(refresh_token: &str) -> Result<Tokens> {}


    fn get_user_profile(&mut self) -> Result<(), AccountError> {
        let profile: Profile = reqwest::blocking::Client::new()
            .get("https://api.minecraftservices.com/minecraft/profile")
            .bearer_auth(&self.access_token)
            .send()?
            .json()?;
        
        self.uuid = profile.id;
        self.name = profile.name;

        Ok(())
    }


}

#[derive(Debug, Deserialize)]
pub struct XboxAuthResponse {
    #[serde(rename = "Token")]
    pub token: String,
    #[serde(rename = "DisplayClaims")]
    pub user_hashes: DisplayClaims,
}

#[derive(Debug, Deserialize)]
pub struct Xui {
    #[serde(rename = "uhs")]
    pub user_hash: String,
}

#[derive(Debug, Deserialize)]
pub struct DisplayClaims {
    pub xui: Vec<Xui>,
}

impl XboxAuthResponse {
    pub fn extract_essential_information(self) -> Result<(String, String), AccountError> {
        let token = self.token;
        let user_hash = self.user_hashes.xui
            .into_iter()
            .next()
            .ok_or(AccountError::CannotFindXUI)?
            // .context("no xui found")?
            .user_hash;

        Ok((token, user_hash))
    }
}


#[derive(Debug, Deserialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub token_type: String,
}

#[derive(Debug, Deserialize)]
pub struct Tokens {
    pub access_token: String,
    pub refresh_token: String
}

#[derive(Deserialize)]
pub struct Profile {
    pub id: String,
    pub name: String,
}