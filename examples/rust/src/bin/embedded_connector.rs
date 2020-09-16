use buttplug::{
    client::ButtplugClient,
    connector::ButtplugInProcessClientConnector,
    server::comm_managers::btleplug::BtlePlugCommunicationManager,
};

async fn main_the_hard_way() {
    // First off, we'll set up our Embedded Connector.
    let connector = ButtplugInProcessClientConnector::new("Example Server", 0);

    // This is how we add Bluetooth manually. (We could also do this with any other communication manager.)
    connector.server_ref()
        .add_comm_manager::<BtlePlugCommunicationManager>()
        .expect("couldn't set up bluetooth communications");

    let _client = ButtplugClient::connect("Example Client", connector)
        .await
        .expect("couldn't connect to internal client");
}

#[async_std::main]
async fn main() {
    // This is the easy way, it sets up an embedded server with everything set up automatically
    let _client = ButtplugClient::connect_in_process("Example Client", 0)
        .await
        .expect("couldn't create internal client");
}
