use async_std::{
    io::{ReadExt, stdin},
    stream::StreamExt,
};
use buttplug::{
    client::{ButtplugClient, ButtplugClientError, ButtplugClientEvent},
    connector::ButtplugInProcessClientConnector,
    core::errors::ButtplugError,
};

async fn wait_for_input() {
    stdin().bytes().next().await;
}

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    // After you've created a connector, the connection looks the same no
    // matter what, though the errors thrown may be different.
    let connector = ButtplugInProcessClientConnector::default();

    // If you'd like to try a remote network connection, comment the
    // connector line above and uncomment the one below. Note that you'll
    // need to turn off SSL on whatever server you're using.

    // let connector: ButtplugRemoteConnector<_, ButtplugClientJSONSerializer, _, _> = ButtplugRemoteConnector::new(
    //     ButtplugWebsocketClientTransport::new_insecure_connector("ws://localhost:12345/buttplug")
    // );

    // Now we connect. If anything goes wrong here, we'll get an Err with either
    //
    // - A ButtplugClientConnectionError if there's a problem with
    //   the Connector, like the network address being wrong, server not
    //   being up, etc.
    // - A ButtplugHandshakeError if there is a client/server version
    //   mismatch.
    let client = ButtplugClient::connect("Example Client", connector)
        .await;
    let client = match client {
        Ok(client) => client,
        Err(ButtplugClientError::ButtplugConnectorError(error)) => {
            // If our connection failed, because the server wasn't turned on,
            // SSL/TLS wasn't turned off, etc, we'll just print and exit
            // here.
            println!("Can't connect, exiting! Message: {}", error);
            wait_for_input().await;
            return Ok(());
        }
        Err(ButtplugClientError::ButtplugError(error)) => match error {
            ButtplugError::ButtplugHandshakeError(error) => {
                // This means our client is newer than our server, and we need to
                // upgrade the server we're connecting to.
                println!("Handshake issue, exiting! Message: {}", error);
                wait_for_input().await;
                return Ok(());
            }
            error => {
                println!("Unexpected error type! {}", error);
                wait_for_input().await;
                return Ok(());
            }
        }
    };

    let (client, events) = client;

    // We're connected, yay!
    println!("Connected! Check Server for Client Name.");

    wait_for_input().await;

    // And now we disconnect as usual
    client.disconnect().await?;

    Ok(())
}
