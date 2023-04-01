use std::net::Ipv4Addr;

use const_format::concatcp;
use lazy_static::lazy_static;
use regex::Regex;
use serde::Deserialize;
use thiserror::Error;

use crate::Credential;
use crate::IspType;

const URL_BASE: &str = "http://10.10.244.11";
const URL_FETCH_IP: &str = concatcp!(URL_BASE, "/");
const URL_BASE_WITH_PORT: &str = concatcp!(URL_BASE, ":801");
const URL_CHECK_STATUS: &str = concatcp!(
    URL_BASE_WITH_PORT,
    "/eportal/?c=ACSetting&a=checkScanIP&wlanuserip={}"
);
const URL_LOGIN: &str = concatcp!(
    URL_BASE_WITH_PORT,
    "/eportal/?c=ACSetting&a=Login&wlanuserip={}&wlanacip=10.255.252.150"
);

lazy_static! {
    static ref RE_FETCH_IP: Regex = Regex::new("v4serip='(.*?)'").unwrap();
}

#[derive(Deserialize)]
struct CheckStatusResult {
    result: String,
    #[serde(rename = "msg")]
    message: String,
    account: Option<String>,
}

#[derive(Error, Debug)]
pub enum LoginError {
    #[error("network disconnected")]
    Disconnect(),
    #[error("http request failed: {0}")]
    HttpRequestFailed(#[from] reqwest::Error),
    #[error("authentication failed")]
    AuthenticationFailed(),
    #[error("fetch ip failed")]
    FetchIpFailed(),
    #[error("already login with another account: {0}")]
    AlreadyLogin(String),
    #[error("deserialize failed")]
    DeserializeFailed(#[from] serde_json::Error),
}

async fn fetch_ip(client: &reqwest::Client) -> Result<Option<Ipv4Addr>, LoginError> {
    let text = client
        .get(URL_FETCH_IP)
        .send()
        .await?
        .text_with_charset("GBK")
        .await?;
    match (*RE_FETCH_IP).captures(text.as_str()) {
        Some(caps) => match caps.get(0) {
            Some(mat) => Ok(mat.as_str().parse().ok()),
            None => Ok(None),
        },
        None => Ok(None),
    }
}

async fn check_status(
    client: &reqwest::Client,
    ip: &Ipv4Addr,
    account: &String,
) -> Result<bool, LoginError> {
    let text = client
        .get(URL_CHECK_STATUS.replacen("{}", ip.to_string().as_str(), 1))
        .send()
        .await?
        .text()
        .await?;
    let result: CheckStatusResult = serde_json::from_str(
        text.chars()
            .skip(2)
            .take(text.chars().count() - 3)
            .collect::<String>()
            .as_str(),
    )?;
    match result.account {
        Some(account_) if &account_ == account => Ok(true),
        Some(account_) => Err(LoginError::AlreadyLogin(account_)),
        None => Ok(false),
    }
}

fn derive_account(userid: &String, isp: IspType) -> String {
    match isp {
        IspType::EDU => format!("{}", userid),
        IspType::CMCC => format!("{}@cmcc", userid),
        IspType::CT => format!("{}@njxy", userid),
    }
}

pub async fn login(client: &reqwest::Client, config: &Credential) -> Result<(), LoginError> {
    println!("login!!");
    let ip = fetch_ip(client).await?.ok_or(LoginError::FetchIpFailed())?;
    let account = derive_account(&config.userid, config.isp);
    if check_status(client, &ip, &account).await? {
        println!("already login");
        return Ok(());
    }
    println!("logging in!!");
    client
        .post(URL_LOGIN.replacen("{}", ip.to_string().as_str(), 1))
        .form(&[("DDDDD", &account), ("upass", &config.password)])
        .send()
        .await?
        .text()
        .await?;
    println!("logging in done!!");
    if check_status(client, &ip, &account).await? {
        println!("logged in!!");
        Ok(())
    } else {
        println!("failed to log in!!");
        Err(LoginError::AuthenticationFailed())
    }
}
