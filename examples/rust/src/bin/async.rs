use async_std::stream::StreamExt;
use buttplug::{
    client::{ButtplugClient, ButtplugClientEvent},
    connector::{ButtplugRemoteConnector, ButtplugWebsocketClientTransport},
    core::messages::serializer::ButtplugClientJSONSerializer,
};

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    // In Rust, anything that will block is awaited. For instance, if
    // we're going to connect to a remote server, that might take some
    // time due to the network connection quality, or other issues. To
    // deal with that, we use async/await.
    //
    // For now, you can ignore the API calls here, since we're just
    // talking about how our API works in general. Setting up a
    // connection is discussed more in the Connecting section of this document.
    let connector: ButtplugRemoteConnector<_, ButtplugClientJSONSerializer, _, _> = ButtplugRemoteConnector::new(
        ButtplugWebsocketClientTransport::new_insecure_connector("ws://localhost:12345/buttplug")
    );

    // For synchronous messages, we'll use our Connect API. Connecting to
    // a server requires the client and server to send information back
    // and forth, so we'll await that while those (possibly somewhat
    // slow, depending on if network is being used and other factors)
    // transfers happen.
    let (_client, mut event_stream) = ButtplugClient::connect("Example Client", connector)
        .await
        .expect("Can't connect to Buttplug Server, exiting!");

    // As an example of async messages, we'll assume the server might
    // send the client notifications about new devices that it has found.
    // The client will let us know about this via events.
    while let Some(event) = event_stream.next().await {
        if let ButtplugClientEvent::DeviceAdded(device) = event {
            println!("Device {} connected", device.name);
        }
    }

    Ok(())
}
