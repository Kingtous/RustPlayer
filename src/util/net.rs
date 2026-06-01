use std::sync::mpsc::Sender;

#[tokio::main]
pub async fn download(url: &str, tx: &Sender<String>) -> anyhow::Result<()> {
    let resp = reqwest::get(url).await?.text().await?;
    tx.send(resp)?;
    Ok(())
}

#[tokio::main]
pub async fn download_as_bytes(
    url: &str,
    tx: &Sender<bytes::Bytes>,
) -> anyhow::Result<()> {
    let resp = reqwest::get(url).await?.bytes().await?;
    tx.send(resp)?;
    Ok(())
}
