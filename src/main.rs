#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let builder = iroh::Endpoint::builder().relay_mode(iroh::RelayMode::Default);
    let endpoint = builder.bind().await?;
    println!("node id: {:?}", endpoint.node_id());

    Ok(())
}
