# Choosing a Connector

The first thing to do when writing a Buttplug application is figuring out where the server is going to live.

- **Inside the Application (aka Embedded)**
  - You'll bundle a buttplug server with your application. Easier to distribute and create UI for, but also means if the Buttplug libraries change or upgrade, you may need to do the same. This can also tie your application to a single platform (which may or may not be an issue), if you redistribute libraries that only run on a single platform.
- **Outside the Application**
  - You'll allow the user to connect to a Buttplug server they're running on their system, outside of your application. This usually means you'll connect via some IPC method, like pipes or websockets. Buttplug Client Connector libraries usually implement the system specific transports for you, so it's just a matter of presenting the user with a UI that lets them connect as they need.
- **Both!**
  - Bundling a server with your application *and* allowing users to connect to an outside server is also an option, albeit a complicated one. Buttplug reference implementation libraries are usually distributed with Client and Server capabilities, so you can embed a server if that's simple for you, and allow the user to connect out if you don't feel like upgrading later. This may require more in-depth UI work to let the user know what's going on, though.

## Embedded Servers and Connectors

An Embedded server means both the client and server are part of the application are you building. While doing this ends up being more convenient for the user in some ways, as they have less setup to do and choices to make, there are a few drawbacks, including:

- If the libraries upgrade (which is how we usually deal with new hardware/protocol support), you'll need to upgrade your app too.
- This may tie you to a certain platform, i.e. if you're using Windows libraries, your application might only run on windows. This all depends on the library you're using, though.
- You'll need to set up the server yourself in your application.

There's not really much to cover about the first two problems, they're just part of the choice you make in using this method. However, going over setting up a server is important, as this may require some extra work on your end to make sure users have access to the devices they expect.

To set up an embedded server, you'll need to have a bit of knowledge of how servers work in the reference implementations. All server reference implementations have a "Device Manager", which manage different communication busses, like usb, bluetooth, serial, and so on. These Device Managers must have "Subtype Managers" added to them (via a server API) before the server starts, so that the server can use them to find and present devices to clients.

In some implementations (this depends on language/framework capabilities), all available Subtype Managers are added by default. Sometimes you may need to add them manually.

This process is outlined in the code example below.

:::: tabs

::: tab C# id="csharp-embedded"
<<< lang=csharp @/examples/csharp/EmbeddedConnectorExample/Program.cs
:::

::: tab Javascript id="javascript-embedded"
```js
embedded example here
```
:::

::: tab Twine id="twine-embedded"
```html
embedded example here
```
:::

::: tab Rust id="rust-embedded"
<<< lang=rust @/examples/rust/src/bin/embedded_connector.rs
::::

## External Servers and Connectors

For using external servers, such as Websocket or IPC Servers, the process is much simpler. You'll need to provide the user a way to pass in the server address, then you just create the connector object using that address. Since you don't have access to the actual server object, you can assume all Device Manager setup is done by the server itself.

:::: tabs

::: tab C# id="csharp-external"
<<< lang=csharp @/examples/csharp/RemoteConnectorExample/Program.cs
:::

::: tab Javascript id="javascript-external"
```js
external example here
```
:::

::: tab Twine id="twine-external"
```html
external example here
```
:::

::::
