use iroh::{Endpoint, RelayMode};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let builder = Endpoint::builder()
        .relay_mode(RelayMode::Default)
        .discovery_n0();

    let endpoint = builder.bind().await?;

    println!("node id: {:?}", endpoint.node_id());
    Ok(())
}
