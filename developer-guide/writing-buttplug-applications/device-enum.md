# Device Enumeration

Once the client and server are connected, they can start talking to each other about devices.

## Scanning

To find out about new devices, Buttplug Client libraries will usually provide 2 functions and an event/callback:

- StartScanning (Method)
   - Tells the server to start looking for devices via the Device Manager. This will start the Bluetooth Manager doing a bluetooth scan, the USB manager looking for USB or HID devices, etc... for all loaded Device Communication Managers
   - **Note:** Scanning may still require user input on the server side! For instance, using WebBluetooth in browsers with buttplug-wasm will require the user to interact with browser dialogs, so calling StartScanning() may open that dialog.
- StopScanning (Method)
   - Tells the server to stop scanning for devices if it hasn't already.
- ScanningFinished (Event/Callback)
   - When all device communication managers have finished looking for new devices, this event will be fired from the client to let applications know to update their UI (for instance, to change a button name from "Stop Scanning" to "Start Scanning"). This event may fire without StopScanning ever being called, as there are cases where scanning is not indefinite (once again, WebBluetooth is a good example, as well as things like gamepad scanners).

## Device Connection Events and Storage

There are 2 events related to device connections that the client may fire:

- DeviceAdded (Event/Callback)
    - This event will contain a new device object. It denotes that the server is now connected to this device, and that the device can take commands.
- DeviceRemoved (Event/Callback)
    - This event will fire when a device disconnects from the server for some reason. It should contain and instance of the device that disconnected.
    
While the events are handy for updating UI, Client implementations usually also hold a list of currently connected devices that can be used for iteration if needed.

Both events may be fired at any time during a Buttplug Client/Server session. DeviceAdded can be called outside of StartScanning()/StopScanning(), and even right after connect in some instances.

If you are using a remote connector (i.e. connecting an application to Intiface Desktop), you don't actually know the state of the Buttplug Server you're connecting too. The server could already be running and have devices connected to it. In this case, the Client will emit DeviceAdded events on successful connection. 

This means you will want to have your event handlers set up **BEFORE** connecting, in order to catch these messages. You can also check the Devices storage (usually a public collection on your Client instance, like an array or list) after connect to see what devices are there.

## Code Example

Here's some examples of how device enumeration works in different implementations of Buttplug.

<CodeSwitcher :languages="{rust:'Rust', csharp:'C#', js: 'Javascript'}">
<template v-slot:rust>

<<< @/examples/rust/src/bin/device_enumeration.rs

</template>
<template v-slot:csharp>

<<< @/examples/csharp/DeviceEnumerationExample/Program.cs

</template>
<template v-slot:js>

<<< @/examples/javascript/device-enumeration-example.js

</template>
</CodeSwitcher>

