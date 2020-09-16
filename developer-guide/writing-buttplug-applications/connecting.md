# Connecting

Once you've created a connector, it's time to connect to a server!

As all connector setup was done via the Connector setup, this is now just down to dealing with whether the connection process actually worked or not.

- For embedded connectors in the reference implementation, connection will always succeed, as long as the client and server are from the same library version (See below for issues involving compatibility). Hopefully, that is a situation that will rarely if ever happen.
- For external connectors, connections can fail due to usual connection issues (wrong address, server not up, network not on, etc...). There is also a chance that the client and server could have a version mismatch. We'll cover this in the next section.

:::: tabs

::: tab C# id="csharp-connecting"
<<< lang=csharp @/examples/csharp/ConnectionExample/Program.cs
:::

::: tab Javascript id="js-connecting"
```js
basic connect here
```
:::

::: tab Twine id="twine-connecting"
```html
basic connect here
```
:::

::: tab Rust id="rust-connecting"
<<< lang=rust @/examples/rust/src/bin/connection.rs
:::

::::

## Client/Server Compatibility

To keep up with new hardware capabilities and the needs of users, the Buttplug protocol spec is versioned. Any time any message changes, the version number of the spec is incremented by 1. When the Connect function is called from a reference client, a "handshake" process occurs where the Client and Server trade their spec version with each other.

Maintaining backward compatibility with applications that may be abandoned (something that is rare in sex software) is a core tenant of Buttplug. Buttplug reference servers try to support *all* versions of the spec. Older clients should always be able to connect to newer servers. 

If a client is running a newer version of the spec than the server, a connection error will be thrown, because otherwise the client can send messages that the server doesn't know what to do with. In this case, it is assumed that there will be a new version of the server library or software available that the client can upgrade to.

## Buttplug Ping

The other session property negotiated on handshake is "Ping". Ping is a protocol negotiated keep-alive, with the server dictating the expected ping time to the client. A ping time of 0 denotes "no ping expected", while any number above that is the expected maximum amount of time in milliseconds between pings.

Ping exists to try ensuring some basic level of safety for usage if a client application locks up, remote connection is interrupted, or other horrible scenarios occur that stop ping messages from being transmitted. If a client does not send a ping message within the alloted time, the server is expected to disconnect and stop all devices that are currently active. 

Keeping in line with the knowledge that reference, and most likely, all implementations of Buttplug are neither real-time constrained nor safety-guaranteed, the Ping system is more of a vaguely hopeful mitigation than a secure requirement. Your milage may vary. Don't die.

In reference library implementations of the Client, ping negotiation is handled opaquely by the client API. It is assumed that if the client's event loop fails to send a ping, the program has most likely locked up or crashed, and therefore everything should be shut down. Therefore, no code samples are provided for this. It should *just work*.

On ping failure in the client APIs, you should either receive some sort of event or callback denoting the error. The event or callback arguments will contain an error with an error class type of ERROR_PING. Any subsequent calls to server commands (device search/commands, etc) will fail from this point on.

:::: tabs

::: tab C# id="csharp-ping"
<<< lang=csharp @/examples/csharp/PingTimeoutExample/Program.cs
:::

::: tab Javascript id="js-ping"
```js
ping example here
```
:::

::: tab Twine id="twine-ping"
```html
ping example here
```
:::

::: tab Rust id="rust-ping"
```rust
ping example here
```
:::

::::

## What to Expect on Successful Connect

On successful connect, any devices that were already connected to the server will be sent to the client as "DeviceAdded" messages. More information on how DeviceAdded message and events work is provided in the Device Enumeration section of this chapter, but for now, it's just worth knowing that you may get notified about this. We'll cover how to deal with it later.

After the Connect function returns without error, you are now ready to enumerate and control devices!
