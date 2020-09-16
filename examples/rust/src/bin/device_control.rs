use async_std::{
    io::{ReadExt, stdin},
    stream::StreamExt,
};
use buttplug::{
    client::{ButtplugClient, ButtplugClientError, ButtplugClientEvent, device::VibrateCommand},
    connector::ButtplugInProcessClientConnector,
    core::messages::ButtplugDeviceMessageType,
    test::new_bluetoothle_test_device,
};

async fn wait_for_input() {
    stdin().bytes().next().await;
}

#[async_std::main]
async fn main() {
    let connector = ButtplugInProcessClientConnector::new("Example Server", 0);
    let server = connector.server_ref();
    server.add_test_comm_manager().expect("Couldn't init test device");
    let (client, events) = ButtplugClient::connect("Example Client", connector)
        .await
        .expect("couldn't connect to embedded server");

    // For this example, we'll use the Test device. This is
    // included in Buttplug Rust >= v0.1.0. It basically emulates how a
    // regular device manager would work, exposing access to a Test
    // Device which can take VibrateCmd messages. This basically exists
    // as a way to test connection and setup UI without having to use
    // actual hardware.
    let (test_device, test_device_internal) = new_bluetoothle_test_device("Test Device")
        .await
        .expect("Couldn't create test device");

    println!("Connected!");

    // You usually shouldn't run Start/Stop scanning back-to-back like
    // this, but with TestDevice we know our device will be found when we
    // call StartScanning, so we can get away with it.
    client.start_scanning().await.expect("couldn't start scanning");
    client.stop_scanning().await.expect("couldn't stop scanning");
    println!("Client currently knows about these devices:");
    for device in client.devices() {
        println!("- {}", device.name);
    }
    wait_for_input().await;

    for device in client.devices() {
        println!("{} supports these messages:", device.name);
        for (r#type, attributes) in device.allowed_messages {
            println!("- {}", r#type);
            if let Some(count) = attributes.feature_count {
                println!(" - Features: {}", count);
            }
        }
    }

    println!("Sending commands");

    // Now that we know the message types for our connected device, we
    // can send a message over! Seeing as we want to stick with the
    // modern generic messages, we'll go with VibrateCmd.
    //
    // There's a couple of ways to send this message.
    let test_client_device = &client.devices()[0];

    // We can use the convenience functions on ButtplugClientDevice to
    // send the message. This version sets all of the motors on a
    // vibrating device to the same speed.
    test_client_device.vibrate(VibrateCommand::Speed(1.0))
        .await
        .expect("couldn't set vibrate speed");

    // If we wanted to just set one motor on and the other off, we could
    // try this version that uses an array. It'll throw an exception if
    // the array isn't the same size as the number of motors available as
    // denoted by FeatureCount, though.
    //
    // You can get the vibrator count using the following code, though we
    // know it's 2 so we don't really have to use it.
    let vibrator_count = test_client_device.allowed_messages
        .get(&ButtplugDeviceMessageType::VibrateCmd)
        .and_then(|attributes| attributes.feature_count);
    test_client_device.vibrate(VibrateCommand::SpeedVec(vec![1.0, 0.0]))
        .await
        .expect("couldn't set vibrate speed");

    wait_for_input().await;

    // And now we disconnect as usual.
    client.disconnect().await.expect("couldn't disconnect");

    // If we try to send a command to a device after the client has
    // disconnected, we'll get an exception thrown.
    let vibrate_result = test_client_device.vibrate(VibrateCommand::Speed(1.0))
        .await;
    if let Err(ButtplugClientError::ButtplugConnectorError(error)) = vibrate_result {
        println!("Tried to send after disconnection! Error: ");
        println!("{}", error);
    }
    wait_for_input().await;
}
