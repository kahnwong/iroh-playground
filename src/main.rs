#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let builder = iroh::Endpoint::builder();
    let endpoint = builder.bind().await?;
    println!("node id: {:?}", endpoint.node_id());

    Ok(())
}
