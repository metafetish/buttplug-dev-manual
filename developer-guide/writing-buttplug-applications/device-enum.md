# Device Enumeration

Once the client and server are connected, they can start talking to each other about devices.

## Scanning

To find out about new devices, Buttplug Client libraries will usually provide 2 functions and an event/callback:

- StartScanning() (Method)
   - Tells all Device Subtype Managers to start looking for devices on their respective busses. This will start the Bluetooth Manager doing a bluetooth scan, the USB manager looking for USB or HID devices, etc...
   - **Note:** Scanning may still require user input on the server side! For instance, using WebBluetooth in browsers with buttplug-js will require the user to interact with browser dialogs, so calling StartScanning() may open that dialog.
- StopScanning() (Method)
   - Tells all Device Subtype Managers that are still scanning to stop.
- ScanningFinished (Event/Callback)
   - When all subtype managers have finished looking for new devices, this event will be fired from the client to let applications know to update their UI (for instance, to change a button name from "Stop Scanning" to "Start Scanning"). This event may fire without StopScanning ever being called, as there are cases where scanning is not indefinite (once again, WebBluetooth is a good example).

## Device Connection Events and Storage

There are 2 events related to device connections that the client may fire:

- DeviceAdded (Event/Callback)
    - This event will usually contain a new device object or description (based on how devices are implemented in the programming language you're using). It denotes that the server is now connected to this device.
- DeviceRemoved (Event/Callback)
    - This event will fire when a device disconnects from the server for some reason. It should contain the device object or description that disconnected.
    
While the events are handy for updating UI, Client implementations usually also hold a list of currently connected devices that can be used for iteration if needed.

Both events may be fired at any time during a Buttplug Client/Server session. DeviceAdded can be called outside of StartScanning()/StopScanning(), and even right after connect, so it is a good idea to set up a callback/handler for that event before connecting the client to the server.

## Code Example

Here's some examples of how device enumeration works in different implementations of Buttplug.

:::: tabs

::: tab C# id="csharp-enum"
<<< lang=csharp @/examples/csharp/DeviceEnumerationExample/Program.cs
:::

::: tab Javascript id="js-enum"
```js
external example here
```
:::

::: tab Twine id="twine-enum"
```html
external example here
```
:::

::: tab Rust id="rust-enum"
<<< lang=rust @/examples/rust/src/bin/device_enumeration.rs
:::

::::
