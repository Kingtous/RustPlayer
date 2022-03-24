use std::fmt::{Debug, Display, Formatter, write};
use std::sync::mpsc::Sender;

pub struct DownloadTimeoutError{
    msg: String
}

impl Display for DownloadTimeoutError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
       write!(f,"Download Timeout! {}",self.msg.as_str())
    }
}

impl Debug for DownloadTimeoutError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"Download Timeout: {}",self.msg.as_str())
    }
}

impl failure::Fail for DownloadTimeoutError {

}

pub async fn download(url: &str, tx: Sender<String>) -> std::result::Result<(), failure::Error> {
    let resp = reqwest::get(url).await?.text().await?;
    tx.send(resp)?;
    Ok(())
}