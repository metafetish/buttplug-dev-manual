use buttplug::{
    client::ButtplugClient,
    connector::ButtplugInProcessClientConnector,
    server::{comm_managers::btleplug::BtlePlugCommunicationManager, ButtplugServerOptions},
};

async fn main_the_hard_way() -> anyhow::Result<()> {
    // First off, we'll set up our Embedded Connector.
    let connector = ButtplugInProcessClientConnector::default();
    // This is how we add Bluetooth manually. (We could also do this with any other communication manager.)
    connector.server_ref()
        .add_comm_manager::<BtlePlugCommunicationManager>()?;

    let _client = 
        ButtplugClient::connect("Example Client", connector)
        .await?;

    Ok(())
}

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    // This is the easy way, it sets up an embedded server with everything set up automatically
    let _client = 
        ButtplugClient::connect_in_process("Example Client", &ButtplugServerOptions::default())
        .await?;

    Ok(())
}
