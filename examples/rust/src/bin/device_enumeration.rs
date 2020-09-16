use async_std::{
    io::{ReadExt, stdin},
    stream::StreamExt,
};
use buttplug::{
    client::{ButtplugClient, ButtplugClientEvent},
};

async fn wait_for_input() {
    stdin().bytes().next().await;
}

#[async_std::main]
async fn main() {
    // Usual embedded connector setup. We'll assume the server found all
    // of the subtype managers for us (the default features include all of them).
    let (client, events) = ButtplugClient::connect_in_process("Example Client", 0)
        .await
        .expect("couldn't connect to embedded server");

    // Set up our DeviceAdded/DeviceRemoved event handlers before connecting.
    let events = events.inspect(|event| match event {
        ButtplugClientEvent::DeviceAdded(device) => {
            println!("Device {} Connected!", device.name);
        }
        ButtplugClientEvent::DeviceRemoved(info) => {
            println!("Device {} Removed!", info.device_name);
        }
        _ => {}
    });

    // We're connected, yay!
    println!("Connected!");

    // Set up our scanning finished function to print whenever scanning is done.

    events.for_each(|event| {
        if let ButtplugClientEvent::ScanningFinished = event {
            println!("Device scanning is finished!");
        }
    });

    // Now we can start scanning for devices, and any time a device is
    // found, we should see the device name printed out.
    client.start_scanning().await.expect("couldn't start scanning");
    wait_for_input().await;

    // Some Subtype Managers will scan until we still them to stop, so
    // let's stop them now.
    client.stop_scanning().await.expect("couldn't stop scanning");
    wait_for_input().await;

    // Since we've scanned, the client holds information about devices it
    // knows about for us. These devices can be accessed with the Devices
    // getter on the client.
    println!("Client currently knows about these devices:");
    for device in client.devices() {
        println!("- {}", device.name);
    }
    wait_for_input().await;

    // And now we disconnect as usual.
    client.disconnect().await.expect("couldn't disconnect");
}
